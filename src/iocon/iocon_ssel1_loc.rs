#[doc = "Reader of register IOCON_SSEL1_LOC"]
pub type R = crate::R<u32, super::IOCON_SSEL1_LOC>;
#[doc = "Writer for register IOCON_SSEL1_LOC"]
pub type W = crate::W<u32, super::IOCON_SSEL1_LOC>;
#[doc = "Register IOCON_SSEL1_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_SSEL1_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for SSEL1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEL1LOC_A {
    #[doc = "0: Function SSEL1 is available for pin PIO2_2"]
    PIO2_2,
    #[doc = "1: Function SSEL1 is available for pin PIO2_4"]
    PIO2_4,
}
impl From<SSEL1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: SSEL1LOC_A) -> Self {
        match variant {
            SSEL1LOC_A::PIO2_2 => 0,
            SSEL1LOC_A::PIO2_4 => 1,
        }
    }
}
#[doc = "Reader of field `SSEL1LOC`"]
pub type SSEL1LOC_R = crate::R<u8, SSEL1LOC_A>;
impl SSEL1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSEL1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSEL1LOC_A::PIO2_2),
            1 => Val(SSEL1LOC_A::PIO2_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_2`"]
    #[inline(always)]
    pub fn is_pio2_2(&self) -> bool {
        *self == SSEL1LOC_A::PIO2_2
    }
    #[doc = "Checks if the value of the field is `PIO2_4`"]
    #[inline(always)]
    pub fn is_pio2_4(&self) -> bool {
        *self == SSEL1LOC_A::PIO2_4
    }
}
#[doc = "Write proxy for field `SSEL1LOC`"]
pub struct SSEL1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEL1LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function SSEL1 is available for pin PIO2_2"]
    #[inline(always)]
    pub fn pio2_2(self) -> &'a mut W {
        self.variant(SSEL1LOC_A::PIO2_2)
    }
    #[doc = "Function SSEL1 is available for pin PIO2_4"]
    #[inline(always)]
    pub fn pio2_4(self) -> &'a mut W {
        self.variant(SSEL1LOC_A::PIO2_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SSEL1 function."]
    #[inline(always)]
    pub fn ssel1loc(&self) -> SSEL1LOC_R {
        SSEL1LOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SSEL1 function."]
    #[inline(always)]
    pub fn ssel1loc(&mut self) -> SSEL1LOC_W {
        SSEL1LOC_W { w: self }
    }
}
