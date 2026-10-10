// --- LLM-generated --- //
// Checks that generic pointers are autocast to/from the address spaces expected by LLVM intrinsics.

//@ add-minicore
//@ compile-flags: --crate-type=rlib --target=nvptx64-nvidia-cuda -Copt-level=0
//@ needs-llvm-components: nvptx
#![feature(no_core, lang_items, link_llvm_intrinsics, rustc_attrs)]
#![no_core]

extern crate minicore;
use minicore::*;

unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.4"]
    fn cp_async_ca_shared_global_4(dst: *mut u8, src: *const u8);
    #[link_name = "llvm.nvvm.mapa.shared.cluster"]
    fn mapa_shared_cluster(p: *mut u8, rank: u32) -> *mut u8;
    #[link_name = "llvm.nvvm.mapa.shared.cluster"]
    #[rustc_llvm_ptr_addrspace(args(3, 0), ret(7))]
    fn mapa_shared_cluster_redundant(p: *mut u8, rank: u32) -> *mut u8;
    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0))]
    fn ldu_global_i32(p: *const i32, align: i32) -> i32;
    #[link_name = "llvm.nvvm.ldu.global.p.p3.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0), ret(3))]
    fn ldu_global_shared_ptr(p: *const *mut u8, align: i32) -> *mut u8;
    #[link_name = "llvm.nvvm.ldu.global.p.p3.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0), ret(3))]
    fn ldu_global_shared_int(p: *const u32, align: i32) -> u32;
    #[link_name = "llvm.nvvm.mapa.shared.cluster"]
    #[rustc_llvm_ptr_addrspace(args(3, 0), ret(7))]
    fn mapa_shared_cluster_int(p: u32, rank: u32) -> u32;
}

// CHECK-LABEL: @non_overloaded_args
#[no_mangle]
pub unsafe fn non_overloaded_args(dst: *mut u8, src: *const u8) {
    // CHECK: [[DST:%[0-9]+]] = addrspacecast ptr %dst to ptr addrspace(3)
    // CHECK: [[SRC:%[0-9]+]] = addrspacecast ptr %src to ptr addrspace(1)
    // CHECK: call void @llvm.nvvm.cp.async.ca.shared.global.4(ptr addrspace(3) [[DST]], ptr addrspace(1) [[SRC]])
    cp_async_ca_shared_global_4(dst, src)
}

// CHECK-LABEL: @non_overloaded_ret
#[no_mangle]
pub unsafe fn non_overloaded_ret(p: *mut u8, rank: u32) -> *mut u8 {
    // CHECK: [[P:%[0-9]+]] = addrspacecast ptr %p to ptr addrspace(3)
    // CHECK: [[R:%[0-9]+]] = call ptr addrspace(7) @llvm.nvvm.mapa.shared.cluster(ptr addrspace(3) [[P]], i32 %rank)
    // CHECK: addrspacecast ptr addrspace(7) [[R]] to ptr
    mapa_shared_cluster(p, rank)
}

// CHECK-LABEL: @non_overloaded_redundant_attr
#[no_mangle]
pub unsafe fn non_overloaded_redundant_attr(p: *mut u8, rank: u32) -> *mut u8 {
    // CHECK: [[P:%[0-9]+]] = addrspacecast ptr %p to ptr addrspace(3)
    // CHECK: [[R:%[0-9]+]] = call ptr addrspace(7) @llvm.nvvm.mapa.shared.cluster(ptr addrspace(3) [[P]], i32 %rank)
    // CHECK: addrspacecast ptr addrspace(7) [[R]] to ptr
    mapa_shared_cluster_redundant(p, rank)
}

// CHECK-LABEL: @overloaded_args
#[no_mangle]
pub unsafe fn overloaded_args(p: *const i32) -> i32 {
    // CHECK: [[P:%[0-9]+]] = addrspacecast ptr %p to ptr addrspace(1)
    // CHECK: call i32 @llvm.nvvm.ldu.global.i.i32.p1(ptr addrspace(1) [[P]], i32 4)
    ldu_global_i32(p, 4)
}

// CHECK-LABEL: @overloaded_ret
#[no_mangle]
pub unsafe fn overloaded_ret(p: *const *mut u8) -> *mut u8 {
    // CHECK: [[P:%[0-9]+]] = addrspacecast ptr %p to ptr addrspace(1)
    // CHECK: [[R:%[0-9]+]] = call ptr addrspace(3) @llvm.nvvm.ldu.global.p.p3.p1(ptr addrspace(1) [[P]], i32 8)
    // CHECK: addrspacecast ptr addrspace(3) [[R]] to ptr
    ldu_global_shared_ptr(p, 8)
}

// CHECK-LABEL: @non_overloaded_int
#[no_mangle]
pub unsafe fn non_overloaded_int(p: u32, rank: u32) -> u32 {
    // CHECK: [[P:%[0-9]+]] = inttoptr i32 %p to ptr addrspace(3)
    // CHECK: [[R:%[0-9]+]] = call ptr addrspace(7) @llvm.nvvm.mapa.shared.cluster(ptr addrspace(3) [[P]], i32 %rank)
    // CHECK: ptrtoint ptr addrspace(7) [[R]] to i32
    mapa_shared_cluster_int(p, rank)
}

// CHECK-LABEL: @overloaded_int_ret
#[no_mangle]
pub unsafe fn overloaded_int_ret(p: *const u32) -> u32 {
    // CHECK: [[P:%[0-9]+]] = addrspacecast ptr %p to ptr addrspace(1)
    // CHECK: [[R:%[0-9]+]] = call ptr addrspace(3) @llvm.nvvm.ldu.global.p.p3.p1(ptr addrspace(1) [[P]], i32 4)
    // CHECK: ptrtoint ptr addrspace(3) [[R]] to i32
    ldu_global_shared_int(p, 4)
}

// CHECK: declare void @llvm.nvvm.cp.async.ca.shared.global.4(ptr addrspace(3){{.*}}, ptr addrspace(1){{.*}})
// CHECK: declare ptr addrspace(7) @llvm.nvvm.mapa.shared.cluster(ptr addrspace(3){{.*}}, i32)
// CHECK: declare i32 @llvm.nvvm.ldu.global.i.i32.p1(ptr addrspace(1){{.*}}, i32)
// CHECK: declare ptr addrspace(3) @llvm.nvvm.ldu.global.p.p3.p1(ptr addrspace(1){{.*}}, i32)
