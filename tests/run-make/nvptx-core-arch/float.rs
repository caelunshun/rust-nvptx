// --- LLM-generated --- //
// Checks the PTX emitted for the floating-point arithmetic intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx, f16)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry float_abs_f16(
// CHECK-DAG: abs.f16 %
// CHECK-DAG: abs.ftz.f16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_abs_f16(out: *mut f16, a: f16) {
    unsafe {
        *out.add(0) = abs_f16::<false>(a);
        *out.add(1) = abs_f16::<true>(a);
    }
}

// CHECK-LABEL: .entry float_abs_f16x2(
// CHECK-DAG: abs.f16x2 %
// CHECK-DAG: abs.ftz.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_abs_f16x2(out: *mut f16x2, pa: *const f16x2) {
    unsafe {
        *out.add(0) = abs_f16x2::<false>(*pa);
        *out.add(1) = abs_f16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry float_abs_f32(
// CHECK-DAG: abs.f32 %
// CHECK-DAG: abs.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_abs_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = abs_f32::<false>(a);
        *out.add(1) = abs_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry float_abs_f64(
// CHECK-DAG: abs.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_abs_f64(out: *mut f64, a: f64) {
    unsafe {
        *out.add(0) = abs_f64(a);
    }
}

// CHECK-LABEL: .entry float_add_f16(
// CHECK-DAG: add.rn.f16 %
// CHECK-DAG: add.rn.sat.f16 %
// CHECK-DAG: add.rn.ftz.sat.f16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_add_f16(out: *mut f16, a: f16, b: f16) {
    unsafe {
        *out.add(0) = add_f16::<false, false>(a, b);
        *out.add(1) = add_f16::<false, true>(a, b);
        *out.add(2) = add_f16::<true, true>(a, b);
    }
}

// CHECK-LABEL: .entry float_add_f16x2(
// CHECK-DAG: add.rn.f16x2 %
// CHECK-DAG: add.rn.sat.f16x2 %
// CHECK-DAG: add.rn.ftz.sat.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_add_f16x2(
    out: *mut f16x2,
    pa: *const f16x2,
    pb: *const f16x2,
) {
    unsafe {
        *out.add(0) = add_f16x2::<false, false>(*pa, *pb);
        *out.add(1) = add_f16x2::<false, true>(*pa, *pb);
        *out.add(2) = add_f16x2::<true, true>(*pa, *pb);
    }
}

