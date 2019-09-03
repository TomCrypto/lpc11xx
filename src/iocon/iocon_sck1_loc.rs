#[doc = "Reader of register IOCON_SCK1_LOC"]
pub type R = crate::R<u32, super::IOCON_SCK1_LOC>;
#[doc = "Writer for register IOCON_SCK1_LOC"]
pub type W = crate::W<u32, super::IOCON_SCK1_LOC>;
#[doc = "Register IOCON_SCK1_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_SCK1_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for SCK1 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK1LOC_A {
    #[doc = "0: Function SCK1 is available for pin PIO2_1"]
    PIO2_1,
    #[doc = "1: Function SCK1 is available for pin PIO3_2"]
    PIO3_2,
}
impl From<SCK1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: SCK1LOC_A) -> Self {
        match variant {
            SCK1LOC_A::PIO2_1 => 0,
            SCK1LOC_A::PIO3_2 => 1,
        }
    }
}
#[doc = "Reader of field `SCK1LOC`"]
pub type SCK1LOC_R = crate::R<u8, SCK1LOC_A>;
impl SCK1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SCK1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SCK1LOC_A::PIO2_1),
            1 => Val(SCK1LOC_A::PIO3_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_1`"]
    #[inline(always)]
    pub fn is_pio2_1(&self) -> bool {
        *self == SCK1LOC_A::PIO2_1
    }
    #[doc = "Checks if the value of the field is `PIO3_2`"]
    #[inline(always)]
    pub fn is_pio3_2(&self) -> bool {
        *self == SCK1LOC_A::PIO3_2
    }
}
#[doc = "Write proxy for field `SCK1LOC`"]
pub struct SCK1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCK1LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function SCK1 is available for pin PIO2_1"]
    #[inline(always)]
    pub fn pio2_1(self) -> &'a mut W {
        self.variant(SCK1LOC_A::PIO2_1)
    }
    #[doc = "Function SCK1 is available for pin PIO3_2"]
    #[inline(always)]
    pub fn pio3_2(self) -> &'a mut W {
        self.variant(SCK1LOC_A::PIO3_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SCK1 function."]
    #[inline(always)]
    pub fn sck1loc(&self) -> SCK1LOC_R {
        SCK1LOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SCK1 function."]
    #[inline(always)]
    pub fn sck1loc(&mut self) -> SCK1LOC_W {
        SCK1LOC_W { w: self }
    }
}
