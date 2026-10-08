// --- LLM-generated --- //
// Checks the PTX emitted for the tensormap replace intrinsics in `core::arch::nvptx` that
// require `sm_100a` and PTX ISA 8.7.
// nvptx-target: sm_100a +ptx87

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The tensor maps are loaded from memory so that LLVM can't infer their address spaces.
// CHECK-LABEL: .entry tensormap_elemtype_b4x16(
// CHECK: tensormap.replace.tile.elemtype.global.b1024.b32 [{{%rd?[0-9]+}}], 13;
// CHECK: tensormap.replace.tile.elemtype.global.b1024.b32 [{{%rd?[0-9]+}}], 14;
// CHECK: tensormap.replace.tile.elemtype.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 15;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_elemtype_b4x16(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_elemtype::<{ StateSpace::Global }, { TensormapElemType::B4x16 }>(*g);
        tensormap_replace_elemtype::<{ StateSpace::Global }, { TensormapElemType::B4x16P64 }>(*g);
        tensormap_replace_elemtype::<{ StateSpace::SharedCta }, { TensormapElemType::B6x16P32 }>(
            *s,
        );
    }
}

// CHECK-LABEL: .entry tensormap_swizzle_atomicity(
// CHECK: tensormap.replace.tile.swizzle_atomicity.global.b1024.b32 [{{%rd?[0-9]+}}], 0;
// CHECK: tensormap.replace.tile.swizzle_atomicity.global.b1024.b32 [{{%rd?[0-9]+}}], 1;
// CHECK: tensormap.replace.tile.swizzle_atomicity.global.b1024.b32 [{{%rd?[0-9]+}}], 2;
// CHECK: tensormap.replace.tile.swizzle_atomicity.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 3;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_swizzle_atomicity(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_swizzle_atomicity::<
            { StateSpace::Global },
            { TensormapSwizzleAtomicity::Atomicity16B },
        >(*g);
        tensormap_replace_swizzle_atomicity::<
            { StateSpace::Global },
            { TensormapSwizzleAtomicity::Atomicity32B },
        >(*g);
        tensormap_replace_swizzle_atomicity::<
            { StateSpace::Global },
            { TensormapSwizzleAtomicity::Atomicity32BFlip8B },
        >(*g);
        tensormap_replace_swizzle_atomicity::<
            { StateSpace::SharedCta },
            { TensormapSwizzleAtomicity::Atomicity64B },
        >(*s);
    }
}