// CHECK-LABEL: .entry float_add_f32(
// CHECK-DAG: add.rn.f32 %
// CHECK-DAG: add.rn.sat.f32 %
// CHECK-DAG: add.rn.ftz.f32 %
// CHECK-DAG: add.rn.sat.ftz.f32 %
// CHECK-DAG: add.rz.f32 %
// CHECK-DAG: add.rz.sat.f32 %
// CHECK-DAG: add.rz.ftz.f32 %
// CHECK-DAG: add.rz.sat.ftz.f32 %
// CHECK-DAG: add.rm.f32 %
// CHECK-DAG: add.rm.sat.f32 %
// CHECK-DAG: add.rm.ftz.f32 %
// CHECK-DAG: add.rm.sat.ftz.f32 %
// CHECK-DAG: add.rp.f32 %
// CHECK-DAG: add.rp.sat.f32 %
// CHECK-DAG: add.rp.ftz.f32 %
// CHECK-DAG: add.rp.sat.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_add_f32(out: *mut f32, a: f32, b: f32) {
    unsafe {
        *out.add(0) = add_f32::<{ Rounding::Rn }, false, false>(a, b);
        *out.add(1) = add_f32::<{ Rounding::Rn }, false, true>(a, b);
        *out.add(2) = add_f32::<{ Rounding::Rn }, true, false>(a, b);
        *out.add(3) = add_f32::<{ Rounding::Rn }, true, true>(a, b);
        *out.add(4) = add_f32::<{ Rounding::Rz }, false, false>(a, b);
        *out.add(5) = add_f32::<{ Rounding::Rz }, false, true>(a, b);
        *out.add(6) = add_f32::<{ Rounding::Rz }, true, false>(a, b);
        *out.add(7) = add_f32::<{ Rounding::Rz }, true, true>(a, b);
        *out.add(8) = add_f32::<{ Rounding::Rm }, false, false>(a, b);
        *out.add(9) = add_f32::<{ Rounding::Rm }, false, true>(a, b);
        *out.add(10) = add_f32::<{ Rounding::Rm }, true, false>(a, b);
        *out.add(11) = add_f32::<{ Rounding::Rm }, true, true>(a, b);
        *out.add(12) = add_f32::<{ Rounding::Rp }, false, false>(a, b);
        *out.add(13) = add_f32::<{ Rounding::Rp }, false, true>(a, b);
        *out.add(14) = add_f32::<{ Rounding::Rp }, true, false>(a, b);
        *out.add(15) = add_f32::<{ Rounding::Rp }, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry float_add_f64(
// CHECK-DAG: add.rn.f64 %
// CHECK-DAG: add.rz.f64 %
// CHECK-DAG: add.rm.f64 %
// CHECK-DAG: add.rp.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_add_f64(out: *mut f64, a: f64, b: f64) {
    unsafe {
        *out.add(0) = add_f64::<{ Rounding::Rn }>(a, b);
        *out.add(1) = add_f64::<{ Rounding::Rz }>(a, b);
        *out.add(2) = add_f64::<{ Rounding::Rm }>(a, b);
        *out.add(3) = add_f64::<{ Rounding::Rp }>(a, b);
    }
}

// CHECK-LABEL: .entry float_cos_f32(
// CHECK-DAG: cos.approx.f32 %
// CHECK-DAG: cos.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_cos_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = cos_f32::<false>(a);
        *out.add(1) = cos_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry float_div_approx_f32(
// CHECK-DAG: div.approx.f32 %
// CHECK-DAG: div.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_div_approx_f32(out: *mut f32, a: f32, b: f32) {
    unsafe {
        *out.add(0) = div_approx_f32::<false>(a, b);
        *out.add(1) = div_approx_f32::<true>(a, b);
    }
}

// CHECK-LABEL: .entry float_div_f32(
// CHECK-DAG: div.rn.f32 %
// CHECK-DAG: div.rn.ftz.f32 %
// CHECK-DAG: div.rz.f32 %
// CHECK-DAG: div.rz.ftz.f32 %
// CHECK-DAG: div.rm.f32 %
// CHECK-DAG: div.rm.ftz.f32 %
// CHECK-DAG: div.rp.f32 %
// CHECK-DAG: div.rp.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_div_f32(out: *mut f32, a: f32, b: f32) {
    unsafe {
        *out.add(0) = div_f32::<{ Rounding::Rn }, false>(a, b);
        *out.add(1) = div_f32::<{ Rounding::Rn }, true>(a, b);
        *out.add(2) = div_f32::<{ Rounding::Rz }, false>(a, b);
        *out.add(3) = div_f32::<{ Rounding::Rz }, true>(a, b);
        *out.add(4) = div_f32::<{ Rounding::Rm }, false>(a, b);
        *out.add(5) = div_f32::<{ Rounding::Rm }, true>(a, b);
        *out.add(6) = div_f32::<{ Rounding::Rp }, false>(a, b);
        *out.add(7) = div_f32::<{ Rounding::Rp }, true>(a, b);
    }
}

// CHECK-LABEL: .entry float_div_f64(
// CHECK-DAG: div.rn.f64 %
// CHECK-DAG: div.rz.f64 %
// CHECK-DAG: div.rm.f64 %
// CHECK-DAG: div.rp.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_div_f64(out: *mut f64, a: f64, b: f64) {
    unsafe {
        *out.add(0) = div_f64::<{ Rounding::Rn }>(a, b);
        *out.add(1) = div_f64::<{ Rounding::Rz }>(a, b);
        *out.add(2) = div_f64::<{ Rounding::Rm }>(a, b);
        *out.add(3) = div_f64::<{ Rounding::Rp }>(a, b);
    }
}

