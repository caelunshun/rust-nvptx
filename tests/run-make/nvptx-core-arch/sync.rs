// --- LLM-generated --- //
// Checks the PTX emitted for the synchronization, barrier and fence intrinsics in
// `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry sync_bar_warp_sync(
// CHECK: bar.warp.sync {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_bar_warp_sync(mask: u32) {
    unsafe { bar_warp_sync(mask) }
}

// CHECK-LABEL: .entry sync_barrier_arrive(
// CHECK: barrier.arrive {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: bar.arrive {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_arrive(a: u32, b: u32) {
    unsafe {
        barrier_arrive::<false>(a, b);
        barrier_arrive::<true>(a, b);
    }
}

// CHECK-LABEL: .entry sync_barrier_cluster_arrive(
// CHECK: barrier.cluster.arrive;
// CHECK: barrier.cluster.arrive.aligned;
// CHECK: barrier.cluster.arrive.relaxed;
// CHECK: barrier.cluster.arrive.relaxed.aligned;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_cluster_arrive() {
    unsafe {
        barrier_cluster_arrive::<{ Semantics::Release }, false>();
        barrier_cluster_arrive::<{ Semantics::Release }, true>();
        barrier_cluster_arrive::<{ Semantics::Relaxed }, false>();
        barrier_cluster_arrive::<{ Semantics::Relaxed }, true>();
    }
}

// CHECK-LABEL: .entry sync_barrier_cluster_wait(
// CHECK: barrier.cluster.wait;
// CHECK: barrier.cluster.wait.aligned;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_cluster_wait() {
    unsafe {
        barrier_cluster_wait::<false>();
        barrier_cluster_wait::<true>();
    }
}

// CHECK-LABEL: .entry sync_barrier_red_popc(
// CHECK: barrier.red.popc.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: bar.red.popc.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_red_popc(out: *mut u32, a: u32, c: u32) {
    unsafe {
        *out = barrier_red_popc::<false>(a, c != 0);
        *out.add(1) = barrier_red_popc::<true>(a, c != 0);
    }
}

// CHECK-LABEL: .entry sync_barrier_red_popc_count(
// CHECK: barrier.red.popc.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: bar.red.popc.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_red_popc_count(
    out: *mut u32,
    a: u32,
    b: u32,
    c: u32,
) {
    unsafe {
        *out = barrier_red_popc_count::<false>(a, b, c != 0);
        *out.add(1) = barrier_red_popc_count::<true>(a, b, c != 0);
    }
}

// CHECK-LABEL: .entry sync_barrier_red_pred(
// CHECK: barrier.red.and.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: bar.red.and.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: barrier.red.or.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: bar.red.or.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_red_pred(out: *mut u32, a: u32, c: u32) {
    unsafe {
        *out = barrier_red_pred::<{ BarrierRedOp::And }, false>(a, c != 0) as u32;
        *out.add(1) = barrier_red_pred::<{ BarrierRedOp::And }, true>(a, c != 0) as u32;
        *out.add(2) = barrier_red_pred::<{ BarrierRedOp::Or }, false>(a, c != 0) as u32;
        *out.add(3) = barrier_red_pred::<{ BarrierRedOp::Or }, true>(a, c != 0) as u32;
    }
}

// CHECK-LABEL: .entry sync_barrier_red_pred_count(
// CHECK: barrier.red.and.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: bar.red.and.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: barrier.red.or.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
// CHECK: bar.red.or.pred {{%p[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%p[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_red_pred_count(
    out: *mut u32,
    a: u32,
    b: u32,
    c: u32,
) {
    unsafe {
        *out = barrier_red_pred_count::<{ BarrierRedOp::And }, false>(a, b, c != 0) as u32;
        *out.add(1) = barrier_red_pred_count::<{ BarrierRedOp::And }, true>(a, b, c != 0) as u32;
        *out.add(2) = barrier_red_pred_count::<{ BarrierRedOp::Or }, false>(a, b, c != 0) as u32;
        *out.add(3) = barrier_red_pred_count::<{ BarrierRedOp::Or }, true>(a, b, c != 0) as u32;
    }
}

// CHECK-LABEL: .entry sync_barrier_sync(
// CHECK: barrier.sync {{%r[0-9]+}};
// CHECK: bar.sync 0;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_sync(a: u32) {
    unsafe {
        barrier_sync::<false>(a);
        barrier_sync::<true>(0);
    }
}

// CHECK-LABEL: .entry sync_barrier_sync_count(
// CHECK: barrier.sync {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: bar.sync {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_barrier_sync_count(a: u32, b: u32) {
    unsafe {
        barrier_sync_count::<false>(a, b);
        barrier_sync_count::<true>(a, b);
    }
}

