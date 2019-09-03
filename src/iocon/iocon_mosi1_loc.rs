#[doc = "Reader of register IOCON_MOSI1_LOC"]
pub type R = crate::R<u32, super::IOCON_MOSI1_LOC>;
#[doc = "Writer for register IOCON_MOSI1_LOC"]
pub type W = crate::W<u32, super::IOCON_MOSI1_LOC>;
#[doc = "Register IOCON_MOSI1_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_MOSI1_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for the MOSI1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSI1LOC_A {
    #[doc = "0: Function MOSI1 is available for pin PIO2_3"]
    PIO2_3,
    #[doc = "1: Function MOSI1 is available for pin PIO1_9"]
    PIO1_9,
}
impl From<MOSI1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MOSI1LOC_A) -> Self {
        match variant {
            MOSI1LOC_A::PIO2_3 => 0,
            MOSI1LOC_A::PIO1_9 => 1,
        }
    }
}
#[doc = "Reader of field `MOSI1LOC`"]
pub type MOSI1LOC_R = crate::R<u8, MOSI1LOC_A>;
impl MOSI1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOSI1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MOSI1LOC_A::PIO2_3),
            1 => Val(MOSI1LOC_A::PIO1_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_3`"]
    #[inline(always)]
    pub fn is_pio2_3(&self) -> bool {
        *self == MOSI1LOC_A::PIO2_3
    }
    #[doc = "Checks if the value of the field is `PIO1_9`"]
    #[inline(always)]
    pub fn is_pio1_9(&self) -> bool {
        *self == MOSI1LOC_A::PIO1_9
    }
}
#[doc = "Write proxy for field `MOSI1LOC`"]
pub struct MOSI1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSI1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOSI1LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function MOSI1 is available for pin PIO2_3"]
    #[inline(always)]
    pub fn pio2_3(self) -> &'a mut W {
        self.variant(MOSI1LOC_A::PIO2_3)
    }
    #[doc = "Function MOSI1 is available for pin PIO1_9"]
    #[inline(always)]
    pub fn pio1_9(self) -> &'a mut W {
        self.variant(MOSI1LOC_A::PIO1_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the MOSI1 function."]
    #[inline(always)]
    pub fn mosi1loc(&self) -> MOSI1LOC_R {
        MOSI1LOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the MOSI1 function."]
    #[inline(always)]
    pub fn mosi1loc(&mut self) -> MOSI1LOC_W {
        MOSI1LOC_W { w: self }
    }
}
