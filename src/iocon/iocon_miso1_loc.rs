#[doc = "Reader of register IOCON_MISO1_LOC"]
pub type R = crate::R<u32, super::IOCON_MISO1_LOC>;
#[doc = "Writer for register IOCON_MISO1_LOC"]
pub type W = crate::W<u32, super::IOCON_MISO1_LOC>;
#[doc = "Register IOCON_MISO1_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_MISO1_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for the MISO1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISO1LOC_A {
    #[doc = "0: Function MISO1 is available for pin PIO2_2"]
    PIO2_2,
    #[doc = "1: Function MISO1 is available for pin PIO1_10"]
    PIO1_10,
}
impl From<MISO1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MISO1LOC_A) -> Self {
        match variant {
            MISO1LOC_A::PIO2_2 => 0,
            MISO1LOC_A::PIO1_10 => 1,
        }
    }
}
#[doc = "Reader of field `MISO1LOC`"]
pub type MISO1LOC_R = crate::R<u8, MISO1LOC_A>;
impl MISO1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MISO1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MISO1LOC_A::PIO2_2),
            1 => Val(MISO1LOC_A::PIO1_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_2`"]
    #[inline(always)]
    pub fn is_pio2_2(&self) -> bool {
        *self == MISO1LOC_A::PIO2_2
    }
    #[doc = "Checks if the value of the field is `PIO1_10`"]
    #[inline(always)]
    pub fn is_pio1_10(&self) -> bool {
        *self == MISO1LOC_A::PIO1_10
    }
}
#[doc = "Write proxy for field `MISO1LOC`"]
pub struct MISO1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MISO1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MISO1LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function MISO1 is available for pin PIO2_2"]
    #[inline(always)]
    pub fn pio2_2(self) -> &'a mut W {
        self.variant(MISO1LOC_A::PIO2_2)
    }
    #[doc = "Function MISO1 is available for pin PIO1_10"]
    #[inline(always)]
    pub fn pio1_10(self) -> &'a mut W {
        self.variant(MISO1LOC_A::PIO1_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the MISO1 function."]
    #[inline(always)]
    pub fn miso1loc(&self) -> MISO1LOC_R {
        MISO1LOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the MISO1 function."]
    #[inline(always)]
    pub fn miso1loc(&mut self) -> MISO1LOC_W {
        MISO1LOC_W { w: self }
    }
}
