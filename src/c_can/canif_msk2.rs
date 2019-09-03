#[doc = "Reader of register CANIF%s_MSK2"]
pub type R = crate::R<u32, super::CANIF_MSK2>;
#[doc = "Writer for register CANIF%s_MSK2"]
pub type W = crate::W<u32, super::CANIF_MSK2>;
#[doc = "Register CANIF%s_MSK2 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CANIF_MSK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Identifier mask.\n\nValue on reset: 8191"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSK_28_16_A {
    #[doc = "0: The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    NOINHIBIT,
    #[doc = "1: The corresponding identifier bit is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl From<MSK_28_16_A> for u16 {
    #[inline(always)]
    fn from(variant: MSK_28_16_A) -> Self {
        match variant {
            MSK_28_16_A::NOINHIBIT => 0,
            MSK_28_16_A::ACCEPTANCEFILTERING => 1,
        }
    }
}
#[doc = "Reader of field `MSK_28_16`"]
pub type MSK_28_16_R = crate::R<u16, MSK_28_16_A>;
impl MSK_28_16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, MSK_28_16_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MSK_28_16_A::NOINHIBIT),
            1 => Val(MSK_28_16_A::ACCEPTANCEFILTERING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOINHIBIT`"]
    #[inline(always)]
    pub fn is_noinhibit(&self) -> bool {
        *self == MSK_28_16_A::NOINHIBIT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline(always)]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MSK_28_16_A::ACCEPTANCEFILTERING
    }
}
#[doc = "Write proxy for field `MSK_28_16`"]
pub struct MSK_28_16_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK_28_16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSK_28_16_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding bit in the identifier of the message can not inhibit the match in the acceptance filtering"]
    #[inline(always)]
    pub fn noinhibit(self) -> &'a mut W {
        self.variant(MSK_28_16_A::NOINHIBIT)
    }
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    #[inline(always)]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MSK_28_16_A::ACCEPTANCEFILTERING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Mask message direction.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIR_A {
    #[doc = "0: The message direction bit (DIR) has no effect on acceptance filtering"]
    NOEFFECT,
    #[doc = "1: The message direction bit (DIR) is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl From<MDIR_A> for bool {
    #[inline(always)]
    fn from(variant: MDIR_A) -> Self {
        match variant {
            MDIR_A::NOEFFECT => false,
            MDIR_A::ACCEPTANCEFILTERING => true,
        }
    }
}
#[doc = "Reader of field `MDIR`"]
pub type MDIR_R = crate::R<bool, MDIR_A>;
impl MDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIR_A {
        match self.bits {
            false => MDIR_A::NOEFFECT,
            true => MDIR_A::ACCEPTANCEFILTERING,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_noeffect(&self) -> bool {
        *self == MDIR_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline(always)]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MDIR_A::ACCEPTANCEFILTERING
    }
}
#[doc = "Write proxy for field `MDIR`"]
pub struct MDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The message direction bit (DIR) has no effect on acceptance filtering"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MDIR_A::NOEFFECT)
    }
    #[doc = "The message direction bit (DIR) is used for acceptance filtering"]
    #[inline(always)]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MDIR_A::ACCEPTANCEFILTERING)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Mask extend identifier.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MXTD_A {
    #[doc = "0: The extended identifier bit (XTD) has no effect on acceptance filtering"]
    NOEFFECT,
    #[doc = "1: The extended identifier bit (XTD) is used for acceptance filtering"]
    ACCEPTANCEFILTERING,
}
impl From<MXTD_A> for bool {
    #[inline(always)]
    fn from(variant: MXTD_A) -> Self {
        match variant {
            MXTD_A::NOEFFECT => false,
            MXTD_A::ACCEPTANCEFILTERING => true,
        }
    }
}
#[doc = "Reader of field `MXTD`"]
pub type MXTD_R = crate::R<bool, MXTD_A>;
impl MXTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MXTD_A {
        match self.bits {
            false => MXTD_A::NOEFFECT,
            true => MXTD_A::ACCEPTANCEFILTERING,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_noeffect(&self) -> bool {
        *self == MXTD_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `ACCEPTANCEFILTERING`"]
    #[inline(always)]
    pub fn is_acceptancefiltering(&self) -> bool {
        *self == MXTD_A::ACCEPTANCEFILTERING
    }
}
#[doc = "Write proxy for field `MXTD`"]
pub struct MXTD_W<'a> {
    w: &'a mut W,
}
impl<'a> MXTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MXTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The extended identifier bit (XTD) has no effect on acceptance filtering"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MXTD_A::NOEFFECT)
    }
    #[doc = "The extended identifier bit (XTD) is used for acceptance filtering"]
    #[inline(always)]
    pub fn acceptancefiltering(self) -> &'a mut W {
        self.variant(MXTD_A::ACCEPTANCEFILTERING)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Identifier mask."]
    #[inline(always)]
    pub fn msk_28_16(&self) -> MSK_28_16_R {
        MSK_28_16_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - Mask message direction."]
    #[inline(always)]
    pub fn mdir(&self) -> MDIR_R {
        MDIR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask extend identifier."]
    #[inline(always)]
    pub fn mxtd(&self) -> MXTD_R {
        MXTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Identifier mask."]
    #[inline(always)]
    pub fn msk_28_16(&mut self) -> MSK_28_16_W {
        MSK_28_16_W { w: self }
    }
    #[doc = "Bit 14 - Mask message direction."]
    #[inline(always)]
    pub fn mdir(&mut self) -> MDIR_W {
        MDIR_W { w: self }
    }
    #[doc = "Bit 15 - Mask extend identifier."]
    #[inline(always)]
    pub fn mxtd(&mut self) -> MXTD_W {
        MXTD_W { w: self }
    }
}
