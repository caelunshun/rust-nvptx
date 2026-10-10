// --- LLM-generated --- //
// Checks the PTX emitted for the mbarrier intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// Pointers are loaded from memory so that LLVM can't infer their address space.

// CHECK-LABEL: .entry kernel_mbarrier_arrive(
// CHECK: mbarrier.arrive.b64 {{%rd[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: mbarrier.arrive.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive(bar: *const *mut u64, out: *mut u64) {
    unsafe {
        *out = mbarrier_arrive::<{ StateSpace::Generic }>(*bar);
        *out.add(1) = mbarrier_arrive::<{ StateSpace::SharedCta }>(*bar);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_drop(
// CHECK: mbarrier.arrive_drop.b64 {{%rd[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: mbarrier.arrive_drop.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_drop(bar: *const *mut u64, out: *mut u64) {
    unsafe {
        *out = mbarrier_arrive_drop::<{ StateSpace::Generic }>(*bar);
        *out.add(1) = mbarrier_arrive_drop::<{ StateSpace::SharedCta }>(*bar);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_drop_expect_tx(
// CHECK: mbarrier.arrive_drop.expect_tx.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.expect_tx.release.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.expect_tx.relaxed.cta.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.expect_tx.relaxed.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_drop_expect_tx(
    bar: *const *mut u64,
    tx: u32,
    out: *mut u64,
) {
    unsafe {
        let bar = *bar;
        *out = mbarrier_arrive_drop_expect_tx::<{ Scope::Cta }, { Semantics::Release }>(bar, tx);
        *out.add(1) =
            mbarrier_arrive_drop_expect_tx::<{ Scope::Cluster }, { Semantics::Release }>(bar, tx);
        *out.add(2) =
            mbarrier_arrive_drop_expect_tx::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, tx);
        *out.add(3) =
            mbarrier_arrive_drop_expect_tx::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, tx);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_drop_expect_tx_cluster(
// CHECK: mbarrier.arrive_drop.expect_tx.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.expect_tx.release.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.expect_tx.relaxed.cta.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.expect_tx.relaxed.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_drop_expect_tx_cluster(
    bar: *const *mut u64,
    tx: u32,
) {
    unsafe {
        let bar = *bar;
        mbarrier_arrive_drop_expect_tx_cluster::<{ Scope::Cta }, { Semantics::Release }>(bar, tx);
        mbarrier_arrive_drop_expect_tx_cluster::<{ Scope::Cluster }, { Semantics::Release }>(
            bar, tx,
        );
        mbarrier_arrive_drop_expect_tx_cluster::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, tx);
        mbarrier_arrive_drop_expect_tx_cluster::<{ Scope::Cluster }, { Semantics::Relaxed }>(
            bar, tx,
        );
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_drop_nocomplete(
// CHECK: mbarrier.arrive_drop.noComplete.b64 {{%rd[0-9]+}}, [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.noComplete.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_drop_nocomplete(
    bar: *const *mut u64,
    count: u32,
    out: *mut u64,
) {
    unsafe {
        *out = mbarrier_arrive_drop_nocomplete::<{ StateSpace::Generic }>(*bar, count);
        *out.add(1) = mbarrier_arrive_drop_nocomplete::<{ StateSpace::SharedCta }>(*bar, count);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_drop_scoped(
// CHECK: mbarrier.arrive_drop.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.release.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.relaxed.cta.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.relaxed.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_drop_scoped(
    bar: *const *mut u64,
    count: u32,
    out: *mut u64,
) {
    unsafe {
        let bar = *bar;
        *out = mbarrier_arrive_drop_scoped::<{ Scope::Cta }, { Semantics::Release }>(bar, count);
        *out.add(1) =
            mbarrier_arrive_drop_scoped::<{ Scope::Cluster }, { Semantics::Release }>(bar, count);
        *out.add(2) =
            mbarrier_arrive_drop_scoped::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, count);
        *out.add(3) =
            mbarrier_arrive_drop_scoped::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, count);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_drop_scoped_cluster(
// CHECK: mbarrier.arrive_drop.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.release.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.relaxed.cta.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive_drop.relaxed.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_drop_scoped_cluster(
    bar: *const *mut u64,
    count: u32,
) {
    unsafe {
        let bar = *bar;
        mbarrier_arrive_drop_scoped_cluster::<{ Scope::Cta }, { Semantics::Release }>(bar, count);
        mbarrier_arrive_drop_scoped_cluster::<{ Scope::Cluster }, { Semantics::Release }>(
            bar, count,
        );
        mbarrier_arrive_drop_scoped_cluster::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, count);
        mbarrier_arrive_drop_scoped_cluster::<{ Scope::Cluster }, { Semantics::Relaxed }>(
            bar, count,
        );
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_expect_tx(
// CHECK: mbarrier.arrive.expect_tx.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.expect_tx.release.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.expect_tx.relaxed.cta.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.expect_tx.relaxed.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_expect_tx(
    bar: *const *mut u64,
    tx: u32,
    out: *mut u64,
) {
    unsafe {
        let bar = *bar;
        *out = mbarrier_arrive_expect_tx::<{ Scope::Cta }, { Semantics::Release }>(bar, tx);
        *out.add(1) =
            mbarrier_arrive_expect_tx::<{ Scope::Cluster }, { Semantics::Release }>(bar, tx);
        *out.add(2) = mbarrier_arrive_expect_tx::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, tx);
        *out.add(3) =
            mbarrier_arrive_expect_tx::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, tx);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_expect_tx_cluster(
// CHECK: mbarrier.arrive.expect_tx.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.expect_tx.release.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.expect_tx.relaxed.cta.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.expect_tx.relaxed.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_expect_tx_cluster(
    bar: *const *mut u64,
    tx: u32,
) {
    unsafe {
        let bar = *bar;
        mbarrier_arrive_expect_tx_cluster::<{ Scope::Cta }, { Semantics::Release }>(bar, tx);
        mbarrier_arrive_expect_tx_cluster::<{ Scope::Cluster }, { Semantics::Release }>(bar, tx);
        mbarrier_arrive_expect_tx_cluster::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, tx);
        mbarrier_arrive_expect_tx_cluster::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, tx);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_nocomplete(
// CHECK: mbarrier.arrive.noComplete.b64 {{%rd[0-9]+}}, [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.noComplete.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_nocomplete(
    bar: *const *mut u64,
    count: u32,
    out: *mut u64,
) {
    unsafe {
        *out = mbarrier_arrive_nocomplete::<{ StateSpace::Generic }>(*bar, count);
        *out.add(1) = mbarrier_arrive_nocomplete::<{ StateSpace::SharedCta }>(*bar, count);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_scoped(
// CHECK: mbarrier.arrive.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.release.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.relaxed.cta.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.relaxed.cluster.shared.b64 {{%rd[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_scoped(
    bar: *const *mut u64,
    count: u32,
    out: *mut u64,
) {
    unsafe {
        let bar = *bar;
        *out = mbarrier_arrive_scoped::<{ Scope::Cta }, { Semantics::Release }>(bar, count);
        *out.add(1) =
            mbarrier_arrive_scoped::<{ Scope::Cluster }, { Semantics::Release }>(bar, count);
        *out.add(2) = mbarrier_arrive_scoped::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, count);
        *out.add(3) =
            mbarrier_arrive_scoped::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, count);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_arrive_scoped_cluster(
// CHECK: mbarrier.arrive.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.release.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.relaxed.cta.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.arrive.relaxed.cluster.shared::cluster.b64 _, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_arrive_scoped_cluster(
    bar: *const *mut u64,
    count: u32,
) {
    unsafe {
        let bar = *bar;
        mbarrier_arrive_scoped_cluster::<{ Scope::Cta }, { Semantics::Release }>(bar, count);
        mbarrier_arrive_scoped_cluster::<{ Scope::Cluster }, { Semantics::Release }>(bar, count);
        mbarrier_arrive_scoped_cluster::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, count);
        mbarrier_arrive_scoped_cluster::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, count);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_complete_tx(
// CHECK: mbarrier.complete_tx.relaxed.cta.shared.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.complete_tx.relaxed.cluster.shared.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.complete_tx.relaxed.cta.shared::cluster.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.complete_tx.relaxed.cluster.shared::cluster.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_complete_tx(bar: *const *mut u64, tx: u32) {
    unsafe {
        let bar = *bar;
        mbarrier_complete_tx::<{ Scope::Cta }, { StateSpace::SharedCta }>(bar, tx);
        mbarrier_complete_tx::<{ Scope::Cluster }, { StateSpace::SharedCta }>(bar, tx);
        mbarrier_complete_tx::<{ Scope::Cta }, { StateSpace::SharedCluster }>(bar, tx);
        mbarrier_complete_tx::<{ Scope::Cluster }, { StateSpace::SharedCluster }>(bar, tx);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_expect_tx(
// CHECK: mbarrier.expect_tx.relaxed.cta.shared.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.expect_tx.relaxed.cluster.shared.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.expect_tx.relaxed.cta.shared::cluster.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.expect_tx.relaxed.cluster.shared::cluster.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_expect_tx(bar: *const *mut u64, tx: u32) {
    unsafe {
        let bar = *bar;
        mbarrier_expect_tx::<{ Scope::Cta }, { StateSpace::SharedCta }>(bar, tx);
        mbarrier_expect_tx::<{ Scope::Cluster }, { StateSpace::SharedCta }>(bar, tx);
        mbarrier_expect_tx::<{ Scope::Cta }, { StateSpace::SharedCluster }>(bar, tx);
        mbarrier_expect_tx::<{ Scope::Cluster }, { StateSpace::SharedCluster }>(bar, tx);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_init(
// CHECK: mbarrier.init.b64 [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.init.shared.b64 [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_init(bar: *const *mut u64, count: u32) {
    unsafe {
        mbarrier_init::<{ StateSpace::Generic }>(*bar, count);
        mbarrier_init::<{ StateSpace::SharedCta }>(*bar, count);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_inval(
// CHECK: mbarrier.inval.b64 [{{%rd[0-9]+}}];
// CHECK: mbarrier.inval.shared.b64 [{{%r[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_inval(bar: *const *mut u64) {
    unsafe {
        mbarrier_inval::<{ StateSpace::Generic }>(*bar);
        mbarrier_inval::<{ StateSpace::SharedCta }>(*bar);
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_pending_count(
// CHECK: mbarrier.pending_count.b64 {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_pending_count(state: u64, out: *mut u32) {
    unsafe { *out = mbarrier_pending_count(state) }
}

// CHECK-LABEL: .entry kernel_mbarrier_test_wait(
// CHECK: mbarrier.test_wait.b64 {{%p[0-9]+}}, [{{%rd[0-9]+}}], {{%rd[0-9]+}};
// CHECK: mbarrier.test_wait.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_test_wait(
    bar: *const *mut u64,
    state: u64,
    out: *mut u32,
) {
    unsafe {
        *out = mbarrier_test_wait::<{ StateSpace::Generic }>(*bar, state) as u32;
        *out.add(1) = mbarrier_test_wait::<{ StateSpace::SharedCta }>(*bar, state) as u32;
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_test_wait_parity(
// CHECK: mbarrier.test_wait.parity.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.test_wait.parity.acquire.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.test_wait.parity.relaxed.cta.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.test_wait.parity.relaxed.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_test_wait_parity(
    bar: *const *mut u64,
    parity: u32,
    out: *mut u32,
) {
    unsafe {
        let bar = *bar;
        *out =
            mbarrier_test_wait_parity::<{ Scope::Cta }, { Semantics::Acquire }>(bar, parity) as u32;
        *out.add(1) =
            mbarrier_test_wait_parity::<{ Scope::Cluster }, { Semantics::Acquire }>(bar, parity)
                as u32;
        *out.add(2) =
            mbarrier_test_wait_parity::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, parity) as u32;
        *out.add(3) =
            mbarrier_test_wait_parity::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, parity)
                as u32;
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_test_wait_scoped(
// CHECK: mbarrier.test_wait.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
// CHECK: mbarrier.test_wait.acquire.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
// CHECK: mbarrier.test_wait.relaxed.cta.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
// CHECK: mbarrier.test_wait.relaxed.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_test_wait_scoped(
    bar: *const *mut u64,
    state: u64,
    out: *mut u32,
) {
    unsafe {
        let bar = *bar;
        *out =
            mbarrier_test_wait_scoped::<{ Scope::Cta }, { Semantics::Acquire }>(bar, state) as u32;
        *out.add(1) =
            mbarrier_test_wait_scoped::<{ Scope::Cluster }, { Semantics::Acquire }>(bar, state)
                as u32;
        *out.add(2) =
            mbarrier_test_wait_scoped::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, state) as u32;
        *out.add(3) =
            mbarrier_test_wait_scoped::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, state)
                as u32;
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_try_wait_parity(
// CHECK: mbarrier.try_wait.parity.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.parity.acquire.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.parity.relaxed.cta.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.parity.relaxed.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_try_wait_parity(
    bar: *const *mut u64,
    parity: u32,
    out: *mut u32,
) {
    unsafe {
        let bar = *bar;
        *out =
            mbarrier_try_wait_parity::<{ Scope::Cta }, { Semantics::Acquire }>(bar, parity) as u32;
        *out.add(1) =
            mbarrier_try_wait_parity::<{ Scope::Cluster }, { Semantics::Acquire }>(bar, parity)
                as u32;
        *out.add(2) =
            mbarrier_try_wait_parity::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, parity) as u32;
        *out.add(3) =
            mbarrier_try_wait_parity::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, parity)
                as u32;
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_try_wait_parity_tl(
// CHECK: mbarrier.try_wait.parity.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.parity.acquire.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.parity.relaxed.cta.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.parity.relaxed.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_try_wait_parity_tl(
    bar: *const *mut u64,
    parity: u32,
    hint: u32,
    out: *mut u32,
) {
    unsafe {
        let bar = *bar;
        *out = mbarrier_try_wait_parity_tl::<{ Scope::Cta }, { Semantics::Acquire }>(
            bar, parity, hint,
        ) as u32;
        *out.add(1) = mbarrier_try_wait_parity_tl::<{ Scope::Cluster }, { Semantics::Acquire }>(
            bar, parity, hint,
        ) as u32;
        *out.add(2) = mbarrier_try_wait_parity_tl::<{ Scope::Cta }, { Semantics::Relaxed }>(
            bar, parity, hint,
        ) as u32;
        *out.add(3) = mbarrier_try_wait_parity_tl::<{ Scope::Cluster }, { Semantics::Relaxed }>(
            bar, parity, hint,
        ) as u32;
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_try_wait_scoped(
// CHECK: mbarrier.try_wait.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
// CHECK: mbarrier.try_wait.acquire.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
// CHECK: mbarrier.try_wait.relaxed.cta.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
// CHECK: mbarrier.try_wait.relaxed.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_try_wait_scoped(
    bar: *const *mut u64,
    state: u64,
    out: *mut u32,
) {
    unsafe {
        let bar = *bar;
        *out =
            mbarrier_try_wait_scoped::<{ Scope::Cta }, { Semantics::Acquire }>(bar, state) as u32;
        *out.add(1) =
            mbarrier_try_wait_scoped::<{ Scope::Cluster }, { Semantics::Acquire }>(bar, state)
                as u32;
        *out.add(2) =
            mbarrier_try_wait_scoped::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, state) as u32;
        *out.add(3) =
            mbarrier_try_wait_scoped::<{ Scope::Cluster }, { Semantics::Relaxed }>(bar, state)
                as u32;
    }
}

// CHECK-LABEL: .entry kernel_mbarrier_try_wait_scoped_tl(
// CHECK: mbarrier.try_wait.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}}, {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.acquire.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}}, {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.relaxed.cta.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}}, {{%r[0-9]+}};
// CHECK: mbarrier.try_wait.relaxed.cluster.shared.b64 {{%p[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn kernel_mbarrier_try_wait_scoped_tl(
    bar: *const *mut u64,
    state: u64,
    hint: u32,
    out: *mut u32,
) {
    unsafe {
        let bar = *bar;
        *out =
            mbarrier_try_wait_scoped_tl::<{ Scope::Cta }, { Semantics::Acquire }>(bar, state, hint)
                as u32;
        *out.add(1) = mbarrier_try_wait_scoped_tl::<{ Scope::Cluster }, { Semantics::Acquire }>(
            bar, state, hint,
        ) as u32;
        *out.add(2) =
            mbarrier_try_wait_scoped_tl::<{ Scope::Cta }, { Semantics::Relaxed }>(bar, state, hint)
                as u32;
        *out.add(3) = mbarrier_try_wait_scoped_tl::<{ Scope::Cluster }, { Semantics::Relaxed }>(
            bar, state, hint,
        ) as u32;
    }
}
