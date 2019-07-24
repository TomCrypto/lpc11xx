#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANIF_ARB2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct ID_28_16R {
    bits: u16,
}
impl ID_28_16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object"]
    RECEIVE,
    #[doc = "Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)"]
    TRANSMIT,
}
impl DIRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DIRR::RECEIVE => false,
            DIRR::TRANSMIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::RECEIVE,
            true => DIRR::TRANSMIT,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline]
    pub fn is_receive(&self) -> bool {
        *self == DIRR::RECEIVE
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline]
    pub fn is_transmit(&self) -> bool {
        *self == DIRR::TRANSMIT
    }
}
#[doc = "Possible values of the field `XTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTDR {
    #[doc = "The 11-bit standard identifier will be used for this message object"]
    _11_BIT_STANDARD_,
    #[doc = "The 29-bit extended identifier will be used for this message object"]
    _29_BIT_EXTENDED_,
}
impl XTDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            XTDR::_11_BIT_STANDARD_ => false,
            XTDR::_29_BIT_EXTENDED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTDR {
        match value {
            false => XTDR::_11_BIT_STANDARD_,
            true => XTDR::_29_BIT_EXTENDED_,
        }
    }
    #[doc = "Checks if the value of the field is `_11_BIT_STANDARD_`"]
    #[inline]
    pub fn is_11_bit_standard_(&self) -> bool {
        *self == XTDR::_11_BIT_STANDARD_
    }
    #[doc = "Checks if the value of the field is `_29_BIT_EXTENDED_`"]
    #[inline]
    pub fn is_29_bit_extended_(&self) -> bool {
        *self == XTDR::_29_BIT_EXTENDED_
    }
}
#[doc = "Possible values of the field `MSGVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGVALR {
    #[doc = "The message object is ignored by the message handler"]
    IGNORE,
    #[doc = "The message object is configured and should be considered by the message handler"]
    CONFIGURED,
}
impl MSGVALR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MSGVALR::IGNORE => false,
            MSGVALR::CONFIGURED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSGVALR {
        match value {
            false => MSGVALR::IGNORE,
            true => MSGVALR::CONFIGURED,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline]
    pub fn is_ignore(&self) -> bool {
        *self == MSGVALR::IGNORE
    }
    #[doc = "Checks if the value of the field is `CONFIGURED`"]
    #[inline]
    pub fn is_configured(&self) -> bool {
        *self == MSGVALR::CONFIGURED
    }
}
#[doc = r" Proxy"]
pub struct _ID_28_16W<'a> {
    w: &'a mut W,
}
impl<'a> _ID_28_16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object"]
    RECEIVE,
    #[doc = "Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)"]
    TRANSMIT,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRW::RECEIVE => false,
            DIRW::TRANSMIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direction = receive. On TXRQST, a Remote Frame with the identifier of this Message Object is transmitted. On reception of a Data Frame with matching identifier, that message is stored in this Message Object"]
    #[inline]
    pub fn receive(self) -> &'a mut W {
        self.variant(DIRW::RECEIVE)
    }
    #[doc = "Direction = transmit. On TXRQST, the respective Message Object is transmitted as a Data Frame. On reception of a Remote Frame with matching identifier, the TXRQST bit of this Message Object is set (if RMTEN = one)"]
    #[inline]
    pub fn transmit(self) -> &'a mut W {
        self.variant(DIRW::TRANSMIT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XTD`"]
pub enum XTDW {
    #[doc = "The 11-bit standard identifier will be used for this message object"]
    _11_BIT_STANDARD_,
    #[doc = "The 29-bit extended identifier will be used for this message object"]
    _29_BIT_EXTENDED_,
}
impl XTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTDW::_11_BIT_STANDARD_ => false,
            XTDW::_29_BIT_EXTENDED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The 11-bit standard identifier will be used for this message object"]
    #[inline]
    pub fn _11_bit_standard_(self) -> &'a mut W {
        self.variant(XTDW::_11_BIT_STANDARD_)
    }
    #[doc = "The 29-bit extended identifier will be used for this message object"]
    #[inline]
    pub fn _29_bit_extended_(self) -> &'a mut W {
        self.variant(XTDW::_29_BIT_EXTENDED_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSGVAL`"]
pub enum MSGVALW {
    #[doc = "The message object is ignored by the message handler"]
    IGNORE,
    #[doc = "The message object is configured and should be considered by the message handler"]
    CONFIGURED,
}
impl MSGVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSGVALW::IGNORE => false,
            MSGVALW::CONFIGURED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSGVALW<'a> {
    w: &'a mut W,
}
impl<'a> _MSGVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSGVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The message object is ignored by the message handler"]
    #[inline]
    pub fn ignore(self) -> &'a mut W {
        self.variant(MSGVALW::IGNORE)
    }
    #[doc = "The message object is configured and should be considered by the message handler"]
    #[inline]
    pub fn configured(self) -> &'a mut W {
        self.variant(MSGVALW::CONFIGURED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:12 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)"]
    #[inline]
    pub fn id_28_16(&self) -> ID_28_16R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ID_28_16R { bits }
    }
    #[doc = "Bit 13 - Message direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Extend identifier"]
    #[inline]
    pub fn xtd(&self) -> XTDR {
        XTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects during the initialization before it resets bit INIT in the CAN Control Register. This bit must also be reset before the identifier ID28:0, the control bits XTD, DIR, or the Data Length Code DLC3:0 are modified, or if the Messages Object is no longer required"]
    #[inline]
    pub fn msgval(&self) -> MSGVALR {
        MSGVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)"]
    #[inline]
    pub fn id_28_16(&mut self) -> _ID_28_16W {
        _ID_28_16W { w: self }
    }
    #[doc = "Bit 13 - Message direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 14 - Extend identifier"]
    #[inline]
    pub fn xtd(&mut self) -> _XTDW {
        _XTDW { w: self }
    }
    #[doc = "Bit 15 - Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects during the initialization before it resets bit INIT in the CAN Control Register. This bit must also be reset before the identifier ID28:0, the control bits XTD, DIR, or the Data Length Code DLC3:0 are modified, or if the Messages Object is no longer required"]
    #[inline]
    pub fn msgval(&mut self) -> _MSGVALW {
        _MSGVALW { w: self }
    }
}