// CHECK-LABEL: .entry float_div_full_f32(
// CHECK-DAG: div.full.f32 %
// CHECK-DAG: div.full.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_div_full_f32(out: *mut f32, a: f32, b: f32) {
    unsafe {
        *out.add(0) = div_full_f32::<false>(a, b);
        *out.add(1) = div_full_f32::<true>(a, b);
    }
}

// CHECK-LABEL: .entry float_ex2_f16(
// CHECK-DAG: ex2.approx.f16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_ex2_f16(out: *mut f16, a: f16) {
    unsafe {
        *out.add(0) = ex2_f16(a);
    }
}

// CHECK-LABEL: .entry float_ex2_f16x2(
// CHECK-DAG: ex2.approx.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_ex2_f16x2(out: *mut f16x2, pa: *const f16x2) {
    unsafe {
        *out.add(0) = ex2_f16x2(*pa);
    }
}

// CHECK-LABEL: .entry float_ex2_f32(
// CHECK-DAG: ex2.approx.f32 %
// CHECK-DAG: ex2.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_ex2_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = ex2_f32::<false>(a);
        *out.add(1) = ex2_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry float_fma_bf16(
// CHECK-DAG: fma.rn.bf16 %
// CHECK-DAG: fma.rn.relu.bf16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_bf16(out: *mut bf16, a: bf16, b: bf16, c: bf16) {
    unsafe {
        *out.add(0) = fma_bf16::<false>(a, b, c);
        *out.add(1) = fma_bf16::<true>(a, b, c);
    }
}

// CHECK-LABEL: .entry float_fma_bf16x2(
// CHECK-DAG: fma.rn.bf16x2 %
// CHECK-DAG: fma.rn.relu.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_bf16x2(
    out: *mut bf16x2,
    pa: *const bf16x2,
    pb: *const bf16x2,
    pc: *const bf16x2,
) {
    unsafe {
        *out.add(0) = fma_bf16x2::<false>(*pa, *pb, *pc);
        *out.add(1) = fma_bf16x2::<true>(*pa, *pb, *pc);
    }
}

// CHECK-LABEL: .entry float_fma_f16(
// CHECK-DAG: fma.rn.f16 %
// CHECK-DAG: fma.rn.ftz.f16 %
// CHECK-DAG: fma.rn.sat.f16 %
// CHECK-DAG: fma.rn.ftz.sat.f16 %
// CHECK-DAG: fma.rn.relu.f16 %
// CHECK-DAG: fma.rn.ftz.relu.f16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_f16(out: *mut f16, a: f16, b: f16, c: f16) {
    unsafe {
        *out.add(0) = fma_f16::<false, false, false>(a, b, c);
        *out.add(1) = fma_f16::<true, false, false>(a, b, c);
        *out.add(2) = fma_f16::<false, true, false>(a, b, c);
        *out.add(3) = fma_f16::<true, true, false>(a, b, c);
        *out.add(4) = fma_f16::<false, false, true>(a, b, c);
        *out.add(5) = fma_f16::<true, false, true>(a, b, c);
    }
}

