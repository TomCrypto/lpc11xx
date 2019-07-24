#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANIF_MCTRL {
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
pub struct DLC_3_0R {
    bits: u8,
}
impl DLC_3_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EOB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBR {
    #[doc = "Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer"]
    FIFO,
    #[doc = "Single message object or last message object of a FIFO buffer"]
    SINGELAST,
}
impl EOBR {
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
            EOBR::FIFO => false,
            EOBR::SINGELAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOBR {
        match value {
            false => EOBR::FIFO,
            true => EOBR::SINGELAST,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO`"]
    #[inline]
    pub fn is_fifo(&self) -> bool {
        *self == EOBR::FIFO
    }
    #[doc = "Checks if the value of the field is `SINGELAST`"]
    #[inline]
    pub fn is_singelast(&self) -> bool {
        *self == EOBR::SINGELAST
    }
}
#[doc = "Possible values of the field `TXRQST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQSTR {
    #[doc = "This message object is not waiting for transmission"]
    NOWAIT,
    #[doc = "The transmission of this message object is requested and is not yet done"]
    WAIT,
}
impl TXRQSTR {
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
            TXRQSTR::NOWAIT => false,
            TXRQSTR::WAIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRQSTR {
        match value {
            false => TXRQSTR::NOWAIT,
            true => TXRQSTR::WAIT,
        }
    }
    #[doc = "Checks if the value of the field is `NOWAIT`"]
    #[inline]
    pub fn is_nowait(&self) -> bool {
        *self == TXRQSTR::NOWAIT
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == TXRQSTR::WAIT
    }
}
#[doc = "Possible values of the field `RMTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMTENR {
    #[doc = "At the reception of a remote frame, TXRQST is left unchanged"]
    NOCHANGE,
    #[doc = "At the reception of a remote frame, TXRQST is set"]
    SET,
}
impl RMTENR {
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
            RMTENR::NOCHANGE => false,
            RMTENR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMTENR {
        match value {
            false => RMTENR::NOCHANGE,
            true => RMTENR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline]
    pub fn is_nochange(&self) -> bool {
        *self == RMTENR::NOCHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RMTENR::SET
    }
}
#[doc = "Possible values of the field `RXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIER {
    #[doc = "INTPND will be left unchanged after successful reception of a frame"]
    NOCHANGE,
    #[doc = "INTPND will be set after successful reception of a frame"]
    SET,
}
impl RXIER {
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
            RXIER::NOCHANGE => false,
            RXIER::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIER {
        match value {
            false => RXIER::NOCHANGE,
            true => RXIER::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline]
    pub fn is_nochange(&self) -> bool {
        *self == RXIER::NOCHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXIER::SET
    }
}
#[doc = "Possible values of the field `TXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIER {
    #[doc = "The INTPND bit will be left unchanged after a successful transmission of a frame"]
    NOCHANGE,
    #[doc = "INTPND will be set after a successful transmission of a frame"]
    SET,
}
impl TXIER {
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
            TXIER::NOCHANGE => false,
            TXIER::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIER {
        match value {
            false => TXIER::NOCHANGE,
            true => TXIER::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline]
    pub fn is_nochange(&self) -> bool {
        *self == TXIER::NOCHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == TXIER::SET
    }
}
#[doc = "Possible values of the field `UMASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UMASKR {
    #[doc = "Mask ignored"]
    IGNORE,
    #[doc = "Use mask (MSK\\[28:0\\], MXTD, and MDIR) for acceptance filtering"]
    USEMASK,
}
impl UMASKR {
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
            UMASKR::IGNORE => false,
            UMASKR::USEMASK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UMASKR {
        match value {
            false => UMASKR::IGNORE,
            true => UMASKR::USEMASK,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline]
    pub fn is_ignore(&self) -> bool {
        *self == UMASKR::IGNORE
    }
    #[doc = "Checks if the value of the field is `USEMASK`"]
    #[inline]
    pub fn is_usemask(&self) -> bool {
        *self == UMASKR::USEMASK
    }
}
#[doc = "Possible values of the field `INTPND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTPNDR {
    #[doc = "This message object is not the source of an interrupt"]
    NOINTSOURCE,
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority"]
    INTSOURCE,
}
impl INTPNDR {
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
            INTPNDR::NOINTSOURCE => false,
            INTPNDR::INTSOURCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTPNDR {
        match value {
            false => INTPNDR::NOINTSOURCE,
            true => INTPNDR::INTSOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTSOURCE`"]
    #[inline]
    pub fn is_nointsource(&self) -> bool {
        *self == INTPNDR::NOINTSOURCE
    }
    #[doc = "Checks if the value of the field is `INTSOURCE`"]
    #[inline]
    pub fn is_intsource(&self) -> bool {
        *self == INTPNDR::INTSOURCE
    }
}
#[doc = "Possible values of the field `MSGLST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGLSTR {
    #[doc = "No message lost since this bit was reset last by the CPU"]
    NOLOST,
    #[doc = "The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message"]
    NEWMESSAGE,
}
impl MSGLSTR {
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
            MSGLSTR::NOLOST => false,
            MSGLSTR::NEWMESSAGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSGLSTR {
        match value {
            false => MSGLSTR::NOLOST,
            true => MSGLSTR::NEWMESSAGE,
        }
    }
    #[doc = "Checks if the value of the field is `NOLOST`"]
    #[inline]
    pub fn is_nolost(&self) -> bool {
        *self == MSGLSTR::NOLOST
    }
    #[doc = "Checks if the value of the field is `NEWMESSAGE`"]
    #[inline]
    pub fn is_newmessage(&self) -> bool {
        *self == MSGLSTR::NEWMESSAGE
    }
}
#[doc = "Possible values of the field `NEWDAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEWDATR {
    #[doc = "No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU"]
    NONEWDATA,
    #[doc = "The message handler or the CPU has written new data into the data portion of this message object"]
    NEWDATA,
}
impl NEWDATR {
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
            NEWDATR::NONEWDATA => false,
            NEWDATR::NEWDATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEWDATR {
        match value {
            false => NEWDATR::NONEWDATA,
            true => NEWDATR::NEWDATA,
        }
    }
    #[doc = "Checks if the value of the field is `NONEWDATA`"]
    #[inline]
    pub fn is_nonewdata(&self) -> bool {
        *self == NEWDATR::NONEWDATA
    }
    #[doc = "Checks if the value of the field is `NEWDATA`"]
    #[inline]
    pub fn is_newdata(&self) -> bool {
        *self == NEWDATR::NEWDATA
    }
}
#[doc = r" Proxy"]
pub struct _DLC_3_0W<'a> {
    w: &'a mut W,
}
impl<'a> _DLC_3_0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOB`"]
pub enum EOBW {
    #[doc = "Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer"]
    FIFO,
    #[doc = "Single message object or last message object of a FIFO buffer"]
    SINGELAST,
}
impl EOBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOBW::FIFO => false,
            EOBW::SINGELAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOBW<'a> {
    w: &'a mut W,
}
impl<'a> _EOBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer"]
    #[inline]
    pub fn fifo(self) -> &'a mut W {
        self.variant(EOBW::FIFO)
    }
    #[doc = "Single message object or last message object of a FIFO buffer"]
    #[inline]
    pub fn singelast(self) -> &'a mut W {
        self.variant(EOBW::SINGELAST)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXRQST`"]
pub enum TXRQSTW {
    #[doc = "This message object is not waiting for transmission"]
    NOWAIT,
    #[doc = "The transmission of this message object is requested and is not yet done"]
    WAIT,
}
impl TXRQSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRQSTW::NOWAIT => false,
            TXRQSTW::WAIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRQSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRQSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRQSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This message object is not waiting for transmission"]
    #[inline]
    pub fn nowait(self) -> &'a mut W {
        self.variant(TXRQSTW::NOWAIT)
    }
    #[doc = "The transmission of this message object is requested and is not yet done"]
    #[inline]
    pub fn wait(self) -> &'a mut W {
        self.variant(TXRQSTW::WAIT)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMTEN`"]
pub enum RMTENW {
    #[doc = "At the reception of a remote frame, TXRQST is left unchanged"]
    NOCHANGE,
    #[doc = "At the reception of a remote frame, TXRQST is set"]
    SET,
}
impl RMTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMTENW::NOCHANGE => false,
            RMTENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RMTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "At the reception of a remote frame, TXRQST is left unchanged"]
    #[inline]
    pub fn nochange(self) -> &'a mut W {
        self.variant(RMTENW::NOCHANGE)
    }
    #[doc = "At the reception of a remote frame, TXRQST is set"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RMTENW::SET)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIE`"]
