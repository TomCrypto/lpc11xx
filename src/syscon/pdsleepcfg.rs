#[doc = "Reader of register PDSLEEPCFG"]
pub type R = crate::R<u32, super::PDSLEEPCFG>;
#[doc = "Writer for register PDSLEEPCFG"]
pub type W = crate::W<u32, super::PDSLEEPCFG>;
#[doc = "Register PDSLEEPCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PDSLEEPCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NOTUSED0`"]
pub type NOTUSED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOTUSED0`"]
pub struct NOTUSED0_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTUSED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "BOD power-down control in Deep-sleep mode, see Table 40.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PD_A {
    #[doc = "0: Powered"]
    POWERED,
    #[doc = "1: Powered down"]
    POWERED_DOWN,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        match variant {
            BOD_PD_A::POWERED => false,
            BOD_PD_A::POWERED_DOWN => true,
        }
    }
}
#[doc = "Reader of field `BOD_PD`"]
pub type BOD_PD_R = crate::R<bool, BOD_PD_A>;
impl BOD_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `BOD_PD`"]
pub struct BOD_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `NOTUSED1`"]
pub type NOTUSED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOTUSED1`"]
pub struct NOTUSED1_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTUSED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Watchdog oscillator power control in Deep-sleep mode, see Table 40.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Powered"]
    POWERED,
    #[doc = "1: Powered down"]
    POWERED_DOWN,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        match variant {
            WDTOSC_PD_A::POWERED => false,
            WDTOSC_PD_A::POWERED_DOWN => true,
        }
    }
}
#[doc = "Reader of field `WDTOSC_PD`"]
pub type WDTOSC_PD_R = crate::R<bool, WDTOSC_PD_A>;
impl WDTOSC_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::POWERED,
            true => WDTOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `WDTOSC_PD`"]
pub struct WDTOSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTOSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTOSC_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED_DOWN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `NOTUSED2`"]
pub type NOTUSED2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOTUSED2`"]
pub struct NOTUSED2_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTUSED2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `NOTUSED3`"]
pub type NOTUSED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOTUSED3`"]
pub struct NOTUSED3_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTUSED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `NOTUSED4`"]
pub type NOTUSED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOTUSED4`"]
pub struct NOTUSED4_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTUSED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn notused0(&self) -> NOTUSED0_R {
        NOTUSED0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn notused1(&self) -> NOTUSED1_R {
        NOTUSED1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn notused2(&self) -> NOTUSED2_R {
        NOTUSED2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reserved. Always write these bits as 000."]
    #[inline(always)]
    pub fn notused3(&self) -> NOTUSED3_R {
        NOTUSED3_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn notused4(&self) -> NOTUSED4_R {
        NOTUSED4_R::new(((self.bits >> 11) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn notused0(&mut self) -> NOTUSED0_W {
        NOTUSED0_W { w: self }
    }
    #[doc = "Bit 3 - BOD power-down control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W {
        BOD_PD_W { w: self }
    }
    #[doc = "Bits 4:5 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn notused1(&mut self) -> NOTUSED1_W {
        NOTUSED1_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator power control in Deep-sleep mode, see Table 40."]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W {
        WDTOSC_PD_W { w: self }
    }
    #[doc = "Bit 7 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn notused2(&mut self) -> NOTUSED2_W {
        NOTUSED2_W { w: self }
    }
    #[doc = "Bits 8:10 - Reserved. Always write these bits as 000."]
    #[inline(always)]
    pub fn notused3(&mut self) -> NOTUSED3_W {
        NOTUSED3_W { w: self }
    }
    #[doc = "Bits 11:12 - Reserved. Always write these bits as 11."]
    #[inline(always)]
    pub fn notused4(&mut self) -> NOTUSED4_W {
        NOTUSED4_W { w: self }
    }
}