// CHECK-LABEL: .entry float_fma_f16x2(
// CHECK-DAG: fma.rn.f16x2 %
// CHECK-DAG: fma.rn.ftz.f16x2 %
// CHECK-DAG: fma.rn.sat.f16x2 %
// CHECK-DAG: fma.rn.ftz.sat.f16x2 %
// CHECK-DAG: fma.rn.relu.f16x2 %
// CHECK-DAG: fma.rn.ftz.relu.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_f16x2(
    out: *mut f16x2,
    pa: *const f16x2,
    pb: *const f16x2,
    pc: *const f16x2,
) {
    unsafe {
        *out.add(0) = fma_f16x2::<false, false, false>(*pa, *pb, *pc);
        *out.add(1) = fma_f16x2::<true, false, false>(*pa, *pb, *pc);
        *out.add(2) = fma_f16x2::<false, true, false>(*pa, *pb, *pc);
        *out.add(3) = fma_f16x2::<true, true, false>(*pa, *pb, *pc);
        *out.add(4) = fma_f16x2::<false, false, true>(*pa, *pb, *pc);
        *out.add(5) = fma_f16x2::<true, false, true>(*pa, *pb, *pc);
    }
}

// CHECK-LABEL: .entry float_fma_f32(
// CHECK-DAG: fma.rn.f32 %
// CHECK-DAG: fma.rn.sat.f32 %
// CHECK-DAG: fma.rn.ftz.f32 %
// CHECK-DAG: fma.rn.ftz.sat.f32 %
// CHECK-DAG: fma.rz.f32 %
// CHECK-DAG: fma.rz.sat.f32 %
// CHECK-DAG: fma.rz.ftz.f32 %
// CHECK-DAG: fma.rz.ftz.sat.f32 %
// CHECK-DAG: fma.rm.f32 %
// CHECK-DAG: fma.rm.sat.f32 %
// CHECK-DAG: fma.rm.ftz.f32 %
// CHECK-DAG: fma.rm.ftz.sat.f32 %
// CHECK-DAG: fma.rp.f32 %
// CHECK-DAG: fma.rp.sat.f32 %
// CHECK-DAG: fma.rp.ftz.f32 %
// CHECK-DAG: fma.rp.ftz.sat.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_f32(out: *mut f32, a: f32, b: f32, c: f32) {
    unsafe {
        *out.add(0) = fma_f32::<{ Rounding::Rn }, false, false>(a, b, c);
        *out.add(1) = fma_f32::<{ Rounding::Rn }, false, true>(a, b, c);
        *out.add(2) = fma_f32::<{ Rounding::Rn }, true, false>(a, b, c);
        *out.add(3) = fma_f32::<{ Rounding::Rn }, true, true>(a, b, c);
        *out.add(4) = fma_f32::<{ Rounding::Rz }, false, false>(a, b, c);
        *out.add(5) = fma_f32::<{ Rounding::Rz }, false, true>(a, b, c);
        *out.add(6) = fma_f32::<{ Rounding::Rz }, true, false>(a, b, c);
        *out.add(7) = fma_f32::<{ Rounding::Rz }, true, true>(a, b, c);
        *out.add(8) = fma_f32::<{ Rounding::Rm }, false, false>(a, b, c);
        *out.add(9) = fma_f32::<{ Rounding::Rm }, false, true>(a, b, c);
        *out.add(10) = fma_f32::<{ Rounding::Rm }, true, false>(a, b, c);
        *out.add(11) = fma_f32::<{ Rounding::Rm }, true, true>(a, b, c);
        *out.add(12) = fma_f32::<{ Rounding::Rp }, false, false>(a, b, c);
        *out.add(13) = fma_f32::<{ Rounding::Rp }, false, true>(a, b, c);
        *out.add(14) = fma_f32::<{ Rounding::Rp }, true, false>(a, b, c);
        *out.add(15) = fma_f32::<{ Rounding::Rp }, true, true>(a, b, c);
    }
}

// CHECK-LABEL: .entry float_fma_f64(
// CHECK-DAG: fma.rn.f64 %
// CHECK-DAG: fma.rz.f64 %
// CHECK-DAG: fma.rm.f64 %
// CHECK-DAG: fma.rp.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_f64(out: *mut f64, a: f64, b: f64, c: f64) {
    unsafe {
        *out.add(0) = fma_f64::<{ Rounding::Rn }>(a, b, c);
        *out.add(1) = fma_f64::<{ Rounding::Rz }>(a, b, c);
        *out.add(2) = fma_f64::<{ Rounding::Rm }>(a, b, c);
        *out.add(3) = fma_f64::<{ Rounding::Rp }>(a, b, c);
    }
}

