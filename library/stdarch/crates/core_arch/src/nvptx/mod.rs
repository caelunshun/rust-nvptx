//! NVPTX intrinsics (experimental)
//!
//! These intrinsics form the foundation of the CUDA
//! programming model.
//!
//! The reference is the [CUDA C Programming Guide][cuda_c]. Relevant is also
//! the [LLVM NVPTX Backend documentation][llvm_docs].
//!
//! [cuda_c]:
//! http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html
//! [llvm_docs]:
//! https://llvm.org/docs/NVPTXUsage.html

use crate::ffi::c_void;

mod float;
mod int;
mod mem;
mod minmax;
mod misc;
mod modifiers;
mod packed;
mod sreg;
mod sync;
mod warp;

#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use float::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use int::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use mem::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use minmax::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use misc::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use modifiers::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use packed::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use sreg::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use sync::*;
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub use warp::*;

/// Generates the trap instruction `TRAP`
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn trap() -> ! {
    crate::intrinsics::abort_immediate()
}

// Basic CUDA syscall declarations.
unsafe extern "C" {
    /// Print formatted output from a kernel to a host-side output stream.
    ///
    /// Syscall arguments:
    /// * `status`: The status value that is returned by `vprintf`.
    /// * `format`: A pointer to the format specifier input (uses common `printf` format).
    /// * `valist`: A pointer to the valist input.
    ///
    /// ```ignore (available only for nvptx)
    /// # use std::mem::transmute;
    /// #[repr(C)]
    /// struct PrintArgs(f32, f32, f32, i32);
    ///
    /// let a = 0.1f32;
    /// let b = 0.2f32;
    /// vprintf(
    ///     "int(%f + %f) = int(%f) = %d\n".as_ptr(),
    ///     transmute(&PrintArgs(a, b, a + b, (a + b) as i32)),
    /// );
    /// ```
    ///
    /// Sources:
    /// [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#formatted-output),
    /// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
    #[unstable(feature = "stdarch_nvptx", issue = "111199")]
    pub fn vprintf(format: *const u8, valist: *const c_void) -> i32;

    /// Allocate memory dynamically from a fixed-size heap in global memory.
    ///
    /// The CUDA in-kernel `malloc()` function allocates at least `size` bytes
    /// from the device heap and returns a pointer to the allocated memory
    /// or `NULL` if insufficient memory exists to fulfill the request.
    ///
    /// The returned pointer is guaranteed to be aligned to a 16-byte boundary.
    ///
    /// The memory allocated by a given CUDA thread via `malloc()` remains allocated
    /// for the lifetime of the CUDA context, or until it is explicitly released
    /// by a call to `free()`. It can be used by any other CUDA threads
    /// even from subsequent kernel launches.
    ///
    /// Sources:
    /// [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#dynamic-global-memory-allocation-and-operations),
    /// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
    // FIXME(denzp): assign `malloc` and `nothrow` attributes.
    #[unstable(feature = "stdarch_nvptx", issue = "111199")]
    pub fn malloc(size: usize) -> *mut c_void;

    /// Free previously dynamically allocated memory.
    ///
    /// The CUDA in-kernel `free()` function deallocates the memory pointed to by `ptr`,
    /// which must have been returned by a previous call to `malloc()`. If `ptr` is NULL,
    /// the call to `free()` is ignored.
    ///
    /// Any CUDA thread may free memory allocated by another thread, but care should be taken
    /// to ensure that the same pointer is not freed more than once. Repeated calls to `free()`
    /// with the same `ptr` has undefined behavior.
    ///
    /// Sources:
    /// [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#dynamic-global-memory-allocation-and-operations),
    /// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
    // FIXME(denzp): assign `nothrow` attribute.
    #[unstable(feature = "stdarch_nvptx", issue = "111199")]
    pub fn free(ptr: *mut c_void);

    // Internal declaration of the syscall. Exported variant has
    // the `char_size` parameter set to `1` (single char size in bytes).
    fn __assertfail(
        message: *const u8,
        file: *const u8,
        line: u32,
        function: *const u8,
        char_size: usize,
    );
}

/// Syscall to be used whenever the *assert expression produces a `false` value*.
///
/// Syscall arguments:
/// * `message`: The pointer to the string that should be output.
/// * `file`: The pointer to the file name string associated with the assert.
/// * `line`: The line number associated with the assert.
/// * `function`: The pointer to the function name string associated with the assert.
///
/// Source:
/// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn __assert_fail(message: *const u8, file: *const u8, line: u32, function: *const u8) {
    __assertfail(message, file, line, function, 1)
}
