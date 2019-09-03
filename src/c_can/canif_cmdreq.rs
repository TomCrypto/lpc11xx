#[doc = "Reader of register CANIF%s_CMDREQ"]
pub type R = crate::R<u32, super::CANIF_CMDREQ>;
#[doc = "Writer for register CANIF%s_CMDREQ"]
pub type W = crate::W<u32, super::CANIF_CMDREQ>;
#[doc = "Register CANIF%s_CMDREQ `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CANIF_CMDREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MN`"]
pub type MN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MN`"]
pub struct MN_W<'a> {
    w: &'a mut W,
}
impl<'a> MN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "BUSY flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Set to zero by hardware when read/write action to this Command request register has finished"]
    ZERO,
    #[doc = "1: Set to one by hardware when writing to this Command request register"]
    ONE,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::ZERO => false,
            BUSY_A::ONE => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::ZERO,
            true => BUSY_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == BUSY_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == BUSY_A::ONE
    }
}
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set to zero by hardware when read/write action to this Command request register has finished"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(BUSY_A::ZERO)
    }
    #[doc = "Set to one by hardware when writing to this Command request register"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(BUSY_A::ONE)
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
    #[doc = "Bits 0:5 - Message number 0x01 - 0x20 = Valid message numbers. The message object in the message RAM is selected for data transfer. 0x00 = Not a valid message number. This value is interpreted as 0x20.\\[1\\] 0x21 - 0x3F = Not a valid message number. This value is interpreted as 0x01 - 0x1F."]
    #[inline(always)]
    pub fn mn(&self) -> MN_R {
        MN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - BUSY flag."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message number 0x01 - 0x20 = Valid message numbers. The message object in the message RAM is selected for data transfer. 0x00 = Not a valid message number. This value is interpreted as 0x20.\\[1\\] 0x21 - 0x3F = Not a valid message number. This value is interpreted as 0x01 - 0x1F."]
    #[inline(always)]
    pub fn mn(&mut self) -> MN_W {
        MN_W { w: self }
    }
    #[doc = "Bit 15 - BUSY flag."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
}
