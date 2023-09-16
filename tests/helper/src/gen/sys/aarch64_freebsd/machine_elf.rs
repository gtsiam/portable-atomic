// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Auxinfo {
    pub a_type: ::std::os::raw::c_long,
    pub a_un: Elf64_Auxinfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Elf64_Auxinfo__bindgen_ty_1 {
    pub a_val: ::std::os::raw::c_long,
    pub a_ptr: *mut ::core::ffi::c_void,
    pub a_fcn: ::core::option::Option<unsafe extern "C" fn()>,
}
pub const HWCAP_FP: u32 = 1;
pub const HWCAP_ASIMD: u32 = 2;
pub const HWCAP_EVTSTRM: u32 = 4;
pub const HWCAP_AES: u32 = 8;
pub const HWCAP_PMULL: u32 = 16;
pub const HWCAP_SHA1: u32 = 32;
pub const HWCAP_SHA2: u32 = 64;
pub const HWCAP_CRC32: u32 = 128;
pub const HWCAP_ATOMICS: u32 = 256;
pub const HWCAP_FPHP: u32 = 512;
pub const HWCAP_ASIMDHP: u32 = 1024;
pub const HWCAP_CPUID: u32 = 2048;
pub const HWCAP_ASIMDRDM: u32 = 4096;
pub const HWCAP_JSCVT: u32 = 8192;
pub const HWCAP_FCMA: u32 = 16384;
pub const HWCAP_LRCPC: u32 = 32768;
pub const HWCAP_DCPOP: u32 = 65536;
pub const HWCAP_SHA3: u32 = 131072;
pub const HWCAP_SM3: u32 = 262144;
pub const HWCAP_SM4: u32 = 524288;
pub const HWCAP_ASIMDDP: u32 = 1048576;
pub const HWCAP_SHA512: u32 = 2097152;
pub const HWCAP_SVE: u32 = 4194304;
pub const HWCAP_ASIMDFHM: u32 = 8388608;
pub const HWCAP_DIT: u32 = 16777216;
pub const HWCAP_USCAT: u32 = 33554432;
pub const HWCAP_ILRCPC: u32 = 67108864;
pub const HWCAP_FLAGM: u32 = 134217728;
pub const HWCAP_SSBS: u32 = 268435456;
pub const HWCAP_SB: u32 = 536870912;
pub const HWCAP_PACA: u32 = 1073741824;
pub const HWCAP_PACG: u32 = 2147483648;
pub const HWCAP2_DCPODP: u32 = 1;
pub const HWCAP2_SVE2: u32 = 2;
pub const HWCAP2_SVEAES: u32 = 4;
pub const HWCAP2_SVEPMULL: u32 = 8;
pub const HWCAP2_SVEBITPERM: u32 = 16;
pub const HWCAP2_SVESHA3: u32 = 32;
pub const HWCAP2_SVESM4: u32 = 64;
pub const HWCAP2_FLAGM2: u32 = 128;
pub const HWCAP2_FRINT: u32 = 256;
pub const HWCAP2_SVEI8MM: u32 = 512;
pub const HWCAP2_SVEF32MM: u32 = 1024;
pub const HWCAP2_SVEF64MM: u32 = 2048;
pub const HWCAP2_SVEBF16: u32 = 4096;
pub const HWCAP2_I8MM: u32 = 8192;
pub const HWCAP2_BF16: u32 = 16384;
pub const HWCAP2_DGH: u32 = 32768;
pub const HWCAP2_RNG: u32 = 65536;
pub const HWCAP2_BTI: u32 = 131072;
