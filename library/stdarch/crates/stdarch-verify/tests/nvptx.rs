// --- LLM-generated --- //
//! Verification of NVPTX intrinsics against `intrinsics_data/nvvm-intrinsics.json`.
//!
//! Every `llvm.nvvm.*` declaration in `core_arch/src/nvptx` must name an
//! in-scope LLVM intrinsic and use a signature compatible with it. Run with
//! `--nocapture` to print the list of in-scope intrinsics not exposed yet.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use syn::parse::Parse;
use syn::punctuated::Punctuated;

#[derive(Deserialize)]
struct Intrinsic {
    name: String,
    class: String,
    ret: Vec<String>,
    params: Vec<String>,
}

struct Decl {
    link_name: String,
    rust_name: String,
    file: PathBuf,
    ret: Vec<String>,
    never_returns: bool,
    params: Vec<String>,
}

fn nvptx_sources() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../core_arch/src/nvptx");
    let mut files = Vec::new();
    let mut stack = vec![dir];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Maps a Rust type to the LLVM types it is lowered to.
fn llvm_types(ty: &syn::Type) -> Result<Vec<String>, String> {
    Ok(match ty {
        syn::Type::Tuple(t) => t
            .elems
            .iter()
            .map(llvm_types)
            .collect::<Result<Vec<_>, _>>()?
            .concat(),
        syn::Type::Ptr(_) => vec!["ptr".to_string()],
        syn::Type::Paren(p) => llvm_types(&p.elem)?,
        syn::Type::Path(p) => {
            let ident = p.path.segments.last().unwrap().ident.to_string();
            let llvm = match ident.as_str() {
                "bool" => "i1",
                "i8" | "u8" => "i8",
                "i16" | "u16" | "bf16" => "i16",
                "i32" | "u32" => "i32",
                "i64" | "u64" | "isize" | "usize" => "i64",
                "i128" | "u128" => "i128",
                "f16" | "f32" | "f64" => &ident,
                "f16x2" => "v2f16",
                "bf16x2" | "i16x2" | "u16x2" => "v2i16",
                "i8x4" | "u8x4" => "v4i8",
                "f32x2" => "v2f32",
                "f32x4" => "v4f32",
                _ => return Err(format!("unknown type `{ident}`")),
            };
            vec![llvm.to_string()]
        }
        _ => return Err(format!("unsupported type `{}`", quote::quote!(#ty))),
    })
}

/// Parses the argument and return address spaces of `#[rustc_llvm_ptr_addrspace(args(..), ret(..))]`,
/// possibly nested in `#[cfg_attr(..)]`.
fn ptr_addrspaces(attrs: &[syn::Attribute]) -> (Vec<u32>, Option<u32>) {
    let mut args = Vec::new();
    let mut ret = None;
    for attr in attrs {
        let lists = if attr.path().is_ident("cfg_attr") {
            attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .unwrap()
                .into_iter()
                .skip(1)
                .collect()
        } else {
            vec![attr.meta.clone()]
        };
        for meta in lists {
            let syn::Meta::List(list) = meta else {
                continue;
            };
            if !list.path.is_ident("rustc_llvm_ptr_addrspace") {
                continue;
            }
            list.parse_nested_meta(|meta| {
                let content;
                syn::parenthesized!(content in meta.input);
                let values = content
                    .parse_terminated(syn::LitInt::parse, syn::Token![,])?
                    .iter()
                    .map(|lit| lit.base10_parse::<u32>())
                    .collect::<syn::Result<Vec<_>>>()?;
                if meta.path.is_ident("args") {
                    args = values;
                } else if meta.path.is_ident("ret") {
                    ret = values.first().copied();
                } else {
                    return Err(meta.error("unknown key"));
                }
                Ok(())
            })
            .unwrap();
        }
    }
    (args, ret)
}

fn with_addrspace(tys: Vec<String>, addrspace: Option<&u32>) -> Vec<String> {
    match addrspace {
        Some(&n) if n != 0 => tys
            .into_iter()
            .map(|t| {
                if t == "ptr" {
                    format!("ptr addrspace({n})")
                } else {
                    t
                }
            })
            .collect(),
        _ => tys,
    }
}

fn parse_decls(file: &Path) -> Vec<Decl> {
    let src = std::fs::read_to_string(file).unwrap();
    let ast = syn::parse_file(&src).unwrap();
    let mut decls = Vec::new();
    for item in &ast.items {
        let syn::Item::ForeignMod(m) = item else {
            continue;
        };
        if m.abi.name.as_ref().map(|n| n.value()).as_deref() != Some("llvm-intrinsic") {
            continue;
        }
        for item in &m.items {
            let syn::ForeignItem::Fn(f) = item else {
                continue;
            };
            let link_name = f.attrs.iter().find_map(|a| {
                let syn::Meta::NameValue(nv) = &a.meta else {
                    return None;
                };
                if !nv.path.is_ident("link_name") {
                    return None;
                }
                let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                else {
                    return None;
                };
                Some(s.value())
            });
            let Some(link_name) = link_name else { continue };
            if !link_name.starts_with("llvm.nvvm.") {
                continue;
            }
            let rust_name = f.sig.ident.to_string();
            let err = |e: String| -> ! { panic!("{}: `{rust_name}`: {e}", file.display()) };
            let (arg_addrspaces, ret_addrspace) = ptr_addrspaces(&f.attrs);
            let params = f
                .sig
                .inputs
                .iter()
                .enumerate()
                .map(|(i, arg)| match arg {
                    syn::FnArg::Typed(t) => {
                        llvm_types(&t.ty).map(|tys| with_addrspace(tys, arg_addrspaces.get(i)))
                    }
                    syn::FnArg::Receiver(_) => unreachable!(),
                })
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|e| err(e))
                .concat();
            let (ret, never_returns) = match &f.sig.output {
                syn::ReturnType::Default => (vec![], false),
                syn::ReturnType::Type(_, ty) if matches!(**ty, syn::Type::Never(_)) => {
                    (vec![], true)
                }
                syn::ReturnType::Type(_, ty) => (
                    with_addrspace(
                        llvm_types(ty).unwrap_or_else(|e| err(e)),
                        ret_addrspace.as_ref(),
                    ),
                    false,
                ),
            };
            decls.push(Decl {
                link_name,
                rust_name,
                file: file.to_owned(),
                ret,
                never_returns,
                params,
            });
        }
    }
    decls
}

/// Finds the intrinsic for a (possibly overload-mangled) name.
fn lookup<'a>(intrinsics: &'a HashMap<&str, &Intrinsic>, name: &str) -> Option<&'a Intrinsic> {
    let mut name = name;
    loop {
        if let Some(i) = intrinsics.get(name) {
            return Some(i);
        }
        name = &name[..name.rfind('.')?];
    }
}

