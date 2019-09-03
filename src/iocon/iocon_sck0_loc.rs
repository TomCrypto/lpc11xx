#[doc = "Reader of register IOCON_SCK0_LOC"]
pub type R = crate::R<u32, super::IOCON_SCK0_LOC>;
#[doc = "Writer for register IOCON_SCK0_LOC"]
pub type W = crate::W<u32, super::IOCON_SCK0_LOC>;
#[doc = "Register IOCON_SCK0_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_SCK0_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for SCK0 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKLOC_A {
    #[doc = "0: Function SCK0 is available for pin PIO0_10"]
    PIO0_10,
    #[doc = "1: Function SCK0 is available for pin PIO2_11"]
    PIO2_11,
    #[doc = "2: Function SCK0 is available for pin PIO0_6"]
    PIO0_6,
}
impl From<SCKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKLOC_A) -> Self {
        match variant {
            SCKLOC_A::PIO0_10 => 0,
            SCKLOC_A::PIO2_11 => 1,
            SCKLOC_A::PIO0_6 => 2,
        }
    }
}
#[doc = "Reader of field `SCKLOC`"]
pub type SCKLOC_R = crate::R<u8, SCKLOC_A>;
impl SCKLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SCKLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SCKLOC_A::PIO0_10),
            1 => Val(SCKLOC_A::PIO2_11),
            2 => Val(SCKLOC_A::PIO0_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_10`"]
    #[inline(always)]
    pub fn is_pio0_10(&self) -> bool {
        *self == SCKLOC_A::PIO0_10
    }
    #[doc = "Checks if the value of the field is `PIO2_11`"]
    #[inline(always)]
    pub fn is_pio2_11(&self) -> bool {
        *self == SCKLOC_A::PIO2_11
    }
    #[doc = "Checks if the value of the field is `PIO0_6`"]
    #[inline(always)]
    pub fn is_pio0_6(&self) -> bool {
        *self == SCKLOC_A::PIO0_6
    }
}
#[doc = "Write proxy for field `SCKLOC`"]
pub struct SCKLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCKLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function SCK0 is available for pin PIO0_10"]
    #[inline(always)]
    pub fn pio0_10(self) -> &'a mut W {
        self.variant(SCKLOC_A::PIO0_10)
    }
    #[doc = "Function SCK0 is available for pin PIO2_11"]
    #[inline(always)]
    pub fn pio2_11(self) -> &'a mut W {
        self.variant(SCKLOC_A::PIO2_11)
    }
    #[doc = "Function SCK0 is available for pin PIO0_6"]
    #[inline(always)]
    pub fn pio0_6(self) -> &'a mut W {
        self.variant(SCKLOC_A::PIO0_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 function."]
    #[inline(always)]
    pub fn sckloc(&self) -> SCKLOC_R {
        SCKLOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 function."]
    #[inline(always)]
    pub fn sckloc(&mut self) -> SCKLOC_W {
        SCKLOC_W { w: self }
    }
}
