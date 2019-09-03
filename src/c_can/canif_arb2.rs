#[doc = "Reader of register CANIF%s_ARB2"]
pub type R = crate::R<u32, super::CANIF_ARB2>;
#[doc = "Writer for register CANIF%s_ARB2"]
pub type W = crate::W<u32, super::CANIF_ARB2>;
#[doc = "Register CANIF%s_ARB2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIF_ARB2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID_28_16`"]
pub type ID_28_16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ID_28_16`"]
pub struct ID_28_16_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_28_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Message direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object"]
    RECEIVE,
    #[doc = "1: Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)"]
    TRANSMIT,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        match variant {
            DIR_A::RECEIVE => false,
            DIR_A::TRANSMIT => true,
        }
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::RECEIVE,
            true => DIR_A::TRANSMIT,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == DIR_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == DIR_A::TRANSMIT
    }
}
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut W {
        self.variant(DIR_A::RECEIVE)
    }
    #[doc = "Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut W {
        self.variant(DIR_A::TRANSMIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Extend identifier.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTD_A {
    #[doc = "0: The 11-bit standard identifier will be used for this message object"]
    _11_BIT_STANDARD_,
    #[doc = "1: The 29-bit extended identifier will be used for this message object"]
    _29_BIT_EXTENDED_,
}
impl From<XTD_A> for bool {
    #[inline(always)]
    fn from(variant: XTD_A) -> Self {
        match variant {
            XTD_A::_11_BIT_STANDARD_ => false,
            XTD_A::_29_BIT_EXTENDED_ => true,
        }
    }
}
#[doc = "Reader of field `XTD`"]
pub type XTD_R = crate::R<bool, XTD_A>;
impl XTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTD_A {
        match self.bits {
            false => XTD_A::_11_BIT_STANDARD_,
            true => XTD_A::_29_BIT_EXTENDED_,
        }
    }
    #[doc = "Checks if the value of the field is `_11_BIT_STANDARD_`"]
    #[inline(always)]
    pub fn is_11_bit_standard_(&self) -> bool {
        *self == XTD_A::_11_BIT_STANDARD_
    }
    #[doc = "Checks if the value of the field is `_29_BIT_EXTENDED_`"]
    #[inline(always)]
    pub fn is_29_bit_extended_(&self) -> bool {
        *self == XTD_A::_29_BIT_EXTENDED_
    }
}
#[doc = "Write proxy for field `XTD`"]
pub struct XTD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 11-bit standard identifier will be used for this message object"]
    #[inline(always)]
    pub fn _11_bit_standard_(self) -> &'a mut W {
        self.variant(XTD_A::_11_BIT_STANDARD_)
    }
    #[doc = "The 29-bit extended identifier will be used for this message object"]
    #[inline(always)]
    pub fn _29_bit_extended_(self) -> &'a mut W {
        self.variant(XTD_A::_29_BIT_EXTENDED_)
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
#[doc = "Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects during the initialization before it resets bit INIT in the CAN Control Register. This bit must also be reset before the identifier ID28:0, the control bits XTD, DIR, or the Data Length Code DLC3:0 are modified, or if the Messages Object is no longer required.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGVAL_A {
    #[doc = "0: The message object is ignored by the message handler"]
    IGNORE,
    #[doc = "1: The message object is configured and should be considered by the message handler"]
    CONFIGURED,
}
impl From<MSGVAL_A> for bool {
    #[inline(always)]
    fn from(variant: MSGVAL_A) -> Self {
        match variant {
            MSGVAL_A::IGNORE => false,
            MSGVAL_A::CONFIGURED => true,
        }
    }
}
#[doc = "Reader of field `MSGVAL`"]
pub type MSGVAL_R = crate::R<bool, MSGVAL_A>;
impl MSGVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGVAL_A {
        match self.bits {
            false => MSGVAL_A::IGNORE,
            true => MSGVAL_A::CONFIGURED,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == MSGVAL_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `CONFIGURED`"]
    #[inline(always)]
    pub fn is_configured(&self) -> bool {
        *self == MSGVAL_A::CONFIGURED
    }
}
#[doc = "Write proxy for field `MSGVAL`"]
pub struct MSGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGVAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The message object is ignored by the message handler"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(MSGVAL_A::IGNORE)
    }
    #[doc = "The message object is configured and should be considered by the message handler"]
    #[inline(always)]
    pub fn configured(self) -> &'a mut W {
        self.variant(MSGVAL_A::CONFIGURED)
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
    #[doc = "Bits 0:12 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)."]
    #[inline(always)]
    pub fn id_28_16(&self) -> ID_28_16_R {
        ID_28_16_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Message direction."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Extend identifier."]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects during the initialization before it resets bit INIT in the CAN Control Register. This bit must also be reset before the identifier ID28:0, the control bits XTD, DIR, or the Data Length Code DLC3:0 are modified, or if the Messages Object is no longer required."]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)."]
    #[inline(always)]
    pub fn id_28_16(&mut self) -> ID_28_16_W {
        ID_28_16_W { w: self }
    }
    #[doc = "Bit 13 - Message direction."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 14 - Extend identifier."]
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W {
        XTD_W { w: self }
    }
    #[doc = "Bit 15 - Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects during the initialization before it resets bit INIT in the CAN Control Register. This bit must also be reset before the identifier ID28:0, the control bits XTD, DIR, or the Data Length Code DLC3:0 are modified, or if the Messages Object is no longer required."]
    #[inline(always)]
    pub fn msgval(&mut self) -> MSGVAL_W {
        MSGVAL_W { w: self }
    }
}
