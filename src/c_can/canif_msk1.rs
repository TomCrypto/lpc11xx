#[doc = "Reader of register CANIF%s_MSK1"]
pub type R = crate::R<u32, super::CANIF_MSK1>;
#[doc = "Writer for register CANIF%s_MSK1"]
pub type W = crate::W<u32, super::CANIF_MSK1>;
#[doc = "Register CANIF%s_MSK1 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CANIF_MSK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Identifier mask.\n\nValue on reset: 65535"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSK_15_0_A {
    #[doc = "0: The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    NOINHIBIT,
    #[doc = "1: The corresponding identifier bit is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl From<MSK_15_0_A> for u16 {
    #[inline(always)]
    fn from(variant: MSK_15_0_A) -> Self {
        match variant {
            MSK_15_0_A::NOINHIBIT => 0,
            MSK_15_0_A::ACCEPTANCEFILTERING => 1,
        }
    }
}
#[doc = "Reader of field `MSK_15_0`"]
pub type MSK_15_0_R = crate::R<u16, MSK_15_0_A>;
impl MSK_15_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, MSK_15_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MSK_15_0_A::NOINHIBIT),
            1 => Val(MSK_15_0_A::ACCEPTANCEFILTERING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOINHIBIT`"]
    #[inline(always)]
    pub fn is_noinhibit(&self) -> bool {
        *self == MSK_15_0_A::NOINHIBIT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline(always)]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MSK_15_0_A::ACCEPTANCEFILTERING
    }
}
#[doc = "Write proxy for field `MSK_15_0`"]
pub struct MSK_15_0_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK_15_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSK_15_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    #[inline(always)]
    pub fn noinhibit(self) -> &'a mut W {
        self.variant(MSK_15_0_A::NOINHIBIT)
    }
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    #[inline(always)]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MSK_15_0_A::ACCEPTANCEFILTERING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Identifier mask."]
    #[inline(always)]
    pub fn msk_15_0(&self) -> MSK_15_0_R {
        MSK_15_0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Identifier mask."]
    #[inline(always)]
    pub fn msk_15_0(&mut self) -> MSK_15_0_W {
        MSK_15_0_W { w: self }
    }
}
