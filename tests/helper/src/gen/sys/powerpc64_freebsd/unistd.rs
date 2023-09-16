// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

pub type __int32_t = ::std::os::raw::c_int;
pub type __pid_t = __int32_t;
pub type pid_t = __pid_t;
extern "C" {
    pub fn getpid() -> pid_t;
}
