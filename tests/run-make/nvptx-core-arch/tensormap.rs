// --- LLM-generated --- //
// Checks the PTX emitted for the tensormap replace intrinsics in `core::arch::nvptx` that
// require `sm_90a` and PTX ISA 8.3.
// nvptx-target: sm_90a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The tensor maps are loaded from memory so that LLVM can't infer their address spaces.
// CHECK-LABEL: .entry tensormap_box_dim(
// CHECK: tensormap.replace.tile.box_dim.global.b1024.b32 [{{%rd?[0-9]+}}], 1, {{%r[0-9]+}};
// CHECK: tensormap.replace.tile.box_dim.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 4, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_box_dim(g: *const *mut c_void, s: *const *mut c_void) {
    unsafe {
        tensormap_replace_box_dim::<{ StateSpace::Global }, 1>(*g, 32);
        tensormap_replace_box_dim::<{ StateSpace::SharedCta }, 4>(*s, 64);
    }
}

// CHECK-LABEL: .entry tensormap_element_stride(
// CHECK: tensormap.replace.tile.element_stride.global.b1024.b32 [{{%rd?[0-9]+}}], 1, {{%r[0-9]+}};
// CHECK: tensormap.replace.tile.element_stride.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 3, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_element_stride(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_element_stride::<{ StateSpace::Global }, 1>(*g, 2);
        tensormap_replace_element_stride::<{ StateSpace::SharedCta }, 3>(*s, 4);
    }
}

// CHECK-LABEL: .entry tensormap_elemtype(
// CHECK: tensormap.replace.tile.elemtype.global.b1024.b32 [{{%rd?[0-9]+}}], 0;
// CHECK: tensormap.replace.tile.elemtype.global.b1024.b32 [{{%rd?[0-9]+}}], 8;
// CHECK: tensormap.replace.tile.elemtype.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 12;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_elemtype(g: *const *mut c_void, s: *const *mut c_void) {
    unsafe {
        tensormap_replace_elemtype::<{ StateSpace::Global }, { TensormapElemType::U8 }>(*g);
        tensormap_replace_elemtype::<{ StateSpace::Global }, { TensormapElemType::F32Ftz }>(*g);
        tensormap_replace_elemtype::<{ StateSpace::SharedCta }, { TensormapElemType::Tf32Ftz }>(*s);
    }
}

// CHECK-LABEL: .entry tensormap_fill_mode(
// CHECK: tensormap.replace.tile.fill_mode.global.b1024.b32 [{{%rd?[0-9]+}}], 0;
// CHECK: tensormap.replace.tile.fill_mode.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 1;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_fill_mode(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_fill_mode::<{ StateSpace::Global }, { TensormapFillMode::ZeroFill }>(*g);
        tensormap_replace_fill_mode::<{ StateSpace::SharedCta }, { TensormapFillMode::OobNanFill }>(
            *s,
        );
    }
}

// CHECK-LABEL: .entry tensormap_global_address(
// CHECK: tensormap.replace.tile.global_address.global.b1024.b64 [{{%rd?[0-9]+}}], {{%rd[0-9]+}};
// CHECK: tensormap.replace.tile.global_address.shared::cta.b1024.b64 [{{%rd?[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_global_address(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_global_address::<{ StateSpace::Global }>(*g, 4096);
        tensormap_replace_global_address::<{ StateSpace::SharedCta }>(*s, 4096);
    }
}

// CHECK-LABEL: .entry tensormap_global_dim(
// CHECK: tensormap.replace.tile.global_dim.global.b1024.b32 [{{%rd?[0-9]+}}], 2, {{%r[0-9]+}};
// CHECK: tensormap.replace.tile.global_dim.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 0, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_global_dim(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_global_dim::<{ StateSpace::Global }, 2>(*g, 8);
        tensormap_replace_global_dim::<{ StateSpace::SharedCta }, 0>(*s, 16);
    }
}

// CHECK-LABEL: .entry tensormap_global_stride(
// CHECK: tensormap.replace.tile.global_stride.global.b1024.b64 [{{%rd?[0-9]+}}], 0, {{%rd[0-9]+}};
// CHECK: tensormap.replace.tile.global_stride.shared::cta.b1024.b64 [{{%rd?[0-9]+}}], 4, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_global_stride(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_global_stride::<{ StateSpace::Global }, 0>(*g, 128);
        tensormap_replace_global_stride::<{ StateSpace::SharedCta }, 4>(*s, 256);
    }
}

// CHECK-LABEL: .entry tensormap_interleave_layout(
// CHECK: tensormap.replace.tile.interleave_layout.global.b1024.b32 [{{%rd?[0-9]+}}], 0;
// CHECK: tensormap.replace.tile.interleave_layout.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 2;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_interleave_layout(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_interleave_layout::<
            { StateSpace::Global },
            { TensormapInterleaveLayout::NoInterleave },
        >(*g);
        tensormap_replace_interleave_layout::<
            { StateSpace::SharedCta },
            { TensormapInterleaveLayout::Interleave32B },
        >(*s);
    }
}

// CHECK-LABEL: .entry tensormap_rank(
// CHECK: tensormap.replace.tile.rank.global.b1024.b32 [{{%rd?[0-9]+}}], {{%r[0-9]+}};
// CHECK: tensormap.replace.tile.rank.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_rank(g: *const *mut c_void, s: *const *mut c_void) {
    unsafe {
        tensormap_replace_rank::<{ StateSpace::Global }>(*g, 4);
        tensormap_replace_rank::<{ StateSpace::SharedCta }>(*s, 4);
    }
}

// CHECK-LABEL: .entry tensormap_swizzle_mode(
// CHECK: tensormap.replace.tile.swizzle_mode.global.b1024.b32 [{{%rd?[0-9]+}}], 0;
// CHECK: tensormap.replace.tile.swizzle_mode.shared::cta.b1024.b32 [{{%rd?[0-9]+}}], 3;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn tensormap_swizzle_mode(
    g: *const *mut c_void,
    s: *const *mut c_void,
) {
    unsafe {
        tensormap_replace_swizzle_mode::<{ StateSpace::Global }, { TensormapSwizzleMode::NoSwizzle }>(
            *g,
        );
        tensormap_replace_swizzle_mode::<
            { StateSpace::SharedCta },
            { TensormapSwizzleMode::Swizzle128B },
        >(*s);
    }
}
