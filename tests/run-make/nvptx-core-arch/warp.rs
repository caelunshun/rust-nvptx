// --- LLM-generated --- //
// Checks the PTX emitted for the warp-level intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry warp_activemask(
// CHECK: activemask.b32 {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_activemask(out: *mut u32) {
    unsafe { *out = activemask() }
}

// CHECK-LABEL: .entry warp_elect_sync(
// CHECK: elect.sync {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_elect_sync(out: *mut u32, mask: u32) {
    unsafe {
        let (lane, leader) = elect_sync(mask);
        *out = lane;
        *out.add(1) = leader as u32;
    }
}

// CHECK-LABEL: .entry warp_match_all_sync_b32(
// CHECK: match.all.sync.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_match_all_sync_b32(out: *mut u32, a: u32, mask: u32) {
    unsafe {
        let (lanes, all) = match_all_sync_b32(a, mask);
        *out = lanes;
        *out.add(1) = all as u32;
    }
}

// CHECK-LABEL: .entry warp_match_all_sync_b64(
// CHECK: match.all.sync.b64 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%rd[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_match_all_sync_b64(out: *mut u32, a: u64, mask: u32) {
    unsafe {
        let (lanes, all) = match_all_sync_b64(a, mask);
        *out = lanes;
        *out.add(1) = all as u32;
    }
}

// CHECK-LABEL: .entry warp_match_any_sync_b32(
// CHECK: match.any.sync.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_match_any_sync_b32(out: *mut u32, a: u32, mask: u32) {
    unsafe { *out = match_any_sync_b32(a, mask) }
}

// CHECK-LABEL: .entry warp_match_any_sync_b64(
// CHECK: match.any.sync.b64 {{%r[0-9]+}}, {{%rd[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_match_any_sync_b64(out: *mut u32, a: u64, mask: u32) {
    unsafe { *out = match_any_sync_b64(a, mask) }
}

// CHECK-LABEL: .entry warp_redux_sync_b32(
// CHECK: redux.sync.and.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.or.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.xor.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_redux_sync_b32(out: *mut u32, a: u32, mask: u32) {
    unsafe {
        *out = redux_sync_b32::<{ ReduxOp::And }>(a, mask);
        *out.add(1) = redux_sync_b32::<{ ReduxOp::Or }>(a, mask);
        *out.add(2) = redux_sync_b32::<{ ReduxOp::Xor }>(a, mask);
    }
}

// CHECK-LABEL: .entry warp_redux_sync_s32(
// CHECK: redux.sync.add.s32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.min.s32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.max.s32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_redux_sync_s32(out: *mut i32, a: i32, mask: u32) {
    unsafe {
        *out = redux_sync_s32::<{ ReduxOp::Add }>(a, mask);
        *out.add(1) = redux_sync_s32::<{ ReduxOp::Min }>(a, mask);
        *out.add(2) = redux_sync_s32::<{ ReduxOp::Max }>(a, mask);
    }
}

// CHECK-LABEL: .entry warp_redux_sync_u32(
// CHECK: redux.sync.add.s32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.min.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.max.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_redux_sync_u32(out: *mut u32, a: u32, mask: u32) {
    unsafe {
        *out = redux_sync_u32::<{ ReduxOp::Add }>(a, mask);
        *out.add(1) = redux_sync_u32::<{ ReduxOp::Min }>(a, mask);
        *out.add(2) = redux_sync_u32::<{ ReduxOp::Max }>(a, mask);
    }
}

// CHECK-LABEL: .entry warp_shfl_sync_b32(
// CHECK: shfl.sync.up.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.down.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.bfly.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.idx.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_shfl_sync_b32(
    out: *mut u32,
    a: u32,
    b: u32,
    c: u32,
    mask: u32,
) {
    unsafe {
        *out = shfl_sync_b32::<{ ShflMode::Up }>(a, b, c, mask);
        *out.add(1) = shfl_sync_b32::<{ ShflMode::Down }>(a, b, c, mask);
        *out.add(2) = shfl_sync_b32::<{ ShflMode::Bfly }>(a, b, c, mask);
        *out.add(3) = shfl_sync_b32::<{ ShflMode::Idx }>(a, b, c, mask);
    }
}

