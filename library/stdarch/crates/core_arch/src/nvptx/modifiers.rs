// --- LLM-generated --- //
//! Instruction modifiers shared by several instruction families.
//!
//! These are used as const generic parameters to select between variants of an instruction.

use crate::marker::ConstParamTy;

/// Floating-point rounding mode (`.rnd`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum Rounding {
    /// Round to nearest, ties to even (`.rn`).
    Rn,
    /// Round towards zero (`.rz`).
    Rz,
    /// Round towards negative infinity (`.rm`).
    Rm,
    /// Round towards positive infinity (`.rp`).
    Rp,
}

/// Scope of a memory operation (`.scope`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum Scope {
    /// The threads of the executing CTA (`.cta`).
    Cta,
    /// The threads of the executing cluster (`.cluster`).
    Cluster,
    /// The threads of the current program on the executing device (`.gpu`).
    Gpu,
    /// The threads of the current program on all devices and the host (`.sys`).
    Sys,
}

/// Memory ordering semantics (`.sem`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum Semantics {
    /// `.relaxed`
    Relaxed,
    /// `.acquire`
    Acquire,
    /// `.release`
    Release,
    /// `.acq_rel`
    AcqRel,
    /// `.sc`
    Sc,
}

/// Handling of out-of-range shift amounts and bit positions (`.mode`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum ClampMode {
    /// Clamp the value to the valid range (`.clamp`).
    Clamp,
    /// Wrap the value modulo the operand width (`.wrap`).
    Wrap,
}
