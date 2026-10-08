#!/usr/bin/env python3
### LLM-generated ###
"""Generates `intrinsics_data/nvvm-intrinsics.json` from LLVM's TableGen sources.

Every `llvm.nvvm.*` intrinsic is listed with its signature, immediate arguments,
the target requirements of the NVPTX instruction patterns that select it, and a
classification describing whether it is in scope for `core::arch::nvptx`.

Usage:
    gen-nvvm-intrinsics.py <llvm-tblgen> <llvm-project/llvm> <output.json>
"""

import json
import os
import re
import subprocess
import sys
import tempfile

ADDRSPACE_PTR_TYS = {
    "llvm_global_ptr_ty": "ptr addrspace(1)",
    "llvm_shared_ptr_ty": "ptr addrspace(3)",
    "llvm_local_ptr_ty": "ptr addrspace(5)",
    "llvm_tmem_ptr_ty": "ptr addrspace(6)",
    "llvm_shared_cluster_ptr_ty": "ptr addrspace(7)",
}

# Address spaces that generic pointers can't be converted to, so intrinsics taking or returning
# pointers in them can't be exposed. Pointers in other address spaces are autocast from generic
# pointers by rustc.
BLOCKED_ADDRSPACE_TYS = ["ptr addrspace(6)"]

# Classification rules, checked in order before the address-space check.
# Each entry is (class, list of name prefixes after `llvm.nvvm.`). A trailing `$` requires
# an exact match.
CLASS_RULES = [
    (
        "excluded-texsurf",
        ["tex.", "tld4.", "suld.", "sust.", "txq.", "suq.", "istypep."],
    ),
    (
        "excluded-matrix",
        ["wmma.", "mma.", "ldmatrix.", "stmatrix.", "movmatrix.", "wgmma.", "tcgen05."],
    ),
    # Pre-Volta warp intrinsics without `.sync`, unavailable for sm_70+ with PTX ISA 6.4+.
    (
        "deprecated",
        [
            "shfl.idx.",
            "shfl.up.",
            "shfl.down.",
            "shfl.bfly.",
            "vote.all$",
            "vote.any$",
            "vote.uni$",
            "vote.ballot$",
        ],
    ),
    # Declared, but the NVPTX backend cannot select them.
    (
        "unsupported",
        [
            "read.ptx.sreg.pm4",
            "fmin.ftz.bf16",
            "fmin.ftz.nan.bf16",
            "fmin.ftz.nan.xorsign.abs.bf16",
            "fmin.ftz.xorsign.abs.bf16",
            "fmax.ftz.bf16",
            "fmax.ftz.nan.bf16",
            "fmax.ftz.nan.xorsign.abs.bf16",
            "fmax.ftz.xorsign.abs.bf16",
            # Emits `lg2.approx.f64`, which doesn't exist in PTX.
            "lg2.approx.d",
            # No lowering at all.
            "bf2h.",
            # ptxas 13.4 fails with an internal compiler error on any `.s2f6x2` destination.
            "ff.to.s2f6x2.",
            "bf16x2.to.s2f6x2.",
        ],
    ),
    # Used internally by NVVM / the backend; not meaningful as user-facing API.
    (
        "internal",
        [
            "move.",
            "compiler.",
            "reflect",
            "texsurf.handle",
            "lohi.i2d",
            "d2i.hi",
            "d2i.lo",
            # Lowers to `sqrt.rn.f32` or `sqrt.approx.f32` depending on compiler options.
            "sqrt.f$",
            "read.ptx.sreg.tid.w",
            "read.ptx.sreg.ntid.w",
            "read.ptx.sreg.ctaid.w",
            "read.ptx.sreg.nctaid.w",
            "read.ptx.sreg.clusterid.w",
            "read.ptx.sreg.nclusterid.w",
            "read.ptx.sreg.cluster.ctaid.w",
            "read.ptx.sreg.cluster.nctaid.w",
        ],
    ),
]

# Requirements of `NVPTXSubtarget` predicate methods, expressed with rustc
# target feature names. `|` separates alternatives, `&` joins conditions.
# An empty string means the predicate is satisfied by every supported target.
SUBTARGET_PREDICATES = {
    "hasAtomAddF64": "",
    "hasAtomScope": "",
    "hasAtomBitwise64": "",
    "hasAtomMinMax64": "",
    "hasLDG": "",
    "hasHWROT32": "",
    "hasFP16Math": "",
    "allowFP16Math": "",
    "hasDotInstructions": "",
    "hasAtomCas16": "ptx63",
    "hasAtomSwap128": "sm_90 & ptx83",
    "hasClusters": "sm_90 & ptx78",
    "hasBF16Math": "sm_80",
    "hasMaskOperator": "ptx71",
    "hasNoReturn": "ptx64",
    "hasMemoryOrdering": "ptx60",
    "hasSplitAcquireAndReleaseFences": "sm_90 & ptx86",
    "hasRelaxedMMIO": "ptx82",
    "hasF32x2Instructions": "sm_100 & ptx86",
    "hasCvtaParam": "ptx77",
    "hasFP8ConversionSupport": "sm_89 & ptx81 | sm_90 & ptx78",
}