// CHECK-LABEL: .entry warp_shfl_sync_f32(
// CHECK: shfl.sync.up.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.down.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.bfly.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.idx.b32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_shfl_sync_f32(
    out: *mut f32,
    a: f32,
    b: u32,
    c: u32,
    mask: u32,
) {
    unsafe {
        *out = shfl_sync_f32::<{ ShflMode::Up }>(a, b, c, mask);
        *out.add(1) = shfl_sync_f32::<{ ShflMode::Down }>(a, b, c, mask);
        *out.add(2) = shfl_sync_f32::<{ ShflMode::Bfly }>(a, b, c, mask);
        *out.add(3) = shfl_sync_f32::<{ ShflMode::Idx }>(a, b, c, mask);
    }
}

// CHECK-LABEL: .entry warp_shfl_sync_pred_b32(
// CHECK: shfl.sync.up.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.down.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.bfly.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.idx.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_shfl_sync_pred_b32(
    out: *mut u32,
    a: u32,
    b: u32,
    c: u32,
    mask: u32,
) {
    unsafe {
        let (v, p) = shfl_sync_pred_b32::<{ ShflMode::Up }>(a, b, c, mask);
        *out = v;
        *out.add(1) = p as u32;
        let (v, p) = shfl_sync_pred_b32::<{ ShflMode::Down }>(a, b, c, mask);
        *out.add(2) = v;
        *out.add(3) = p as u32;
        let (v, p) = shfl_sync_pred_b32::<{ ShflMode::Bfly }>(a, b, c, mask);
        *out.add(4) = v;
        *out.add(5) = p as u32;
        let (v, p) = shfl_sync_pred_b32::<{ ShflMode::Idx }>(a, b, c, mask);
        *out.add(6) = v;
        *out.add(7) = p as u32;
    }
}

// CHECK-LABEL: .entry warp_shfl_sync_pred_f32(
// CHECK: shfl.sync.up.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.down.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.bfly.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: shfl.sync.idx.b32 {{%r[0-9]+}}|{{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_shfl_sync_pred_f32(
    out: *mut f32,
    a: f32,
    b: u32,
    c: u32,
    mask: u32,
) {
    unsafe {
        let (v, p) = shfl_sync_pred_f32::<{ ShflMode::Up }>(a, b, c, mask);
        *out = v;
        *out.add(1) = p as u32 as f32;
        let (v, p) = shfl_sync_pred_f32::<{ ShflMode::Down }>(a, b, c, mask);
        *out.add(2) = v;
        *out.add(3) = p as u32 as f32;
        let (v, p) = shfl_sync_pred_f32::<{ ShflMode::Bfly }>(a, b, c, mask);
        *out.add(4) = v;
        *out.add(5) = p as u32 as f32;
        let (v, p) = shfl_sync_pred_f32::<{ ShflMode::Idx }>(a, b, c, mask);
        *out.add(6) = v;
        *out.add(7) = p as u32 as f32;
    }
}

// CHECK-LABEL: .entry warp_vote_sync_ballot_b32(
// CHECK: vote.sync.ballot.b32 {{%r[0-9]+}}, {{%p[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_vote_sync_ballot_b32(out: *mut u32, a: bool, mask: u32) {
    unsafe { *out = vote_sync_ballot_b32(a, mask) }
}

// CHECK-LABEL: .entry warp_vote_sync_pred(
// CHECK: vote.sync.all.pred {{%p[0-9]+}}, {{%p[0-9]+}}, {{%r[0-9]+}};
// CHECK: vote.sync.any.pred {{%p[0-9]+}}, {{%p[0-9]+}}, {{%r[0-9]+}};
// CHECK: vote.sync.uni.pred {{%p[0-9]+}}, {{%p[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_vote_sync_pred(out: *mut bool, a: bool, mask: u32) {
    unsafe {
        *out = vote_sync_pred::<{ VoteMode::All }>(a, mask);
        *out.add(1) = vote_sync_pred::<{ VoteMode::Any }>(a, mask);
        *out.add(2) = vote_sync_pred::<{ VoteMode::Uni }>(a, mask);
    }
}
