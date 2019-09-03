#[doc = "Reader of register IOCON_CT32B0_CAP0_LOC"]
pub type R = crate::R<u32, super::IOCON_CT32B0_CAP0_LOC>;
#[doc = "Writer for register IOCON_CT32B0_CAP0_LOC"]
pub type W = crate::W<u32, super::IOCON_CT32B0_CAP0_LOC>;
#[doc = "Register IOCON_CT32B0_CAP0_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_CT32B0_CAP0_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for the CT32B0_CAP0 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0_CAP0LOC_A {
    #[doc = "0: Function CT32B0_CAP0 is available for pin PIO1_5"]
    PIO1_5,
    #[doc = "1: Function CT32B0_CAP0 is available for pin PIO2_9"]
    PIO2_9,
}
impl From<CT32B0_CAP0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CT32B0_CAP0LOC_A) -> Self {
        match variant {
            CT32B0_CAP0LOC_A::PIO1_5 => 0,
            CT32B0_CAP0LOC_A::PIO2_9 => 1,
        }
    }
}
#[doc = "Reader of field `CT32B0_CAP0LOC`"]
pub type CT32B0_CAP0LOC_R = crate::R<u8, CT32B0_CAP0LOC_A>;
impl CT32B0_CAP0LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CT32B0_CAP0LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CT32B0_CAP0LOC_A::PIO1_5),
            1 => Val(CT32B0_CAP0LOC_A::PIO2_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO1_5`"]
    #[inline(always)]
    pub fn is_pio1_5(&self) -> bool {
        *self == CT32B0_CAP0LOC_A::PIO1_5
    }
    #[doc = "Checks if the value of the field is `PIO2_9`"]
    #[inline(always)]
    pub fn is_pio2_9(&self) -> bool {
        *self == CT32B0_CAP0LOC_A::PIO2_9
    }
}
#[doc = "Write proxy for field `CT32B0_CAP0LOC`"]
pub struct CT32B0_CAP0LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CT32B0_CAP0LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT32B0_CAP0LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function CT32B0_CAP0 is available for pin PIO1_5"]
    #[inline(always)]
    pub fn pio1_5(self) -> &'a mut W {
        self.variant(CT32B0_CAP0LOC_A::PIO1_5)
    }
    #[doc = "Function CT32B0_CAP0 is available for pin PIO2_9"]
    #[inline(always)]
    pub fn pio2_9(self) -> &'a mut W {
        self.variant(CT32B0_CAP0LOC_A::PIO2_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for the CT32B0_CAP0 function."]
    #[inline(always)]
    pub fn ct32b0_cap0loc(&self) -> CT32B0_CAP0LOC_R {
        CT32B0_CAP0LOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for the CT32B0_CAP0 function."]
    #[inline(always)]
    pub fn ct32b0_cap0loc(&mut self) -> CT32B0_CAP0LOC_W {
        CT32B0_CAP0LOC_W { w: self }
    }
}
