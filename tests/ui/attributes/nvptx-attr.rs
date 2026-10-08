// --- LLM-generated --- //
//@ add-minicore
//@ needs-llvm-components: nvptx
//@ compile-flags: --target=nvptx64-nvidia-cuda --crate-type=rlib
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items, abi_ptx, abi_gpu_kernel, nvptx_ext)]

extern crate minicore;
use minicore::*;

#[nvptx(
    max_ctas_per_cluster(4),
    min_ctas_per_sm(2),
    max_registers(64),
    max_threads_per_cta(256, 1, 1)
)]
extern "ptx-kernel" fn ok_ptx() {}

#[nvptx(exact_threads_per_cta(32, 4, 1), exact_cluster_dim(2, 1, 1))]
extern "gpu-kernel" fn ok_gpu() {}

#[nvptx(max_registers(32))]
//~^ ERROR `#[nvptx]` can only be applied to `extern "gpu-kernel"` or `extern "ptx-kernel"` functions
fn rust_abi() {}

#[nvptx(max_registers(32))]
//~^ ERROR `#[nvptx]` can only be applied to `extern "gpu-kernel"` or `extern "ptx-kernel"` functions
extern "C" fn c_abi() {}

#[nvptx(max_registers(32))] //~ ERROR the `nvptx` attribute cannot be used on structs
struct S;

#[nvptx] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn no_args() {}

#[nvptx()] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn empty() {}

#[nvptx(max_registers(0))] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn zero() {}

#[nvptx(max_registers("32"))] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn string() {}

#[nvptx(max_registers)] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn missing_value() {}

#[nvptx(max_registers = 32)] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn name_value() {}

#[nvptx(max_registers(32, 64))] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn two_scalars() {}

#[nvptx(max_threads_per_cta = 256)] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn scalar_dims() {}

#[nvptx(max_threads_per_cta(256, 1))] //~ ERROR expected exactly 3 dimensions, found 2
extern "ptx-kernel" fn two_dims() {}

#[nvptx(exact_cluster_dim(1, 0, 1))] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn zero_dim() {}

#[nvptx(max_registers(32), max_registers(64))] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn duplicate() {}

#[nvptx(max_blocks(32))] //~ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn unknown_key() {}

#[nvptx(max_threads_per_cta(256, 1, 1), exact_threads_per_cta(256, 1, 1))]
//~^ ERROR `max_threads_per_cta` and `exact_threads_per_cta` cannot both be specified
extern "ptx-kernel" fn conflicting_threads() {}

#[nvptx(exact_cluster_dim(2, 1, 1), max_ctas_per_cluster(2))]
//~^ ERROR `max_ctas_per_cluster` and `exact_cluster_dim` cannot both be specified
extern "ptx-kernel" fn conflicting_cluster() {}

#[nvptx(max_registers(32))]
#[nvptx(max_registers(64))] //~ ERROR multiple `nvptx` attributes
extern "ptx-kernel" fn repeated() {}
