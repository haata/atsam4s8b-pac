#[doc = "Reader of register CIDR"]
pub type R = crate::R<u32, super::CIDR>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u8, u8>;
#[doc = "Embedded Processor"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPROC_A {
    #[doc = "1: ARM946ES"]
    ARM946ES = 1,
    #[doc = "2: ARM7TDMI"]
    ARM7TDMI = 2,
    #[doc = "3: Cortex-M3"]
    CM3 = 3,
    #[doc = "4: ARM920T"]
    ARM920T = 4,
    #[doc = "5: ARM926EJS"]
    ARM926EJS = 5,
    #[doc = "6: Cortex-A5"]
    CA5 = 6,
    #[doc = "7: Cortex-M4"]
    CM4 = 7,
}
impl From<EPROC_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EPROC`"]
pub type EPROC_R = crate::R<u8, EPROC_A>;
impl EPROC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EPROC_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(EPROC_A::ARM946ES),
            2 => Val(EPROC_A::ARM7TDMI),
            3 => Val(EPROC_A::CM3),
            4 => Val(EPROC_A::ARM920T),
            5 => Val(EPROC_A::ARM926EJS),
            6 => Val(EPROC_A::CA5),
            7 => Val(EPROC_A::CM4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARM946ES`"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROC_A::ARM946ES
    }
    #[doc = "Checks if the value of the field is `ARM7TDMI`"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROC_A::ARM7TDMI
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == EPROC_A::CM3
    }
    #[doc = "Checks if the value of the field is `ARM920T`"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROC_A::ARM920T
    }
    #[doc = "Checks if the value of the field is `ARM926EJS`"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROC_A::ARM926EJS
    }
    #[doc = "Checks if the value of the field is `CA5`"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == EPROC_A::CA5
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == EPROC_A::CM4
    }
}
#[doc = "Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NVPSIZ_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "8: 160 Kbytes"]
    _160K = 8,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NVPSIZ`"]
pub type NVPSIZ_R = crate::R<u8, NVPSIZ_A>;
impl NVPSIZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NVPSIZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NVPSIZ_A::NONE),
            1 => Val(NVPSIZ_A::_8K),
            2 => Val(NVPSIZ_A::_16K),
            3 => Val(NVPSIZ_A::_32K),
            5 => Val(NVPSIZ_A::_64K),
            7 => Val(NVPSIZ_A::_128K),
            8 => Val(NVPSIZ_A::_160K),
            9 => Val(NVPSIZ_A::_256K),
            10 => Val(NVPSIZ_A::_512K),
            12 => Val(NVPSIZ_A::_1024K),
            14 => Val(NVPSIZ_A::_2048K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ_A::_128K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == NVPSIZ_A::_160K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ_A::_2048K
    }
}
#[doc = "Second Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NVPSIZ2_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZ2_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NVPSIZ2`"]
pub type NVPSIZ2_R = crate::R<u8, NVPSIZ2_A>;
impl NVPSIZ2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NVPSIZ2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NVPSIZ2_A::NONE),
            1 => Val(NVPSIZ2_A::_8K),
            2 => Val(NVPSIZ2_A::_16K),
            3 => Val(NVPSIZ2_A::_32K),
            5 => Val(NVPSIZ2_A::_64K),
            7 => Val(NVPSIZ2_A::_128K),
            9 => Val(NVPSIZ2_A::_256K),
            10 => Val(NVPSIZ2_A::_512K),
            12 => Val(NVPSIZ2_A::_1024K),
            14 => Val(NVPSIZ2_A::_2048K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ2_A::_2048K
    }
}
#[doc = "Internal SRAM Size"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMSIZ_A {
    #[doc = "0: 48 Kbytes"]
    _48K = 0,
    #[doc = "1: 192 Kbytes"]
    _192K = 1,
    #[doc = "2: 384 Kbytes"]
    _384K = 2,
    #[doc = "3: 6 Kbytes"]
    _6K = 3,
    #[doc = "4: 24 Kbytes"]
    _24K = 4,
    #[doc = "5: 4 Kbytes"]
    _4K = 5,
    #[doc = "6: 80 Kbytes"]
    _80K = 6,
    #[doc = "7: 160 Kbytes"]
    _160K = 7,
    #[doc = "8: 8 Kbytes"]
    _8K = 8,
    #[doc = "9: 16 Kbytes"]
    _16K = 9,
    #[doc = "10: 32 Kbytes"]
    _32K = 10,
    #[doc = "11: 64 Kbytes"]
    _64K = 11,
    #[doc = "12: 128 Kbytes"]
    _128K = 12,
    #[doc = "13: 256 Kbytes"]
    _256K = 13,
    #[doc = "14: 96 Kbytes"]
    _96K = 14,
    #[doc = "15: 512 Kbytes"]
    _512K = 15,
}
impl From<SRAMSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMSIZ`"]
pub type SRAMSIZ_R = crate::R<u8, SRAMSIZ_A>;
impl SRAMSIZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSIZ_A {
        match self.bits {
            0 => SRAMSIZ_A::_48K,
            1 => SRAMSIZ_A::_192K,
            2 => SRAMSIZ_A::_384K,
            3 => SRAMSIZ_A::_6K,
            4 => SRAMSIZ_A::_24K,
            5 => SRAMSIZ_A::_4K,
            6 => SRAMSIZ_A::_80K,
            7 => SRAMSIZ_A::_160K,
            8 => SRAMSIZ_A::_8K,
            9 => SRAMSIZ_A::_16K,
            10 => SRAMSIZ_A::_32K,
            11 => SRAMSIZ_A::_64K,
            12 => SRAMSIZ_A::_128K,
            13 => SRAMSIZ_A::_256K,
            14 => SRAMSIZ_A::_96K,
            15 => SRAMSIZ_A::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_48K`"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZ_A::_48K
    }
    #[doc = "Checks if the value of the field is `_192K`"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == SRAMSIZ_A::_192K
    }
    #[doc = "Checks if the value of the field is `_384K`"]
    #[inline(always)]
    pub fn is_384k(&self) -> bool {
        *self == SRAMSIZ_A::_384K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZ_A::_6K
    }
    #[doc = "Checks if the value of the field is `_24K`"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZ_A::_24K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZ_A::_4K
    }
    #[doc = "Checks if the value of the field is `_80K`"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZ_A::_80K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZ_A::_160K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZ_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZ_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZ_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZ_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZ_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZ_A::_256K
    }
    #[doc = "Checks if the value of the field is `_96K`"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZ_A::_96K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == SRAMSIZ_A::_512K
    }
}
#[doc = "Architecture Identifier"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARCH_A {
    #[doc = "136: SAM4SxA (48-pin version)"]
    SAM4SXA = 136,
    #[doc = "137: SAM4SxB (64-pin version)"]
    SAM4SXB = 137,
    #[doc = "138: SAM4SxC (100-pin version)"]
    SAM4SXC = 138,
}
impl From<ARCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ARCH`"]
pub type ARCH_R = crate::R<u8, ARCH_A>;
impl ARCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ARCH_A> {
        use crate::Variant::*;
        match self.bits {
            136 => Val(ARCH_A::SAM4SXA),
            137 => Val(ARCH_A::SAM4SXB),
            138 => Val(ARCH_A::SAM4SXC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAM4SXA`"]
    #[inline(always)]
    pub fn is_sam4sx_a(&self) -> bool {
        *self == ARCH_A::SAM4SXA
    }
    #[doc = "Checks if the value of the field is `SAM4SXB`"]
    #[inline(always)]
    pub fn is_sam4sx_b(&self) -> bool {
        *self == ARCH_A::SAM4SXB
    }
    #[doc = "Checks if the value of the field is `SAM4SXC`"]
    #[inline(always)]
    pub fn is_sam4sx_c(&self) -> bool {
        *self == ARCH_A::SAM4SXC
    }
}
#[doc = "Nonvolatile Program Memory Type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NVPTYP_A {
    #[doc = "0: ROM"]
    ROM = 0,
    #[doc = "1: ROMless or on-chip Flash"]
    ROMLESS = 1,
    #[doc = "2: Embedded Flash Memory"]
    FLASH = 2,
    #[doc = "3: ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    ROM_FLASH = 3,
    #[doc = "4: SRAM emulating ROM"]
    SRAM = 4,
}
impl From<NVPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NVPTYP`"]
pub type NVPTYP_R = crate::R<u8, NVPTYP_A>;
impl NVPTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NVPTYP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NVPTYP_A::ROM),
            1 => Val(NVPTYP_A::ROMLESS),
            2 => Val(NVPTYP_A::FLASH),
            3 => Val(NVPTYP_A::ROM_FLASH),
            4 => Val(NVPTYP_A::SRAM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYP_A::ROM
    }
    #[doc = "Checks if the value of the field is `ROMLESS`"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYP_A::ROMLESS
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYP_A::FLASH
    }
    #[doc = "Checks if the value of the field is `ROM_FLASH`"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYP_A::ROM_FLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYP_A::SRAM
    }
}
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NVPSIZ_R {
        NVPSIZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> NVPSIZ2_R {
        NVPSIZ2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
