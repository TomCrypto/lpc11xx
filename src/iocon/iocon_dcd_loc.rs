#[doc = "Reader of register IOCON_DCD_LOC"]
pub type R = crate::R<u32, super::IOCON_DCD_LOC>;
#[doc = "Writer for register IOCON_DCD_LOC"]
pub type W = crate::W<u32, super::IOCON_DCD_LOC>;
#[doc = "Register IOCON_DCD_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_DCD_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for DCD function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDLOC_A {
    #[doc = "0: Function DCD is available for pin PIO2_2"]
    PIO2_2,
    #[doc = "1: Function DCD is available for pin PIO3_2"]
    PIO3_2,
}
impl From<DCDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDLOC_A) -> Self {
        match variant {
            DCDLOC_A::PIO2_2 => 0,
            DCDLOC_A::PIO3_2 => 1,
        }
    }
}
#[doc = "Reader of field `DCDLOC`"]
pub type DCDLOC_R = crate::R<u8, DCDLOC_A>;
impl DCDLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCDLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DCDLOC_A::PIO2_2),
            1 => Val(DCDLOC_A::PIO3_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_2`"]
    #[inline(always)]
    pub fn is_pio2_2(&self) -> bool {
        *self == DCDLOC_A::PIO2_2
    }
    #[doc = "Checks if the value of the field is `PIO3_2`"]
    #[inline(always)]
    pub fn is_pio3_2(&self) -> bool {
        *self == DCDLOC_A::PIO3_2
    }
}
#[doc = "Write proxy for field `DCDLOC`"]
pub struct DCDLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function DCD is available for pin PIO2_2"]
    #[inline(always)]
    pub fn pio2_2(self) -> &'a mut W {
        self.variant(DCDLOC_A::PIO2_2)
    }
    #[doc = "Function DCD is available for pin PIO3_2"]
    #[inline(always)]
    pub fn pio3_2(self) -> &'a mut W {
        self.variant(DCDLOC_A::PIO3_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DCD function."]
    #[inline(always)]
    pub fn dcdloc(&self) -> DCDLOC_R {
        DCDLOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DCD function."]
    #[inline(always)]
    pub fn dcdloc(&mut self) -> DCDLOC_W {
        DCDLOC_W { w: self }
    }
}
