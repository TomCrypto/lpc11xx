#[doc = "Reader of register IOCON_CT16B0_CAP0_LOC"]
pub type R = crate::R<u32, super::IOCON_CT16B0_CAP0_LOC>;
#[doc = "Writer for register IOCON_CT16B0_CAP0_LOC"]
pub type W = crate::W<u32, super::IOCON_CT16B0_CAP0_LOC>;
#[doc = "Register IOCON_CT16B0_CAP0_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCON_CT16B0_CAP0_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for CT16B0_CAP0 function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0_CAP0LOC_A {
    #[doc = "0: Function CT16B0_CAP0 is available for pin PIO0_2"]
    PIO0_2,
    #[doc = "1: Function CT16B0_CAP0 is available for pin PIO3_3"]
    PIO3_3,
}
impl From<CT16B0_CAP0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CT16B0_CAP0LOC_A) -> Self {
        match variant {
            CT16B0_CAP0LOC_A::PIO0_2 => 0,
            CT16B0_CAP0LOC_A::PIO3_3 => 1,
        }
    }
}
#[doc = "Reader of field `CT16B0_CAP0LOC`"]
pub type CT16B0_CAP0LOC_R = crate::R<u8, CT16B0_CAP0LOC_A>;
impl CT16B0_CAP0LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CT16B0_CAP0LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CT16B0_CAP0LOC_A::PIO0_2),
            1 => Val(CT16B0_CAP0LOC_A::PIO3_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_2`"]
    #[inline(always)]
    pub fn is_pio0_2(&self) -> bool {
        *self == CT16B0_CAP0LOC_A::PIO0_2
    }
    #[doc = "Checks if the value of the field is `PIO3_3`"]
    #[inline(always)]
    pub fn is_pio3_3(&self) -> bool {
        *self == CT16B0_CAP0LOC_A::PIO3_3
    }
}
#[doc = "Write proxy for field `CT16B0_CAP0LOC`"]
pub struct CT16B0_CAP0LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CT16B0_CAP0LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT16B0_CAP0LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function CT16B0_CAP0 is available for pin PIO0_2"]
    #[inline(always)]
    pub fn pio0_2(self) -> &'a mut W {
        self.variant(CT16B0_CAP0LOC_A::PIO0_2)
    }
    #[doc = "Function CT16B0_CAP0 is available for pin PIO3_3"]
    #[inline(always)]
    pub fn pio3_3(self) -> &'a mut W {
        self.variant(CT16B0_CAP0LOC_A::PIO3_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for CT16B0_CAP0 function."]
    #[inline(always)]
    pub fn ct16b0_cap0loc(&self) -> CT16B0_CAP0LOC_R {
        CT16B0_CAP0LOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for CT16B0_CAP0 function."]
    #[inline(always)]
    pub fn ct16b0_cap0loc(&mut self) -> CT16B0_CAP0LOC_W {
        CT16B0_CAP0LOC_W { w: self }
    }
}
