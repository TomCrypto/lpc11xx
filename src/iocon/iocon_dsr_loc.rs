#[doc = "Reader of register IOCON_DSR_LOC"]
pub type R = crate::R<u32, super::IOCON_DSR_LOC>;
#[doc = "Writer for register IOCON_DSR_LOC"]
pub type W = crate::W<u32, super::IOCON_DSR_LOC>;
#[doc = "Register IOCON_DSR_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_DSR_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for DSR function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRLOC_A {
    #[doc = "0: Function DSR is available for pin PIO2_1"]
    PIO2_1,
    #[doc = "1: Function DSR is available for pin PIO3_1"]
    PIO3_1,
}
impl From<DSRLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSRLOC_A) -> Self {
        match variant {
            DSRLOC_A::PIO2_1 => 0,
            DSRLOC_A::PIO3_1 => 1,
        }
    }
}
#[doc = "Reader of field `DSRLOC`"]
pub type DSRLOC_R = crate::R<u8, DSRLOC_A>;
impl DSRLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSRLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSRLOC_A::PIO2_1),
            1 => Val(DSRLOC_A::PIO3_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_1`"]
    #[inline(always)]
    pub fn is_pio2_1(&self) -> bool {
        *self == DSRLOC_A::PIO2_1
    }
    #[doc = "Checks if the value of the field is `PIO3_1`"]
    #[inline(always)]
    pub fn is_pio3_1(&self) -> bool {
        *self == DSRLOC_A::PIO3_1
    }
}
#[doc = "Write proxy for field `DSRLOC`"]
pub struct DSRLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSRLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function DSR is available for pin PIO2_1"]
    #[inline(always)]
    pub fn pio2_1(self) -> &'a mut W {
        self.variant(DSRLOC_A::PIO2_1)
    }
    #[doc = "Function DSR is available for pin PIO3_1"]
    #[inline(always)]
    pub fn pio3_1(self) -> &'a mut W {
        self.variant(DSRLOC_A::PIO3_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DSR function."]
    #[inline(always)]
    pub fn dsrloc(&self) -> DSRLOC_R {
        DSRLOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DSR function."]
    #[inline(always)]
    pub fn dsrloc(&mut self) -> DSRLOC_W {
        DSRLOC_W { w: self }
    }
}