// CHECK-LABEL: .entry float_fma_oob_f16(
// CHECK-DAG: fma.rn.oob.f16 %
// CHECK-DAG: fma.rn.oob.relu.f16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_oob_f16(out: *mut f16, a: f16, b: f16, c: f16) {
    unsafe {
        *out.add(0) = fma_oob_f16::<false>(a, b, c);
        *out.add(1) = fma_oob_f16::<true>(a, b, c);
    }
}

// CHECK-LABEL: .entry float_fma_oob_f16x2(
// CHECK-DAG: fma.rn.oob.f16x2 %
// CHECK-DAG: fma.rn.oob.relu.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_fma_oob_f16x2(
    out: *mut f16x2,
    pa: *const f16x2,
    pb: *const f16x2,
    pc: *const f16x2,
) {
    unsafe {
        *out.add(0) = fma_oob_f16x2::<false>(*pa, *pb, *pc);
        *out.add(1) = fma_oob_f16x2::<true>(*pa, *pb, *pc);
    }
}

// CHECK-LABEL: .entry float_lg2_f32(
// CHECK-DAG: lg2.approx.f32 %
// CHECK-DAG: lg2.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_lg2_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = lg2_f32::<false>(a);
        *out.add(1) = lg2_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry float_mul_f16(
// CHECK-DAG: mul.rn.f16 %
// CHECK-DAG: mul.rn.sat.f16 %
// CHECK-DAG: mul.rn.ftz.sat.f16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_mul_f16(out: *mut f16, a: f16, b: f16) {
    unsafe {
        *out.add(0) = mul_f16::<false, false>(a, b);
        *out.add(1) = mul_f16::<false, true>(a, b);
        *out.add(2) = mul_f16::<true, true>(a, b);
    }
}

// CHECK-LABEL: .entry float_mul_f16x2(
// CHECK-DAG: mul.rn.f16x2 %
// CHECK-DAG: mul.rn.sat.f16x2 %
// CHECK-DAG: mul.rn.ftz.sat.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_mul_f16x2(
    out: *mut f16x2,
    pa: *const f16x2,
    pb: *const f16x2,
) {
    unsafe {
        *out.add(0) = mul_f16x2::<false, false>(*pa, *pb);
        *out.add(1) = mul_f16x2::<false, true>(*pa, *pb);
        *out.add(2) = mul_f16x2::<true, true>(*pa, *pb);
    }
}

// CHECK-LABEL: .entry float_mul_f32(
// CHECK-DAG: mul.rn.f32 %
// CHECK-DAG: mul.rn.ftz.f32 %
// CHECK-DAG: mul.rz.f32 %
// CHECK-DAG: mul.rz.ftz.f32 %
// CHECK-DAG: mul.rm.f32 %
// CHECK-DAG: mul.rm.ftz.f32 %
// CHECK-DAG: mul.rp.f32 %
// CHECK-DAG: mul.rp.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_mul_f32(out: *mut f32, a: f32, b: f32) {
    unsafe {
        *out.add(0) = mul_f32::<{ Rounding::Rn }, false>(a, b);
        *out.add(1) = mul_f32::<{ Rounding::Rn }, true>(a, b);
        *out.add(2) = mul_f32::<{ Rounding::Rz }, false>(a, b);
        *out.add(3) = mul_f32::<{ Rounding::Rz }, true>(a, b);
        *out.add(4) = mul_f32::<{ Rounding::Rm }, false>(a, b);
        *out.add(5) = mul_f32::<{ Rounding::Rm }, true>(a, b);
        *out.add(6) = mul_f32::<{ Rounding::Rp }, false>(a, b);
        *out.add(7) = mul_f32::<{ Rounding::Rp }, true>(a, b);
    }
}