# Requirements of intrinsics selected by custom C++ lowering, keyed by name prefix after
# `llvm.nvvm.`.
CUSTOM_REQUIREMENTS = {
    # `hasConvertWithStochasticRounding`
    "f32x4.to.": ["sm_100a & ptx87", "sm_103a & ptx87"],
    # Matched through a `PatFrag`.
    "prefetch.tensormap": ["sm_90 & ptx80"],
    # Selected in `NVPTXDAGToDAGISel::SelectCpAsyncBulkTensorReduceCommon`.
    "cp.async.bulk.tensor.reduce.": ["sm_90 & ptx80"],
    # Base requirements; some immediate values require newer targets (checked during lowering).
    "tensormap.replace.": [
        "sm_90a & ptx83 | sm_100a & ptx83 | sm_101a & ptx83 | sm_120a & ptx83"
        " | sm_90f & ptx88 | sm_100f & ptx88 | sm_101f & ptx88 | sm_120f & ptx88"
        " | sm_90f & ptx90 | sm_100f & ptx90 | sm_110f & ptx90 | sm_120f & ptx90"
    ],
    "tensormap.replace.swizzle.atomicity": [
        "sm_100a & ptx87 | sm_101a & ptx87 | sm_120a & ptx87"
        " | sm_100f & ptx88 | sm_101f & ptx88 | sm_120f & ptx88"
        " | sm_100f & ptx90 | sm_110f & ptx90 | sm_120f & ptx90"
    ],
}

# Predicates that describe codegen options rather than target requirements.
IGNORED_PREDICATES = [
    r"useF32FTZ\(\)",
    r"doMADWideOpt\(\)",
    r"doRsqrtOpt\(\)",
    r"TM\.getOptLevel\(\)",
    r"hasPTXASUnreachableBug\(\)",
]


def tblgen(tblgen_bin, llvm_dir, td, includes):
    with tempfile.TemporaryDirectory() as tmp:
        out = os.path.join(tmp, "out.json")
        args = [tblgen_bin, "--dump-json", "-o", out]
        for inc in includes:
            args += ["-I", os.path.join(llvm_dir, inc)]
        args.append(os.path.join(llvm_dir, td))
        subprocess.run(args, check=True)
        with open(out) as f:
            return json.load(f)


def parse_family_accel(subtarget_h):
    """Translates `hasPTXWith{Family,Accel}SMs` based predicates in NVPTXSubtarget.h."""
    preds = {}
    for m in re.finditer(r"bool (has\w+)\(\) const \{((?:(?!\bbool\b).)*?)\n  \}", subtarget_h, re.S):
        name, body = m.group(1), m.group(2)
        calls = re.findall(r"hasPTXWith(Family|Accel)SMs\((\d+),\s*\{([\d,\s]+)\}\)", body)
        if not calls or "if" in body:
            continue
        alts = []
        for kind, ptx, sms in calls:
            suffix = "f" if kind == "Family" else "a"
            for sm in sms.split(","):
                alts.append(f"sm_{sm.strip()}{suffix} & ptx{ptx}")
        preds[name] = " | ".join(alts)
    return preds


def normalize_type(intr, t):
    d = t["def"]
    if d in ADDRSPACE_PTR_TYS:
        return ADDRSPACE_PTR_TYS[d]
    if d.startswith("anonymous_"):
        rec = intr[d]
        if "LLVMMatchType" in rec["!superclasses"]:
            return f"match{rec['OverloadIndex']}"
        return rec["VT"]["def"]
    m = re.fullmatch(r"llvm_(\w+)_ty", d)
    name = m.group(1) if m else d
    return {"float": "f32", "double": "f64", "half": "f16", "bfloat": "bf16"}.get(name, name)


def immargs(intr, rec):
    out = []
    for p in rec["IntrProperties"]:
        prop = intr[p["def"]]
        if "ImmArg" in prop["!superclasses"]:
            out.append(prop["ArgNo"] - 1)
    return sorted(out)


def classify(name, tys):
    short = name[len("llvm.nvvm.") :]
    for cls, prefixes in CLASS_RULES:
        if any(short == p[:-1] if p.endswith("$") else short.startswith(p) for p in prefixes):
            return cls
    if any(t in BLOCKED_ADDRSPACE_TYS for t in tys):
        return "blocked-addrspace"
    return "in-scope"


