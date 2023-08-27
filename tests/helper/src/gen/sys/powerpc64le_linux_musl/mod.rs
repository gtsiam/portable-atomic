// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
mod linux_headers_linux_auxvec;
pub use linux_headers_linux_auxvec::AT_HWCAP;
pub use linux_headers_linux_auxvec::AT_HWCAP2;
mod linux_headers_asm_cputable;
pub use linux_headers_asm_cputable::PPC_FEATURE_32;
pub use linux_headers_asm_cputable::PPC_FEATURE_64;
pub use linux_headers_asm_cputable::PPC_FEATURE_601_INSTR;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_ALTIVEC;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_FPU;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_MMU;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_4xxMAC;
pub use linux_headers_asm_cputable::PPC_FEATURE_UNIFIED_CACHE;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_SPE;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_EFP_SINGLE;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_EFP_DOUBLE;
pub use linux_headers_asm_cputable::PPC_FEATURE_NO_TB;
pub use linux_headers_asm_cputable::PPC_FEATURE_POWER4;
pub use linux_headers_asm_cputable::PPC_FEATURE_POWER5;
pub use linux_headers_asm_cputable::PPC_FEATURE_POWER5_PLUS;
pub use linux_headers_asm_cputable::PPC_FEATURE_CELL;
pub use linux_headers_asm_cputable::PPC_FEATURE_BOOKE;
pub use linux_headers_asm_cputable::PPC_FEATURE_SMT;
pub use linux_headers_asm_cputable::PPC_FEATURE_ICACHE_SNOOP;
pub use linux_headers_asm_cputable::PPC_FEATURE_ARCH_2_05;
pub use linux_headers_asm_cputable::PPC_FEATURE_PA6T;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_DFP;
pub use linux_headers_asm_cputable::PPC_FEATURE_POWER6_EXT;
pub use linux_headers_asm_cputable::PPC_FEATURE_ARCH_2_06;
pub use linux_headers_asm_cputable::PPC_FEATURE_HAS_VSX;
pub use linux_headers_asm_cputable::PPC_FEATURE_PSERIES_PERFMON_COMPAT;
pub use linux_headers_asm_cputable::PPC_FEATURE_TRUE_LE;
pub use linux_headers_asm_cputable::PPC_FEATURE_PPC_LE;
pub use linux_headers_asm_cputable::PPC_FEATURE2_ARCH_2_07;
pub use linux_headers_asm_cputable::PPC_FEATURE2_HTM;
pub use linux_headers_asm_cputable::PPC_FEATURE2_DSCR;
pub use linux_headers_asm_cputable::PPC_FEATURE2_EBB;
pub use linux_headers_asm_cputable::PPC_FEATURE2_ISEL;
pub use linux_headers_asm_cputable::PPC_FEATURE2_TAR;
pub use linux_headers_asm_cputable::PPC_FEATURE2_VEC_CRYPTO;
pub use linux_headers_asm_cputable::PPC_FEATURE2_HTM_NOSC;
pub use linux_headers_asm_cputable::PPC_FEATURE2_ARCH_3_00;
pub use linux_headers_asm_cputable::PPC_FEATURE2_HAS_IEEE128;
pub use linux_headers_asm_cputable::PPC_FEATURE2_DARN;
pub use linux_headers_asm_cputable::PPC_FEATURE2_SCV;
pub use linux_headers_asm_cputable::PPC_FEATURE2_HTM_NO_SUSPEND;
pub use linux_headers_asm_cputable::PPC_FEATURE2_ARCH_3_1;
pub use linux_headers_asm_cputable::PPC_FEATURE2_MMA;
mod sys_auxv;
pub use sys_auxv::getauxval;
pub type c_char = u8;
