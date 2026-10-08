// --- LLM-generated --- //
// Checks the PTX emitted for the tensormap replace intrinsics in `core::arch::nvptx` that
// require `sm_103a` and PTX ISA 8.8.
// nvptx-target: sm_103a +ptx88

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The tensor maps are loaded from memory so that LLVM can't infer their address spaces.
// CHECK-LABEL: .entry tensormap_swizzle_mode_96b(
// CHECK: tensormap.replace.tile.swizzle_mode.global.b1024.b32 [{{%rd?[0-9]+}}], 4;
// CHECK: tensormap.replace.tile.swizzle_mode.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 4;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_swizzle_mode_96b(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_swizzle_mode::<
            { StateSpace::Global },
            { TensormapSwizzleMode::Swizzle96B },
        >(*g);
        tensormap_replace_swizzle_mode::<
            { StateSpace::SharedCta },
            { TensormapSwizzleMode::Swizzle96B },
        >(*s);
    }
}