def walk_operators(node, out):
    if isinstance(node, dict):
        if node.get("kind") == "dag":
            op = node["operator"]
            if isinstance(op, dict) and op.get("def", "").startswith("int_nvvm_"):
                out.add(op["def"])
            for arg, _ in node["args"]:
                walk_operators(arg, out)
    elif isinstance(node, list):
        for n in node:
            walk_operators(n, out)


def translate_predicate(cond, family_accel):
    for ign in IGNORED_PREDICATES:
        if re.search(ign, cond):
            return None
    if cond.startswith("!("):
        inner = cond[2:-1]
        inner = re.sub(r"Subtarget->getSmVersion\(\) >= (\d+)", r"sm_\1", inner)
        inner = re.sub(r"Subtarget->getPTXVersion\(\) >= (\d+)", r"ptx\1", inner)
        return "!(" + re.sub(r"\s*&&\s*", " & ", inner) + ")"
    m = re.fullmatch(r"Subtarget->getSmVersion\(\) >= (\d+)", cond)
    if m:
        sm = int(m.group(1))
        return f"sm_{sm}" if sm > 70 else ""
    m = re.fullmatch(r"Subtarget->getPTXVersion\(\) >= (\d+)", cond)
    if m:
        ptx = int(m.group(1))
        return f"ptx{ptx}" if ptx > 60 else ""
    m = re.fullmatch(r"Subtarget->getSmVersion\(\) == (\d+) && Subtarget->hasArchAccelFeatures\(\)", cond)
    if m:
        return f"sm_{m.group(1)}a & ptx80"
    m = re.fullmatch(r"Subtarget->(\w+)\(\)", cond)
    if m:
        pred = m.group(1)
        if pred in SUBTARGET_PREDICATES:
            return SUBTARGET_PREDICATES[pred]
        if pred in family_accel:
            return family_accel[pred]
    return f"<{cond}>"


def combine(conds):
    """Combines a conjunction of translated predicates into a single expression."""
    conds = sorted(set(c for c in conds if c), key=lambda c: (c.startswith("ptx"), c))
    if not conds:
        return ""
    # Group disjunctions so that `&` binds correctly.
    return " & ".join(f"({c})" if "|" in c and len(conds) > 1 else c for c in conds)


def requirements(target, family_accel):
    reqs = {}
    io = target["!instanceof"]
    for name in io["Instruction"] + io["Pattern"]:
        rec = target[name]
        ops = set()
        walk_operators(rec.get("Pattern"), ops)
        walk_operators(rec.get("PatternToMatch"), ops)
        if not ops:
            continue
        conds = []
        for p in rec.get("Predicates", []):
            t = translate_predicate(target[p["def"]]["CondString"], family_accel)
            if t is not None:
                conds.append(t)
        expr = combine(conds)
        for op in ops:
            reqs.setdefault(op, set()).add(expr)
    return reqs


def main():
    tblgen_bin, llvm_dir, output = sys.argv[1:4]
    intr = tblgen(tblgen_bin, llvm_dir, "include/llvm/IR/Intrinsics.td", ["include"])
    target = tblgen(
        tblgen_bin,
        llvm_dir,
        "lib/Target/NVPTX/NVPTX.td",
        ["include", "lib/Target/NVPTX"],
    )
    with open(os.path.join(llvm_dir, "lib/Target/NVPTX/NVPTXSubtarget.h")) as f:
        family_accel = parse_family_accel(f.read())
    reqs = requirements(target, family_accel)

    entries = []
    for rec_name in intr["!instanceof"]["Intrinsic"]:
        if not rec_name.startswith("int_nvvm_"):
            continue
        rec = intr[rec_name]
        name = rec["LLVMName"] or "llvm." + rec_name[len("int_") :].replace("_", ".")
        ret = [normalize_type(intr, t) for t in rec["RetTypes"]]
        params = [normalize_type(intr, t) for t in rec["ParamTypes"]]
        alts = sorted(reqs.get(rec_name, []))
        short = name[len("llvm.nvvm.") :]
        for prefix, custom in CUSTOM_REQUIREMENTS.items():
            if short.startswith(prefix):
                alts = custom
        entries.append(
            {
                "name": name,
                "class": classify(name, ret + params),
                "ret": ret,
                "params": params,
                "immargs": immargs(intr, rec),
                "convergent": any(
                    p["def"] == "IntrConvergent" for p in rec["IntrProperties"]
                ),
                # Alternative requirement expressions, one per selecting
                # pattern. Empty list: selected by custom C++ lowering.
                "requires": alts,
            }
        )
    entries.sort(key=lambda e: e["name"])

    with open(output, "w") as f:
        f.write("[\n")
        f.write(",\n".join(json.dumps(e) for e in entries))
        f.write("\n]\n")


if __name__ == "__main__":
    main()
