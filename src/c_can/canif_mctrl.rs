#[doc = "Reader of register CANIF%s_MCTRL"]
pub type R = crate::R<u32, super::CANIF_MCTRL>;
#[doc = "Writer for register CANIF%s_MCTRL"]
pub type W = crate::W<u32, super::CANIF_MCTRL>;
#[doc = "Register CANIF%s_MCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIF_MCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLC_3_0`"]
pub type DLC_3_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLC_3_0`"]
pub struct DLC_3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_3_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "End of buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOB_A {
    #[doc = "0: Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer"]
    FIFO,
    #[doc = "1: Single message object or last message object of a FIFO buffer"]
    SINGELAST,
}
impl From<EOB_A> for bool {
    #[inline(always)]
    fn from(variant: EOB_A) -> Self {
        match variant {
            EOB_A::FIFO => false,
            EOB_A::SINGELAST => true,
        }
    }
}
#[doc = "Reader of field `EOB`"]
pub type EOB_R = crate::R<bool, EOB_A>;
impl EOB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOB_A {
        match self.bits {
            false => EOB_A::FIFO,
            true => EOB_A::SINGELAST,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO`"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == EOB_A::FIFO
    }
    #[doc = "Checks if the value of the field is `SINGELAST`"]
    #[inline(always)]
    pub fn is_singelast(&self) -> bool {
        *self == EOB_A::SINGELAST
    }
}
#[doc = "Write proxy for field `EOB`"]
pub struct EOB_W<'a> {
    w: &'a mut W,
}
impl<'a> EOB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Message object belongs to a FIFO buffer and is not the last message object of that FIFO buffer"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut W {
        self.variant(EOB_A::FIFO)
    }
    #[doc = "Single message object or last message object of a FIFO buffer"]
    #[inline(always)]
    pub fn singelast(self) -> &'a mut W {
        self.variant(EOB_A::SINGELAST)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Transmit request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQST_A {
    #[doc = "0: This message object is not waiting for transmission"]
    NOWAIT,
    #[doc = "1: The transmission of this message object is requested and is not yet done"]
    WAIT,
}
impl From<TXRQST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRQST_A) -> Self {
        match variant {
            TXRQST_A::NOWAIT => false,
            TXRQST_A::WAIT => true,
        }
    }
}
#[doc = "Reader of field `TXRQST`"]
pub type TXRQST_R = crate::R<bool, TXRQST_A>;
impl TXRQST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRQST_A {
        match self.bits {
            false => TXRQST_A::NOWAIT,
            true => TXRQST_A::WAIT,
        }
    }
    #[doc = "Checks if the value of the field is `NOWAIT`"]
    #[inline(always)]
    pub fn is_nowait(&self) -> bool {
        *self == TXRQST_A::NOWAIT
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == TXRQST_A::WAIT
    }
}
#[doc = "Write proxy for field `TXRQST`"]
pub struct TXRQST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRQST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This message object is not waiting for transmission"]
    #[inline(always)]
    pub fn nowait(self) -> &'a mut W {
        self.variant(TXRQST_A::NOWAIT)
    }
    #[doc = "The transmission of this message object is requested and is not yet done"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut W {
        self.variant(TXRQST_A::WAIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Remote enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMTEN_A {
    #[doc = "0: At the reception of a remote frame, TXRQST is left unchanged"]
    NOCHANGE,
    #[doc = "1: At the reception of a remote frame, TXRQST is set"]
    SET,
}
impl From<RMTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RMTEN_A) -> Self {
        match variant {
            RMTEN_A::NOCHANGE => false,
            RMTEN_A::SET => true,
        }
    }
}
#[doc = "Reader of field `RMTEN`"]
pub type RMTEN_R = crate::R<bool, RMTEN_A>;
impl RMTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMTEN_A {
        match self.bits {
            false => RMTEN_A::NOCHANGE,
            true => RMTEN_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline(always)]
    pub fn is_nochange(&self) -> bool {
        *self == RMTEN_A::NOCHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RMTEN_A::SET
    }
}
#[doc = "Write proxy for field `RMTEN`"]
pub struct RMTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "At the reception of a remote frame, TXRQST is left unchanged"]
    #[inline(always)]
    pub fn nochange(self) -> &'a mut W {
        self.variant(RMTEN_A::NOCHANGE)
    }
    #[doc = "At the reception of a remote frame, TXRQST is set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RMTEN_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Receive interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIE_A {
    #[doc = "0: INTPND will be left unchanged after successful reception of a frame"]
    NOCHANGE,
    #[doc = "1: INTPND will be set after successful reception of a frame"]
    SET,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        match variant {
            RXIE_A::NOCHANGE => false,
            RXIE_A::SET => true,
        }
    }
}
#[doc = "Reader of field `RXIE`"]
pub type RXIE_R = crate::R<bool, RXIE_A>;
impl RXIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::NOCHANGE,
            true => RXIE_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline(always)]
    pub fn is_nochange(&self) -> bool {
        *self == RXIE_A::NOCHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RXIE_A::SET
    }
}
#[doc = "Write proxy for field `RXIE`"]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "INTPND will be left unchanged after successful reception of a frame"]
    #[inline(always)]
    pub fn nochange(self) -> &'a mut W {
        self.variant(RXIE_A::NOCHANGE)
    }
    #[doc = "INTPND will be set after successful reception of a frame"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXIE_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Transmit interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIE_A {
    #[doc = "0: The INTPND bit will be left unchanged after a successful transmission of a frame"]
    NOCHANGE,
    #[doc = "1: INTPND will be set after a successful transmission of a frame"]
    SET,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        match variant {
            TXIE_A::NOCHANGE => false,
            TXIE_A::SET => true,
        }
    }
}
#[doc = "Reader of field `TXIE`"]
pub type TXIE_R = crate::R<bool, TXIE_A>;
impl TXIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::NOCHANGE,
            true => TXIE_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline(always)]
    pub fn is_nochange(&self) -> bool {
        *self == TXIE_A::NOCHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TXIE_A::SET
    }
}
#[doc = "Write proxy for field `TXIE`"]
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The INTPND bit will be left unchanged after a successful transmission of a frame"]
    #[inline(always)]
    pub fn nochange(self) -> &'a mut W {
        self.variant(TXIE_A::NOCHANGE)
    }
    #[doc = "INTPND will be set after a successful transmission of a frame"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXIE_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Use acceptance mask If UMASK is set to 1, the message object's mask bits have to be programmed during initialization of the message object before MAGVAL is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UMASK_A {
    #[doc = "0: Mask ignored"]
    IGNORE,
    #[doc = "1: Use mask (MSK\\[28:0\\], MXTD, and MDIR) for acceptance filtering"]
    USEMASK,
}
impl From<UMASK_A> for bool {
    #[inline(always)]
    fn from(variant: UMASK_A) -> Self {
        match variant {
            UMASK_A::IGNORE => false,
            UMASK_A::USEMASK => true,
        }
    }
}
#[doc = "Reader of field `UMASK`"]
pub type UMASK_R = crate::R<bool, UMASK_A>;
impl UMASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UMASK_A {
        match self.bits {
            false => UMASK_A::IGNORE,
            true => UMASK_A::USEMASK,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == UMASK_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `USEMASK`"]
    #[inline(always)]
    pub fn is_usemask(&self) -> bool {
        *self == UMASK_A::USEMASK
    }
}
#[doc = "Write proxy for field `UMASK`"]
pub struct UMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> UMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UMASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mask ignored"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(UMASK_A::IGNORE)
    }
    #[doc = "Use mask (MSK\\[28:0\\], MXTD, and MDIR) for acceptance filtering"]
    #[inline(always)]
    pub fn usemask(self) -> &'a mut W {
        self.variant(UMASK_A::USEMASK)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Interrupt pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTPND_A {
    #[doc = "0: This message object is not the source of an interrupt"]
    NOINTSOURCE,
    #[doc = "1: This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority"]
    INTSOURCE,
}
impl From<INTPND_A> for bool {
    #[inline(always)]
    fn from(variant: INTPND_A) -> Self {
        match variant {
            INTPND_A::NOINTSOURCE => false,
            INTPND_A::INTSOURCE => true,
        }
    }
}
#[doc = "Reader of field `INTPND`"]
pub type INTPND_R = crate::R<bool, INTPND_A>;
impl INTPND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTPND_A {
        match self.bits {
            false => INTPND_A::NOINTSOURCE,
            true => INTPND_A::INTSOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTSOURCE`"]
    #[inline(always)]
    pub fn is_nointsource(&self) -> bool {
        *self == INTPND_A::NOINTSOURCE
    }
    #[doc = "Checks if the value of the field is `INTSOURCE`"]
    #[inline(always)]
    pub fn is_intsource(&self) -> bool {
        *self == INTPND_A::INTSOURCE
    }
}
#[doc = "Write proxy for field `INTPND`"]
pub struct INTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTPND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This message object is not the source of an interrupt"]
    #[inline(always)]
    pub fn nointsource(self) -> &'a mut W {
        self.variant(INTPND_A::NOINTSOURCE)
    }
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority"]
    #[inline(always)]
    pub fn intsource(self) -> &'a mut W {
        self.variant(INTPND_A::INTSOURCE)
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
#[doc = "Message lost (only valid for message objects in the direction receive).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGLST_A {
    #[doc = "0: No message lost since this bit was reset last by the CPU"]
    NOLOST,
    #[doc = "1: The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message"]
    NEWMESSAGE,
}
impl From<MSGLST_A> for bool {
    #[inline(always)]
    fn from(variant: MSGLST_A) -> Self {
        match variant {
            MSGLST_A::NOLOST => false,
            MSGLST_A::NEWMESSAGE => true,
        }
    }
}
#[doc = "Reader of field `MSGLST`"]
pub type MSGLST_R = crate::R<bool, MSGLST_A>;
impl MSGLST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGLST_A {
        match self.bits {
            false => MSGLST_A::NOLOST,
            true => MSGLST_A::NEWMESSAGE,
        }
    }
    #[doc = "Checks if the value of the field is `NOLOST`"]
    #[inline(always)]
    pub fn is_nolost(&self) -> bool {
        *self == MSGLST_A::NOLOST
    }
    #[doc = "Checks if the value of the field is `NEWMESSAGE`"]
    #[inline(always)]
    pub fn is_newmessage(&self) -> bool {
        *self == MSGLST_A::NEWMESSAGE
    }
}
#[doc = "Write proxy for field `MSGLST`"]
pub struct MSGLST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGLST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGLST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No message lost since this bit was reset last by the CPU"]
    #[inline(always)]
    pub fn nolost(self) -> &'a mut W {
        self.variant(MSGLST_A::NOLOST)
    }
    #[doc = "The Message Handler stored a new message into this object when NEWDAT was still set, the CPU has lost a message"]
    #[inline(always)]
    pub fn newmessage(self) -> &'a mut W {
        self.variant(MSGLST_A::NEWMESSAGE)
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
#[doc = "New data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEWDAT_A {
    #[doc = "0: No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU"]
    NONEWDATA,
    #[doc = "1: The message handler or the CPU has written new data into the data portion of this message object"]
    NEWDATA,
}
impl From<NEWDAT_A> for bool {
    #[inline(always)]
    fn from(variant: NEWDAT_A) -> Self {
        match variant {
            NEWDAT_A::NONEWDATA => false,
            NEWDAT_A::NEWDATA => true,
        }
    }
}
#[doc = "Reader of field `NEWDAT`"]
pub type NEWDAT_R = crate::R<bool, NEWDAT_A>;
impl NEWDAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEWDAT_A {
        match self.bits {
            false => NEWDAT_A::NONEWDATA,
            true => NEWDAT_A::NEWDATA,
        }
    }
    #[doc = "Checks if the value of the field is `NONEWDATA`"]
    #[inline(always)]
    pub fn is_nonewdata(&self) -> bool {
        *self == NEWDAT_A::NONEWDATA
    }
    #[doc = "Checks if the value of the field is `NEWDATA`"]
    #[inline(always)]
    pub fn is_newdata(&self) -> bool {
        *self == NEWDAT_A::NEWDATA
    }
}
#[doc = "Write proxy for field `NEWDAT`"]
pub struct NEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> NEWDAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEWDAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No new data has been written into the data portion of this message object by the message handler since this flag was cleared last by the CPU"]
    #[inline(always)]
    pub fn nonewdata(self) -> &'a mut W {
        self.variant(NEWDAT_A::NONEWDATA)
    }
    #[doc = "The message handler or the CPU has written new data into the data portion of this message object"]
    #[inline(always)]
    pub fn newdata(self) -> &'a mut W {
        self.variant(NEWDAT_A::NEWDATA)
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
    #[doc = "Bits 0:3 - Data length code The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. 0000 - 1000 = Data frame has 0 - 8 data bytes. 1001 - 1111 = Data frame has 8 data bytes."]
    #[inline(always)]
    pub fn dlc_3_0(&self) -> DLC_3_0_R {
        DLC_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of buffer."]
    #[inline(always)]
    pub fn eob(&self) -> EOB_R {
        EOB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit request."]
    #[inline(always)]
    pub fn txrqst(&self) -> TXRQST_R {
        TXRQST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remote enable."]
    #[inline(always)]
    pub fn rmten(&self) -> RMTEN_R {
        RMTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive interrupt enable."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit interrupt enable."]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Use acceptance mask If UMASK is set to 1, the message object's mask bits have to be programmed during initialization of the message object before MAGVAL is set to 1."]
    #[inline(always)]
    pub fn umask(&self) -> UMASK_R {
        UMASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt pending."]
    #[inline(always)]
    pub fn intpnd(&self) -> INTPND_R {
        INTPND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Message lost (only valid for message objects in the direction receive)."]
    #[inline(always)]
    pub fn msglst(&self) -> MSGLST_R {
        MSGLST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New data."]
    #[inline(always)]
    pub fn newdat(&self) -> NEWDAT_R {
        NEWDAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. 0000 - 1000 = Data frame has 0 - 8 data bytes. 1001 - 1111 = Data frame has 8 data bytes."]
    #[inline(always)]
    pub fn dlc_3_0(&mut self) -> DLC_3_0_W {
        DLC_3_0_W { w: self }
    }
    #[doc = "Bit 7 - End of buffer."]
    #[inline(always)]
    pub fn eob(&mut self) -> EOB_W {
        EOB_W { w: self }
    }
    #[doc = "Bit 8 - Transmit request."]
    #[inline(always)]
    pub fn txrqst(&mut self) -> TXRQST_W {
        TXRQST_W { w: self }
    }
    #[doc = "Bit 9 - Remote enable."]
    #[inline(always)]
    pub fn rmten(&mut self) -> RMTEN_W {
        RMTEN_W { w: self }
    }
    #[doc = "Bit 10 - Receive interrupt enable."]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    #[doc = "Bit 11 - Transmit interrupt enable."]
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W {
        TXIE_W { w: self }
    }
    #[doc = "Bit 12 - Use acceptance mask If UMASK is set to 1, the message object's mask bits have to be programmed during initialization of the message object before MAGVAL is set to 1."]
    #[inline(always)]
    pub fn umask(&mut self) -> UMASK_W {
        UMASK_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt pending."]
    #[inline(always)]
    pub fn intpnd(&mut self) -> INTPND_W {
        INTPND_W { w: self }
    }
    #[doc = "Bit 14 - Message lost (only valid for message objects in the direction receive)."]
    #[inline(always)]
    pub fn msglst(&mut self) -> MSGLST_W {
        MSGLST_W { w: self }
    }
    #[doc = "Bit 15 - New data."]
    #[inline(always)]
    pub fn newdat(&mut self) -> NEWDAT_W {
        NEWDAT_W { w: self }
    }
}
