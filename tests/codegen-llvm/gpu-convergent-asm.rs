// --- LLM-generated --- //
// Checks that when compiling for GPU targets, the convergent attribute
// is added to inline assembly call sites.

//@ add-minicore
//@ revisions: amdgpu nvptx
//@ [amdgpu] compile-flags: --crate-type=rlib --target=amdgcn-amd-amdhsa -Ctarget-cpu=gfx900
//@ [amdgpu] needs-llvm-components: amdgpu
//@ [nvptx] compile-flags: --crate-type=rlib --target=nvptx64-nvidia-cuda
//@ [nvptx] needs-llvm-components: nvptx
#![feature(no_core, asm_experimental_arch)]
#![no_core]

extern crate minicore;
use minicore::*;

// CHECK-LABEL: @asm_volatile
// CHECK: call void asm sideeffect {{.*}}"", "{{.*}}"() #[[VOLATILE:[0-9]+]]
#[no_mangle]
pub fn asm_volatile() {
    unsafe {
        asm!("");
    }
}

// CHECK-LABEL: @asm_nomem
// CHECK: call void asm sideeffect {{.*}}"", "{{.*}}"() #[[NOMEM:[0-9]+]]
#[no_mangle]
pub fn asm_nomem() {
    unsafe {
        asm!("", options(nomem, nostack));
    }
}

// CHECK-DAG: attributes #[[VOLATILE]] = {{.*}}convergent
// CHECK-DAG: attributes #[[NOMEM]] = {{.*}}convergent
