// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aarch64_sysctl_cpu_id {
    pub ac_midr: u64,
    pub ac_revidr: u64,
    pub ac_mpidr: u64,
    pub ac_aa64dfr0: u64,
    pub ac_aa64dfr1: u64,
    pub ac_aa64isar0: u64,
    pub ac_aa64isar1: u64,
    pub ac_aa64mmfr0: u64,
    pub ac_aa64mmfr1: u64,
    pub ac_aa64mmfr2: u64,
    pub ac_aa64pfr0: u64,
    pub ac_aa64pfr1: u64,
    pub ac_aa64zfr0: u64,
    pub ac_mvfr0: u32,
    pub ac_mvfr1: u32,
    pub ac_mvfr2: u32,
    pub ac_pad: u32,
    pub ac_clidr: u64,
    pub ac_ctr: u64,
}