pub enum RXIEW {
    #[doc = "INTPND will be left unchanged after successful reception of a frame"]
    NOCHANGE,
    #[doc = "INTPND will be set after successful reception of a frame"]
    SET,
}
impl RXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIEW::NOCHANGE => false,
            RXIEW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INTPND will be left unchanged after successful reception of a frame"]
    #[inline]
    pub fn nochange(self) -> &'a mut W {
        self.variant(RXIEW::NOCHANGE)
    }
    #[doc = "INTPND will be set after successful reception of a frame"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXIEW::SET)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXIE`"]
pub enum TXIEW {
    #[doc = "The INTPND bit will be left unchanged after a successful transmission of a frame"]
    NOCHANGE,
    #[doc = "INTPND will be set after a successful transmission of a frame"]
    SET,
}
impl TXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIEW::NOCHANGE => false,
            TXIEW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The INTPND bit will be left unchanged after a successful transmission of a frame"]
    #[inline]
    pub fn nochange(self) -> &'a mut W {
        self.variant(TXIEW::NOCHANGE)
    }
    #[doc = "INTPND will be set after a successful transmission of a frame"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TXIEW::SET)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UMASK`"]
pub enum UMASKW {
    #[doc = "Mask ignored"]
    IGNORE,
    #[doc = "Use mask (MSK\\[28:0\\], MXTD, and MDIR) for acceptance filtering"]
    USEMASK,
}
impl UMASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UMASKW::IGNORE => false,
            UMASKW::USEMASK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _UMASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UMASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mask ignored"]
    #[inline]
    pub fn ignore(self) -> &'a mut W {
        self.variant(UMASKW::IGNORE)
    }
    #[doc = "Use mask (MSK\\[28:0\\], MXTD, and MDIR) for acceptance filtering"]
    #[inline]
    pub fn usemask(self) -> &'a mut W {
        self.variant(UMASKW::USEMASK)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTPND`"]
pub enum INTPNDW {
    #[doc = "This message object is not the source of an interrupt"]
    NOINTSOURCE,
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority"]
    INTSOURCE,
}
impl INTPNDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTPNDW::NOINTSOURCE => false,
            INTPNDW::INTSOURCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _INTPNDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTPNDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This message object is not the source of an interrupt"]
    #[inline]
    pub fn nointsource(self) -> &'a mut W {
        self.variant(INTPNDW::NOINTSOURCE)
    }
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority"]
    #[inline]
    pub fn intsource(self) -> &'a mut W {
        self.variant(INTPNDW::INTSOURCE)
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
#[doc = "Values that can be written to the field `MSGLST`"]
pub enum MSGLSTW {
    #[doc = "No message lost since this bit was reset last by the CPU"]
    NOLOST,
    #[doc = "The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message"]
    NEWMESSAGE,
}
impl MSGLSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSGLSTW::NOLOST => false,
            MSGLSTW::NEWMESSAGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSGLSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSGLSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSGLSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No message lost since this bit was reset last by the CPU"]
    #[inline]
    pub fn nolost(self) -> &'a mut W {
        self.variant(MSGLSTW::NOLOST)
    }
    #[doc = "The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message"]
    #[inline]
    pub fn newmessage(self) -> &'a mut W {
        self.variant(MSGLSTW::NEWMESSAGE)
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
#[doc = "Values that can be written to the field `NEWDAT`"]
pub enum NEWDATW {
    #[doc = "No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU"]
    NONEWDATA,
    #[doc = "The message handler or the CPU has written new data into the data portion of this message object"]
    NEWDATA,
}
impl NEWDATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NEWDATW::NONEWDATA => false,
            NEWDATW::NEWDATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEWDATW<'a> {
    w: &'a mut W,
}
impl<'a> _NEWDATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEWDATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU"]
    #[inline]
    pub fn nonewdata(self) -> &'a mut W {
        self.variant(NEWDATW::NONEWDATA)
    }
    #[doc = "The message handler or the CPU has written new data into the data portion of this message object"]
    #[inline]
    pub fn newdata(self) -> &'a mut W {
        self.variant(NEWDATW::NEWDATA)
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
    #[doc = "Bits 0:3 - Data length code The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. 0000 - 1000 = Data frame has 0 - 8 data bytes. 1001 - 1111 = Data frame has 8 data bytes"]
    #[inline]
    pub fn dlc_3_0(&self) -> DLC_3_0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLC_3_0R { bits }
    }
    #[doc = "Bit 7 - End of buffer"]
    #[inline]
    pub fn eob(&self) -> EOBR {
        EOBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Transmit request"]
    #[inline]
    pub fn txrqst(&self) -> TXRQSTR {
        TXRQSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Remote enable"]
    #[inline]
    pub fn rmten(&self) -> RMTENR {
        RMTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receive interrupt enable"]
    #[inline]
    pub fn rxie(&self) -> RXIER {
        RXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit interrupt enable"]
    #[inline]
    pub fn txie(&self) -> TXIER {
        TXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Use acceptance mask If UMASK is set to 1, the message object's mask bits have to be programmed during initialization of the message object before MAGVAL is set to 1"]
    #[inline]
    pub fn umask(&self) -> UMASKR {
        UMASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Interrupt pending"]
    #[inline]
    pub fn intpnd(&self) -> INTPNDR {
        INTPNDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Message lost (only valid for message objects in the direction receive)"]
    #[inline]
    pub fn msglst(&self) -> MSGLSTR {
        MSGLSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - New data"]
    #[inline]
    pub fn newdat(&self) -> NEWDATR {
        NEWDATR::_from({
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
    #[doc = "Bits 0:3 - Data length code The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. 0000 - 1000 = Data frame has 0 - 8 data bytes. 1001 - 1111 = Data frame has 8 data bytes"]
    #[inline]
    pub fn dlc_3_0(&mut self) -> _DLC_3_0W {
        _DLC_3_0W { w: self }
    }
    #[doc = "Bit 7 - End of buffer"]
    #[inline]
    pub fn eob(&mut self) -> _EOBW {
        _EOBW { w: self }
    }
    #[doc = "Bit 8 - Transmit request"]
    #[inline]
    pub fn txrqst(&mut self) -> _TXRQSTW {
        _TXRQSTW { w: self }
    }
    #[doc = "Bit 9 - Remote enable"]
    #[inline]
    pub fn rmten(&mut self) -> _RMTENW {
        _RMTENW { w: self }
    }
    #[doc = "Bit 10 - Receive interrupt enable"]
    #[inline]
    pub fn rxie(&mut self) -> _RXIEW {
        _RXIEW { w: self }
    }
    #[doc = "Bit 11 - Transmit interrupt enable"]
    #[inline]
    pub fn txie(&mut self) -> _TXIEW {
        _TXIEW { w: self }
    }
    #[doc = "Bit 12 - Use acceptance mask If UMASK is set to 1, the message object's mask bits have to be programmed during initialization of the message object before MAGVAL is set to 1"]
    #[inline]
    pub fn umask(&mut self) -> _UMASKW {
        _UMASKW { w: self }
    }
    #[doc = "Bit 13 - Interrupt pending"]
    #[inline]
    pub fn intpnd(&mut self) -> _INTPNDW {
        _INTPNDW { w: self }
    }
    #[doc = "Bit 14 - Message lost (only valid for message objects in the direction receive)"]
    #[inline]
    pub fn msglst(&mut self) -> _MSGLSTW {
        _MSGLSTW { w: self }
    }
    #[doc = "Bit 15 - New data"]
    #[inline]
    pub fn newdat(&mut self) -> _NEWDATW {
        _NEWDATW { w: self }
    }
}