fn compatible(rust: &str, llvm: &str, overloads: &mut Vec<String>) -> bool {
    let bound = |overloads: &mut Vec<String>| overloads.push(rust.to_string());
    match llvm {
        "anyint" if rust.starts_with('i') && !rust.contains('x') => bound(overloads),
        "anyfloat" if matches!(rust, "f16" | "f32" | "f64" | "v2f16") => bound(overloads),
        "anyptr" if rust.starts_with("ptr") => bound(overloads),
        // Generic pointers are autocast to pointers in other address spaces.
        _ if llvm.starts_with("ptr addrspace(") => return rust == "ptr" || rust == llvm,
        "any" => bound(overloads),
        _ if llvm.starts_with("match") => {
            let idx: usize = llvm["match".len()..].parse().unwrap();
            return overloads.get(idx).is_some_and(|t| t == rust);
        }
        "bf16" => return rust == "i16",
        "v2bf16" => return rust == "v2i16",
        _ => return rust == llvm,
    }
    true
}

fn check_signature(decl: &Decl, intr: &Intrinsic) -> Result<(), String> {
    if intr.ret.is_empty() != decl.ret.is_empty() || decl.ret.len() != intr.ret.len() {
        return Err(format!(
            "return type mismatch: rust {:?}, llvm {:?}",
            decl.ret, intr.ret
        ));
    }
    if decl.params.len() != intr.params.len() {
        return Err(format!(
            "parameter mismatch: rust {:?}, llvm {:?}",
            decl.params, intr.params
        ));
    }
    let mut overloads = Vec::new();
    let pairs = decl
        .ret
        .iter()
        .zip(&intr.ret)
        .chain(decl.params.iter().zip(&intr.params));
    for (rust, llvm) in pairs {
        if !compatible(rust, llvm, &mut overloads) {
            return Err(format!(
                "type mismatch: rust ({:?}) -> {:?}, llvm ({:?}) -> {:?}",
                decl.params, decl.ret, intr.params, intr.ret
            ));
        }
    }
    Ok(())
}

#[test]
fn verify_all_signatures() {
    let json = include_bytes!("../../../intrinsics_data/nvvm-intrinsics.json");
    let data: Vec<Intrinsic> = serde_json::from_slice(json).unwrap();
    let intrinsics: HashMap<&str, &Intrinsic> = data.iter().map(|i| (i.name.as_str(), i)).collect();

    let mut errors = Vec::new();
    let mut exposed = BTreeSet::new();
    for file in nvptx_sources() {
        for decl in parse_decls(&file) {
            let loc = format!(
                "{}: `{}` ({})",
                decl.file.display(),
                decl.rust_name,
                decl.link_name
            );
            let Some(intr) = lookup(&intrinsics, &decl.link_name) else {
                errors.push(format!("{loc}: unknown intrinsic"));
                continue;
            };
            exposed.insert(intr.name.as_str());
            if intr.class != "in-scope" {
                errors.push(format!(
                    "{loc}: intrinsic is classified as `{}`",
                    intr.class
                ));
            }
            if decl.never_returns && !intr.ret.is_empty() {
                errors.push(format!(
                    "{loc}: declared as never returning but returns a value"
                ));
            }
            if let Err(e) = check_signature(&decl, intr) {
                errors.push(format!("{loc}: {e}"));
            }
        }
    }

    let mut missing: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for intr in &data {
        if intr.class == "in-scope" && !exposed.contains(intr.name.as_str()) {
            let group = intr.name["llvm.nvvm.".len()..].split('.').next().unwrap();
            missing.entry(group).or_default().push(&intr.name);
        }
    }
    let in_scope = data.iter().filter(|i| i.class == "in-scope").count();
    println!(
        "exposed {}/{} in-scope NVVM intrinsics",
        exposed.len(),
        in_scope
    );
    for (group, names) in &missing {
        println!("missing `{group}` ({}): {}", names.len(), names.join(", "));
    }

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("{e}");
        }
        panic!("found {} invalid NVVM intrinsic declarations", errors.len());
    }
}
