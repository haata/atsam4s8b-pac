#[doc = "Reader of register PMC_MCKR"]
pub type R = crate::R<u32, super::PMC_MCKR>;
#[doc = "Writer for register PMC_MCKR"]
pub type W = crate::W<u32, super::PMC_MCKR>;
#[doc = "Register PMC_MCKR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PMC_MCKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Master Clock Source Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSS_A {
    #[doc = "0: Slow Clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main Clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLA Clock is selected"]
    PLLA_CLK = 2,
    #[doc = "3: PLLBClock is selected"]
    PLLB_CLK = 3,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSS`"]
pub type CSS_R = crate::R<u8, CSS_A>;
impl CSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSS_A {
        match self.bits {
            0 => CSS_A::SLOW_CLK,
            1 => CSS_A::MAIN_CLK,
            2 => CSS_A::PLLA_CLK,
            3 => CSS_A::PLLB_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `PLLB_CLK`"]
    #[inline(always)]
    pub fn is_pllb_clk(&self) -> bool {
        *self == CSS_A::PLLB_CLK
    }
}
#[doc = "Write proxy for field `CSS`"]
pub struct CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSS_A::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSS_A::MAIN_CLK)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLA_CLK)
    }
    #[doc = "PLLBClock is selected"]
    #[inline(always)]
    pub fn pllb_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLB_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Processor Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRES_A {
    #[doc = "0: Selected clock"]
    CLK_1 = 0,
    #[doc = "1: Selected clock divided by 2"]
    CLK_2 = 1,
    #[doc = "2: Selected clock divided by 4"]
    CLK_4 = 2,
    #[doc = "3: Selected clock divided by 8"]
    CLK_8 = 3,
    #[doc = "4: Selected clock divided by 16"]
    CLK_16 = 4,
    #[doc = "5: Selected clock divided by 32"]
    CLK_32 = 5,
    #[doc = "6: Selected clock divided by 64"]
    CLK_64 = 6,
    #[doc = "7: Selected clock divided by 3"]
    CLK_3 = 7,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRES`"]
pub type PRES_R = crate::R<u8, PRES_A>;
impl PRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            0 => PRES_A::CLK_1,
            1 => PRES_A::CLK_2,
            2 => PRES_A::CLK_4,
            3 => PRES_A::CLK_8,
            4 => PRES_A::CLK_16,
            5 => PRES_A::CLK_32,
            6 => PRES_A::CLK_64,
            7 => PRES_A::CLK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == PRES_A::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == PRES_A::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == PRES_A::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == PRES_A::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == PRES_A::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        *self == PRES_A::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        *self == PRES_A::CLK_64
    }
    #[doc = "Checks if the value of the field is `CLK_3`"]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        *self == PRES_A::CLK_3
    }
}
#[doc = "Write proxy for field `PRES`"]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRES_A::CLK_1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRES_A::CLK_2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRES_A::CLK_4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRES_A::CLK_8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRES_A::CLK_16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRES_A::CLK_32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRES_A::CLK_64)
    }
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut W {
        self.variant(PRES_A::CLK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `PLLADIV2`"]
pub type PLLADIV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLADIV2`"]
pub struct PLLADIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLADIV2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PLLBDIV2`"]
pub type PLLBDIV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLBDIV2`"]
pub struct PLLBDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLBDIV2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 12 - PLLA Divisor by 2"]
    #[inline(always)]
    pub fn plladiv2(&self) -> PLLADIV2_R {
        PLLADIV2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PLLB Divisor by 2"]
    #[inline(always)]
    pub fn pllbdiv2(&self) -> PLLBDIV2_R {
        PLLBDIV2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W {
        CSS_W { w: self }
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    #[doc = "Bit 12 - PLLA Divisor by 2"]
    #[inline(always)]
    pub fn plladiv2(&mut self) -> PLLADIV2_W {
        PLLADIV2_W { w: self }
    }
    #[doc = "Bit 13 - PLLB Divisor by 2"]
    #[inline(always)]
    pub fn pllbdiv2(&mut self) -> PLLBDIV2_W {
        PLLBDIV2_W { w: self }
    }
}
