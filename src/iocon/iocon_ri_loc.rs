#[doc = "Reader of register IOCON_RI_LOC"]
pub type R = crate::R<u32, super::IOCON_RI_LOC>;
#[doc = "Writer for register IOCON_RI_LOC"]
pub type W = crate::W<u32, super::IOCON_RI_LOC>;
#[doc = "Register IOCON_RI_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_RI_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for RI function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RILOC_A {
    #[doc = "0: Function RI is available for pin PIO2_3"]
    PIO2_3,
    #[doc = "1: Function RI is available for pin PIO3_3"]
    PIO3_3,
}
impl From<RILOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RILOC_A) -> Self {
        match variant {
            RILOC_A::PIO2_3 => 0,
            RILOC_A::PIO3_3 => 1,
        }
    }
}
#[doc = "Reader of field `RILOC`"]
pub type RILOC_R = crate::R<u8, RILOC_A>;
impl RILOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RILOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RILOC_A::PIO2_3),
            1 => Val(RILOC_A::PIO3_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_3`"]
    #[inline(always)]
    pub fn is_pio2_3(&self) -> bool {
        *self == RILOC_A::PIO2_3
    }
    #[doc = "Checks if the value of the field is `PIO3_3`"]
    #[inline(always)]
    pub fn is_pio3_3(&self) -> bool {
        *self == RILOC_A::PIO3_3
    }
}
#[doc = "Write proxy for field `RILOC`"]
pub struct RILOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RILOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RILOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function RI is available for pin PIO2_3"]
    #[inline(always)]
    pub fn pio2_3(self) -> &'a mut W {
        self.variant(RILOC_A::PIO2_3)
    }
    #[doc = "Function RI is available for pin PIO3_3"]
    #[inline(always)]
    pub fn pio3_3(self) -> &'a mut W {
        self.variant(RILOC_A::PIO3_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for RI function."]
    #[inline(always)]
    pub fn riloc(&self) -> RILOC_R {
        RILOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for RI function."]
    #[inline(always)]
    pub fn riloc(&mut self) -> RILOC_W {
        RILOC_W { w: self }
    }
}