// CHECK-LABEL: .entry float_mul_f64(
// CHECK-DAG: mul.rn.f64 %
// CHECK-DAG: mul.rz.f64 %
// CHECK-DAG: mul.rm.f64 %
// CHECK-DAG: mul.rp.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_mul_f64(out: *mut f64, a: f64, b: f64) {
    unsafe {
        *out.add(0) = mul_f64::<{ Rounding::Rn }>(a, b);
        *out.add(1) = mul_f64::<{ Rounding::Rz }>(a, b);
        *out.add(2) = mul_f64::<{ Rounding::Rm }>(a, b);
        *out.add(3) = mul_f64::<{ Rounding::Rp }>(a, b);
    }
}

// CHECK-LABEL: .entry float_neg_bf16(
// CHECK-DAG: neg.bf16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_neg_bf16(out: *mut bf16, a: bf16) {
    unsafe {
        *out.add(0) = neg_bf16(a);
    }
}

// CHECK-LABEL: .entry float_neg_bf16x2(
// CHECK-DAG: neg.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_neg_bf16x2(out: *mut bf16x2, pa: *const bf16x2) {
    unsafe {
        *out.add(0) = neg_bf16x2(*pa);
    }
}

// CHECK-LABEL: .entry float_neg_f16(
// CHECK-DAG: neg.f16 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_neg_f16(out: *mut f16, a: f16) {
    unsafe {
        *out.add(0) = neg_f16(a);
    }
}

// CHECK-LABEL: .entry float_neg_f16x2(
// CHECK-DAG: neg.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_neg_f16x2(out: *mut f16x2, pa: *const f16x2) {
    unsafe {
        *out.add(0) = neg_f16x2(*pa);
    }
}

// CHECK-LABEL: .entry float_rcp_approx_f64(
// CHECK-DAG: rcp.approx.ftz.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_rcp_approx_f64(out: *mut f64, a: f64) {
    unsafe {
        *out.add(0) = rcp_approx_f64(a);
    }
}

// CHECK-LABEL: .entry float_rcp_approx_ftz_f32(
// CHECK-DAG: rcp.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_rcp_approx_ftz_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = rcp_approx_ftz_f32(a);
    }
}

// CHECK-LABEL: .entry float_rcp_f32(
// CHECK-DAG: rcp.rn.f32 %
// CHECK-DAG: rcp.rn.ftz.f32 %
// CHECK-DAG: rcp.rz.f32 %
// CHECK-DAG: rcp.rz.ftz.f32 %
// CHECK-DAG: rcp.rm.f32 %
// CHECK-DAG: rcp.rm.ftz.f32 %
// CHECK-DAG: rcp.rp.f32 %
// CHECK-DAG: rcp.rp.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_rcp_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = rcp_f32::<{ Rounding::Rn }, false>(a);
        *out.add(1) = rcp_f32::<{ Rounding::Rn }, true>(a);
        *out.add(2) = rcp_f32::<{ Rounding::Rz }, false>(a);
        *out.add(3) = rcp_f32::<{ Rounding::Rz }, true>(a);
        *out.add(4) = rcp_f32::<{ Rounding::Rm }, false>(a);
        *out.add(5) = rcp_f32::<{ Rounding::Rm }, true>(a);
        *out.add(6) = rcp_f32::<{ Rounding::Rp }, false>(a);
        *out.add(7) = rcp_f32::<{ Rounding::Rp }, true>(a);
    }
}

// CHECK-LABEL: .entry float_rcp_f64(
// CHECK-DAG: rcp.rn.f64 %
// CHECK-DAG: rcp.rz.f64 %
// CHECK-DAG: rcp.rm.f64 %
// CHECK-DAG: rcp.rp.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_rcp_f64(out: *mut f64, a: f64) {
    unsafe {
        *out.add(0) = rcp_f64::<{ Rounding::Rn }>(a);
        *out.add(1) = rcp_f64::<{ Rounding::Rz }>(a);
        *out.add(2) = rcp_f64::<{ Rounding::Rm }>(a);
        *out.add(3) = rcp_f64::<{ Rounding::Rp }>(a);
    }
}

