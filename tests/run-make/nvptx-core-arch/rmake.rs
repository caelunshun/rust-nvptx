// --- LLM-generated --- //
// Checks the PTX emitted for the intrinsics in `core::arch::nvptx`.
//
// Every other `.rs` file in this directory is a test crate. Each one contains a line of the
// form `// nvptx-target: <cpu> <target features>` selecting the configuration it is compiled
// with, and FileCheck directives checked against the emitted PTX.
//
// `core` is built from source for the nvptx64 target once per configuration, since the target
// requires all crates to be built for the same CPU.

//@ needs-llvm-components: nvptx
//@ ignore-backends: gcc

use std::collections::BTreeMap;

use run_make_support::{cwd, llvm_filecheck, rfs, rustc, source_root};

const DIRECTIVE: &str = "// nvptx-target:";

fn main() {
    let mut configs: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for entry in rfs::read_dir(cwd()) {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        if !name.ends_with(".rs") || name == "rmake.rs" {
            continue;
        }
        let src = rfs::read_to_string(&path);
        let directive = src
            .lines()
            .find_map(|l| l.strip_prefix(DIRECTIVE))
            .unwrap_or_else(|| panic!("{name}: missing `{DIRECTIVE}` line"));
        let mut parts = directive.split_whitespace();
        let cpu = parts.next().unwrap_or_else(|| panic!("{name}: missing target CPU")).to_owned();
        let features = parts.collect::<Vec<_>>().join(",");
        configs.entry((cpu, features)).or_default().push(name);
    }

    for ((cpu, features), tests) in &configs {
        let core_dir = format!("core-{cpu}-{}", features.replace(['+', ','], ""));
        let mut core = rustc();
        core.edition("2024")
            .crate_type("rlib")
            .crate_name("core")
            .target("nvptx64-nvidia-cuda")
            .target_cpu(cpu)
            .input(source_root().join("library/core/src/lib.rs"))
            .out_dir(&core_dir);
        if !features.is_empty() {
            core.arg(format!("-Ctarget-feature={features}"));
        }
        core.run();

        for test in tests {
            let asm = format!("{}.s", test.trim_end_matches(".rs"));
            let mut cmd = rustc();
            cmd.edition("2024")
                .crate_type("rlib")
                .target("nvptx64-nvidia-cuda")
                .target_cpu(cpu)
                .opt_level("3")
                .extern_("core", format!("{core_dir}/libcore.rlib"))
                .emit(format!("asm={asm}"))
                .input(test);
            if !features.is_empty() {
                cmd.arg(format!("-Ctarget-feature={features}"));
            }
            cmd.run();
            llvm_filecheck().patterns(test).input_file(&asm).run();
        }
    }
}