// CHECK-LABEL: .entry sync_fence_mbarrier_init(
// CHECK: fence.mbarrier_init.release.cluster;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_fence_mbarrier_init() {
    unsafe { fence_mbarrier_init() }
}

// CHECK-LABEL: .entry sync_fence_proxy(
// CHECK: fence.proxy.alias;
// CHECK: fence.proxy.async;
// CHECK: fence.proxy.async.global;
// CHECK: fence.proxy.async.shared::cta;
// CHECK: fence.proxy.async.shared::cluster;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_fence_proxy() {
    unsafe {
        fence_proxy::<{ ProxyKind::Alias }>();
        fence_proxy::<{ ProxyKind::Async }>();
        fence_proxy::<{ ProxyKind::AsyncGlobal }>();
        fence_proxy::<{ ProxyKind::AsyncSharedCta }>();
        fence_proxy::<{ ProxyKind::AsyncSharedCluster }>();
    }
}

// CHECK-LABEL: .entry sync_fence_proxy_async_generic_sync_restrict(
// CHECK: fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;
// CHECK: fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_fence_proxy_async_generic_sync_restrict() {
    unsafe {
        fence_proxy_async_generic_sync_restrict::<{ Semantics::Acquire }>();
        fence_proxy_async_generic_sync_restrict::<{ Semantics::Release }>();
    }
}

// CHECK-LABEL: .entry sync_fence_proxy_tensormap_generic_acquire(
// CHECK: fence.proxy.tensormap::generic.acquire.cta [{{%rd[0-9]+}}], 128;
// CHECK: fence.proxy.tensormap::generic.acquire.cluster [{{%rd[0-9]+}}], 128;
// CHECK: fence.proxy.tensormap::generic.acquire.gpu [{{%rd[0-9]+}}], 128;
// CHECK: fence.proxy.tensormap::generic.acquire.sys [{{%rd[0-9]+}}], 128;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_fence_proxy_tensormap_generic_acquire(tmap: *const c_void) {
    unsafe {
        fence_proxy_tensormap_generic_acquire::<{ Scope::Cta }>(tmap);
        fence_proxy_tensormap_generic_acquire::<{ Scope::Cluster }>(tmap);
        fence_proxy_tensormap_generic_acquire::<{ Scope::Gpu }>(tmap);
        fence_proxy_tensormap_generic_acquire::<{ Scope::Sys }>(tmap);
    }
}

// CHECK-LABEL: .entry sync_fence_proxy_tensormap_generic_release(
// CHECK: fence.proxy.tensormap::generic.release.cta;
// CHECK: fence.proxy.tensormap::generic.release.cluster;
// CHECK: fence.proxy.tensormap::generic.release.gpu;
// CHECK: fence.proxy.tensormap::generic.release.sys;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_fence_proxy_tensormap_generic_release() {
    unsafe {
        fence_proxy_tensormap_generic_release::<{ Scope::Cta }>();
        fence_proxy_tensormap_generic_release::<{ Scope::Cluster }>();
        fence_proxy_tensormap_generic_release::<{ Scope::Gpu }>();
        fence_proxy_tensormap_generic_release::<{ Scope::Sys }>();
    }
}

// CHECK-LABEL: .entry sync_fence_sc(
// CHECK: membar.cta;
// CHECK: fence.sc.cluster;
// CHECK: membar.gl;
// CHECK: membar.sys;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_fence_sc() {
    unsafe {
        fence_sc::<{ Scope::Cta }>();
        fence_sc::<{ Scope::Cluster }>();
        fence_sc::<{ Scope::Gpu }>();
        fence_sc::<{ Scope::Sys }>();
    }
}

// CHECK-LABEL: .entry sync_fence_sync_restrict(
// CHECK: fence.acquire.sync_restrict::shared::cluster.cluster;
// CHECK: fence.release.sync_restrict::shared::cta.cluster;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_fence_sync_restrict() {
    unsafe {
        fence_sync_restrict::<{ Semantics::Acquire }>();
        fence_sync_restrict::<{ Semantics::Release }>();
    }
}

// CHECK-LABEL: .entry sync_griddepcontrol(
// CHECK: griddepcontrol.launch_dependents;
// CHECK: griddepcontrol.wait;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_griddepcontrol() {
    unsafe {
        griddepcontrol::<{ GriddepcontrolAction::LaunchDependents }>();
        griddepcontrol::<{ GriddepcontrolAction::Wait }>();
    }
}