// CHECK-LABEL: .entry float_rsqrt_approx_f32(
// CHECK-DAG: rsqrt.approx.f32 %
// CHECK-DAG: rsqrt.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_rsqrt_approx_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = rsqrt_approx_f32::<false>(a);
        *out.add(1) = rsqrt_approx_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry float_rsqrt_approx_f64(
// CHECK-DAG: rsqrt.approx.f64 %
// CHECK-DAG: rsqrt.approx.ftz.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_rsqrt_approx_f64(out: *mut f64, a: f64) {
    unsafe {
        *out.add(0) = rsqrt_approx_f64::<false>(a);
        *out.add(1) = rsqrt_approx_f64::<true>(a);
    }
}

// CHECK-LABEL: .entry float_sin_f32(
// CHECK-DAG: sin.approx.f32 %
// CHECK-DAG: sin.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_sin_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = sin_f32::<false>(a);
        *out.add(1) = sin_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry float_sqrt_approx_f32(
// CHECK-DAG: sqrt.approx.f32 %
// CHECK-DAG: sqrt.approx.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_sqrt_approx_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = sqrt_approx_f32::<false>(a);
        *out.add(1) = sqrt_approx_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry float_sqrt_f32(
// CHECK-DAG: sqrt.rn.f32 %
// CHECK-DAG: sqrt.rn.ftz.f32 %
// CHECK-DAG: sqrt.rz.f32 %
// CHECK-DAG: sqrt.rz.ftz.f32 %
// CHECK-DAG: sqrt.rm.f32 %
// CHECK-DAG: sqrt.rm.ftz.f32 %
// CHECK-DAG: sqrt.rp.f32 %
// CHECK-DAG: sqrt.rp.ftz.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_sqrt_f32(out: *mut f32, a: f32) {
    unsafe {
        *out.add(0) = sqrt_f32::<{ Rounding::Rn }, false>(a);
        *out.add(1) = sqrt_f32::<{ Rounding::Rn }, true>(a);
        *out.add(2) = sqrt_f32::<{ Rounding::Rz }, false>(a);
        *out.add(3) = sqrt_f32::<{ Rounding::Rz }, true>(a);
        *out.add(4) = sqrt_f32::<{ Rounding::Rm }, false>(a);
        *out.add(5) = sqrt_f32::<{ Rounding::Rm }, true>(a);
        *out.add(6) = sqrt_f32::<{ Rounding::Rp }, false>(a);
        *out.add(7) = sqrt_f32::<{ Rounding::Rp }, true>(a);
    }
}

// CHECK-LABEL: .entry float_sqrt_f64(
// CHECK-DAG: sqrt.rn.f64 %
// CHECK-DAG: sqrt.rz.f64 %
// CHECK-DAG: sqrt.rm.f64 %
// CHECK-DAG: sqrt.rp.f64 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_sqrt_f64(out: *mut f64, a: f64) {
    unsafe {
        *out.add(0) = sqrt_f64::<{ Rounding::Rn }>(a);
        *out.add(1) = sqrt_f64::<{ Rounding::Rz }>(a);
        *out.add(2) = sqrt_f64::<{ Rounding::Rm }>(a);
        *out.add(3) = sqrt_f64::<{ Rounding::Rp }>(a);
    }
}

// CHECK-LABEL: .entry float_sub_f16x2(
// CHECK-DAG: sub.rn.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn float_sub_f16x2(
    out: *mut f16x2,
    pa: *const f16x2,
    pb: *const f16x2,
) {
    unsafe {
        *out.add(0) = sub_f16x2(*pa, *pb);
    }
}
