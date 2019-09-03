#[doc = "Reader of register CANSTAT"]
pub type R = crate::R<u32, super::CANSTAT>;
#[doc = "Writer for register CANSTAT"]
pub type W = crate::W<u32, super::CANSTAT>;
#[doc = "Register CANSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CANSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Last error code Type of the last error to occur on the CAN bus.The LEC field holds a code which indicates the type of the last error to occur on the CAN bus. This field will be cleared to 0 when a message has been transferred (reception or transmission) without error. The unused code 111 may be written by the CPU to check for updates.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEC_A {
    #[doc = "0: No error"]
    NO_ERROR_,
    #[doc = "1: Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed"]
    STUFF_ERROR,
    #[doc = "2: Form error: A fixed format part of a received frame has the wrong format"]
    FORM_ERROR,
    #[doc = "3: AckError: The message this CAN core transmitted was not acknowledged"]
    ACKERROR,
    #[doc = "4: Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant"]
    BIT1ERROR,
    #[doc = "5: Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)"]
    BIT0ERROR,
    #[doc = "6: CRCError: The CRC checksum was incorrect in the message received"]
    CRCERROR,
    #[doc = "7: Unused: No CAN bus event was detected (written by the CPU)"]
    UNUSED,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        match variant {
            LEC_A::NO_ERROR_ => 0,
            LEC_A::STUFF_ERROR => 1,
            LEC_A::FORM_ERROR => 2,
            LEC_A::ACKERROR => 3,
            LEC_A::BIT1ERROR => 4,
            LEC_A::BIT0ERROR => 5,
            LEC_A::CRCERROR => 6,
            LEC_A::UNUSED => 7,
        }
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, LEC_A>;
impl LEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NO_ERROR_,
            1 => LEC_A::STUFF_ERROR,
            2 => LEC_A::FORM_ERROR,
            3 => LEC_A::ACKERROR,
            4 => LEC_A::BIT1ERROR,
            5 => LEC_A::BIT0ERROR,
            6 => LEC_A::CRCERROR,
            7 => LEC_A::UNUSED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR_`"]
    #[inline(always)]
    pub fn is_no_error_(&self) -> bool {
        *self == LEC_A::NO_ERROR_
    }
    #[doc = "Checks if the value of the field is `STUFF_ERROR`"]
    #[inline(always)]
    pub fn is_stuff_error(&self) -> bool {
        *self == LEC_A::STUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR`"]
    #[inline(always)]
    pub fn is_form_error(&self) -> bool {
        *self == LEC_A::FORM_ERROR
    }
    #[doc = "Checks if the value of the field is `ACKERROR`"]
    #[inline(always)]
    pub fn is_ackerror(&self) -> bool {
        *self == LEC_A::ACKERROR
    }
    #[doc = "Checks if the value of the field is `BIT1ERROR`"]
    #[inline(always)]
    pub fn is_bit1error(&self) -> bool {
        *self == LEC_A::BIT1ERROR
    }
    #[doc = "Checks if the value of the field is `BIT0ERROR`"]
    #[inline(always)]
    pub fn is_bit0error(&self) -> bool {
        *self == LEC_A::BIT0ERROR
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline(always)]
    pub fn is_crcerror(&self) -> bool {
        *self == LEC_A::CRCERROR
    }
    #[doc = "Checks if the value of the field is `UNUSED`"]
    #[inline(always)]
    pub fn is_unused(&self) -> bool {
        *self == LEC_A::UNUSED
    }
}
#[doc = "Write proxy for field `LEC`"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error_(self) -> &'a mut W {
        self.variant(LEC_A::NO_ERROR_)
    }
    #[doc = "Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed"]
    #[inline(always)]
    pub fn stuff_error(self) -> &'a mut W {
        self.variant(LEC_A::STUFF_ERROR)
    }
    #[doc = "Form error: A fixed format part of a received frame has the wrong format"]
    #[inline(always)]
    pub fn form_error(self) -> &'a mut W {
        self.variant(LEC_A::FORM_ERROR)
    }
    #[doc = "AckError: The message this CAN core transmitted was not acknowledged"]
    #[inline(always)]
    pub fn ackerror(self) -> &'a mut W {
        self.variant(LEC_A::ACKERROR)
    }
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant"]
    #[inline(always)]
    pub fn bit1error(self) -> &'a mut W {
        self.variant(LEC_A::BIT1ERROR)
    }
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)"]
    #[inline(always)]
    pub fn bit0error(self) -> &'a mut W {
        self.variant(LEC_A::BIT0ERROR)
    }
    #[doc = "CRCError: The CRC checksum was incorrect in the message received"]
    #[inline(always)]
    pub fn crcerror(self) -> &'a mut W {
        self.variant(LEC_A::CRCERROR)
    }
    #[doc = "Unused: No CAN bus event was detected (written by the CPU)"]
    #[inline(always)]
    pub fn unused(self) -> &'a mut W {
        self.variant(LEC_A::UNUSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Transmitted a message successfully This bit is reset by the CPU. It is never reset by the CAN controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOK_A {
    #[doc = "0: Since this bit was reset by the CPU, no message has been successfully transmitted"]
    NOTRANSMIT,
    #[doc = "1: Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)"]
    TRANSMIT,
}
impl From<TXOK_A> for bool {
    #[inline(always)]
    fn from(variant: TXOK_A) -> Self {
        match variant {
            TXOK_A::NOTRANSMIT => false,
            TXOK_A::TRANSMIT => true,
        }
    }
}
#[doc = "Reader of field `TXOK`"]
pub type TXOK_R = crate::R<bool, TXOK_A>;
impl TXOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOK_A {
        match self.bits {
            false => TXOK_A::NOTRANSMIT,
            true => TXOK_A::TRANSMIT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRANSMIT`"]
    #[inline(always)]
    pub fn is_notransmit(&self) -> bool {
        *self == TXOK_A::NOTRANSMIT
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == TXOK_A::TRANSMIT
    }
}
#[doc = "Write proxy for field `TXOK`"]
pub struct TXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted"]
    #[inline(always)]
    pub fn notransmit(self) -> &'a mut W {
        self.variant(TXOK_A::NOTRANSMIT)
    }
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut W {
        self.variant(TXOK_A::TRANSMIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Received a message successfully This bit is reset by the CPU. It is never reset by the CAN controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOK_A {
    #[doc = "0: Since this bit was last reset by the CPU, no message has been successfully transmitted"]
    NOTRANSMIT,
    #[doc = "1: Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering"]
    TRANSMIT,
}
impl From<RXOK_A> for bool {
    #[inline(always)]
    fn from(variant: RXOK_A) -> Self {
        match variant {
            RXOK_A::NOTRANSMIT => false,
            RXOK_A::TRANSMIT => true,
        }
    }
}
#[doc = "Reader of field `RXOK`"]
pub type RXOK_R = crate::R<bool, RXOK_A>;
impl RXOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOK_A {
        match self.bits {
            false => RXOK_A::NOTRANSMIT,
            true => RXOK_A::TRANSMIT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRANSMIT`"]
    #[inline(always)]
    pub fn is_notransmit(&self) -> bool {
        *self == RXOK_A::NOTRANSMIT
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == RXOK_A::TRANSMIT
    }
}
#[doc = "Write proxy for field `RXOK`"]
pub struct RXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Since this bit was last reset by the CPU, no message has been successfully transmitted"]
    #[inline(always)]
    pub fn notransmit(self) -> &'a mut W {
        self.variant(RXOK_A::NOTRANSMIT)
    }
    #[doc = "Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut W {
        self.variant(RXOK_A::TRANSMIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Error passive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPASS_A {
    #[doc = "0: The CAN controller is in the error active state"]
    ACTIVE,
    #[doc = "1: The CAN controller is in the error passive state as defined in the CAN 2.0 specification"]
    PASSIVE,
}
impl From<EPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EPASS_A) -> Self {
        match variant {
            EPASS_A::ACTIVE => false,
            EPASS_A::PASSIVE => true,
        }
    }
}
#[doc = "Reader of field `EPASS`"]
pub type EPASS_R = crate::R<bool, EPASS_A>;
impl EPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPASS_A {
        match self.bits {
            false => EPASS_A::ACTIVE,
            true => EPASS_A::PASSIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == EPASS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `PASSIVE`"]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == EPASS_A::PASSIVE
    }
}
#[doc = "Write proxy for field `EPASS`"]
pub struct EPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CAN controller is in the error active state"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EPASS_A::ACTIVE)
    }
    #[doc = "The CAN controller is in the error passive state as defined in the CAN 2.0 specification"]
    #[inline(always)]
    pub fn passive(self) -> &'a mut W {
        self.variant(EPASS_A::PASSIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Warning status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWARN_A {
    #[doc = "0: Both error counters are below the error warning limit of 96"]
    BELOWWARNINGLIM,
    #[doc = "1: At least one of the error counters in the EML has reached the error warning limit of 96"]
    WARNINGLIM,
}
impl From<EWARN_A> for bool {
    #[inline(always)]
    fn from(variant: EWARN_A) -> Self {
        match variant {
            EWARN_A::BELOWWARNINGLIM => false,
            EWARN_A::WARNINGLIM => true,
        }
    }
}
#[doc = "Reader of field `EWARN`"]
pub type EWARN_R = crate::R<bool, EWARN_A>;
impl EWARN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWARN_A {
        match self.bits {
            false => EWARN_A::BELOWWARNINGLIM,
            true => EWARN_A::WARNINGLIM,
        }
    }
    #[doc = "Checks if the value of the field is `BELOWWARNINGLIM`"]
    #[inline(always)]
    pub fn is_belowwarninglim(&self) -> bool {
        *self == EWARN_A::BELOWWARNINGLIM
    }
    #[doc = "Checks if the value of the field is `WARNINGLIM`"]
    #[inline(always)]
    pub fn is_warninglim(&self) -> bool {
        *self == EWARN_A::WARNINGLIM
    }
}
#[doc = "Write proxy for field `EWARN`"]
pub struct EWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> EWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWARN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Both error counters are below the error warning limit of 96"]
    #[inline(always)]
    pub fn belowwarninglim(self) -> &'a mut W {
        self.variant(EWARN_A::BELOWWARNINGLIM)
    }
    #[doc = "At least one of the error counters in the EML has reached the error warning limit of 96"]
    #[inline(always)]
    pub fn warninglim(self) -> &'a mut W {
        self.variant(EWARN_A::WARNINGLIM)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Busoff status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF_A {
    #[doc = "0: The CAN module is not in busoff"]
    NOTBUSOFF,
    #[doc = "1: The CAN controller is in busoff state"]
    BUSOFF,
}
impl From<BOFF_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF_A) -> Self {
        match variant {
            BOFF_A::NOTBUSOFF => false,
            BOFF_A::BUSOFF => true,
        }
    }
}
#[doc = "Reader of field `BOFF`"]
pub type BOFF_R = crate::R<bool, BOFF_A>;
impl BOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFF_A {
        match self.bits {
            false => BOFF_A::NOTBUSOFF,
            true => BOFF_A::BUSOFF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSOFF`"]
    #[inline(always)]
    pub fn is_notbusoff(&self) -> bool {
        *self == BOFF_A::NOTBUSOFF
    }
    #[doc = "Checks if the value of the field is `BUSOFF`"]
    #[inline(always)]
    pub fn is_busoff(&self) -> bool {
        *self == BOFF_A::BUSOFF
    }
}
#[doc = "Write proxy for field `BOFF`"]
pub struct BOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CAN module is not in busoff"]
    #[inline(always)]
    pub fn notbusoff(self) -> &'a mut W {
        self.variant(BOFF_A::NOTBUSOFF)
    }
    #[doc = "The CAN controller is in busoff state"]
    #[inline(always)]
    pub fn busoff(self) -> &'a mut W {
        self.variant(BOFF_A::BUSOFF)
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
impl R {
    #[doc = "Bits 0:2 - Last error code Type of the last error to occur on the CAN bus.The LEC field holds a code which indicates the type of the last error to occur on the CAN bus. This field will be cleared to 0 when a message has been transferred (reception or transmission) without error. The unused code 111 may be written by the CPU to check for updates."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Transmitted a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline(always)]
    pub fn rxok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error passive."]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Warning status."]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Busoff status."]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last error code Type of the last error to occur on the CAN bus.The LEC field holds a code which indicates the type of the last error to occur on the CAN bus. This field will be cleared to 0 when a message has been transferred (reception or transmission) without error. The unused code 111 may be written by the CPU to check for updates."]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bit 3 - Transmitted a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline(always)]
    pub fn txok(&mut self) -> TXOK_W {
        TXOK_W { w: self }
    }
    #[doc = "Bit 4 - Received a message successfully This bit is reset by the CPU. It is never reset by the CAN controller."]
    #[inline(always)]
    pub fn rxok(&mut self) -> RXOK_W {
        RXOK_W { w: self }
    }
    #[doc = "Bit 5 - Error passive."]
    #[inline(always)]
    pub fn epass(&mut self) -> EPASS_W {
        EPASS_W { w: self }
    }
    #[doc = "Bit 6 - Warning status."]
    #[inline(always)]
    pub fn ewarn(&mut self) -> EWARN_W {
        EWARN_W { w: self }
    }
    #[doc = "Bit 7 - Busoff status."]
    #[inline(always)]
    pub fn boff(&mut self) -> BOFF_W {
        BOFF_W { w: self }
    }
}
