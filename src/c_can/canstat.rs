#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANSTAT {
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
#[doc = "Possible values of the field `LEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECR {
    #[doc = "No error"]
    NO_ERROR_,
    #[doc = "Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed"]
    STUFF_ERROR,
    #[doc = "Form error: A fixed format part of a received frame has the wrong format"]
    FORM_ERROR,
    #[doc = "AckError: The message this CAN core transmitted was not acknowledged"]
    ACKERROR,
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant"]
    BIT1ERROR,
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)"]
    BIT0ERROR,
    #[doc = "CRCError: The CRC checksum was incorrect in the message received"]
    CRCERROR,
    #[doc = "Unused: No CAN bus event was detected (written by the CPU)"]
    UNUSED,
}
impl LECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LECR::NO_ERROR_ => 0,
            LECR::STUFF_ERROR => 1,
            LECR::FORM_ERROR => 2,
            LECR::ACKERROR => 3,
            LECR::BIT1ERROR => 4,
            LECR::BIT0ERROR => 5,
            LECR::CRCERROR => 6,
            LECR::UNUSED => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LECR {
        match value {
            0 => LECR::NO_ERROR_,
            1 => LECR::STUFF_ERROR,
            2 => LECR::FORM_ERROR,
            3 => LECR::ACKERROR,
            4 => LECR::BIT1ERROR,
            5 => LECR::BIT0ERROR,
            6 => LECR::CRCERROR,
            7 => LECR::UNUSED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR_`"]
    #[inline]
    pub fn is_no_error_(&self) -> bool {
        *self == LECR::NO_ERROR_
    }
    #[doc = "Checks if the value of the field is `STUFF_ERROR`"]
    #[inline]
    pub fn is_stuff_error(&self) -> bool {
        *self == LECR::STUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR`"]
    #[inline]
    pub fn is_form_error(&self) -> bool {
        *self == LECR::FORM_ERROR
    }
    #[doc = "Checks if the value of the field is `ACKERROR`"]
    #[inline]
    pub fn is_ackerror(&self) -> bool {
        *self == LECR::ACKERROR
    }
    #[doc = "Checks if the value of the field is `BIT1ERROR`"]
    #[inline]
    pub fn is_bit1error(&self) -> bool {
        *self == LECR::BIT1ERROR
    }
    #[doc = "Checks if the value of the field is `BIT0ERROR`"]
    #[inline]
    pub fn is_bit0error(&self) -> bool {
        *self == LECR::BIT0ERROR
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline]
    pub fn is_crcerror(&self) -> bool {
        *self == LECR::CRCERROR
    }
    #[doc = "Checks if the value of the field is `UNUSED`"]
    #[inline]
    pub fn is_unused(&self) -> bool {
        *self == LECR::UNUSED
    }
}
#[doc = "Possible values of the field `TXOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOKR {
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted"]
    NOTRANSMIT,
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)"]
    TRANSMIT,
}
impl TXOKR {
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
            TXOKR::NOTRANSMIT => false,
            TXOKR::TRANSMIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOKR {
        match value {
            false => TXOKR::NOTRANSMIT,
            true => TXOKR::TRANSMIT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRANSMIT`"]
    #[inline]
    pub fn is_notransmit(&self) -> bool {
        *self == TXOKR::NOTRANSMIT
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline]
    pub fn is_transmit(&self) -> bool {
        *self == TXOKR::TRANSMIT
    }
}
#[doc = "Possible values of the field `RXOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOKR {
    #[doc = "Since this bit was last reset by the CPU, no message has been successfully transmitted"]
    NOTRANSMIT,
    #[doc = "Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering"]
    TRANSMIT,
}
impl RXOKR {
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
            RXOKR::NOTRANSMIT => false,
            RXOKR::TRANSMIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOKR {
        match value {
            false => RXOKR::NOTRANSMIT,
            true => RXOKR::TRANSMIT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRANSMIT`"]
    #[inline]
    pub fn is_notransmit(&self) -> bool {
        *self == RXOKR::NOTRANSMIT
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline]
    pub fn is_transmit(&self) -> bool {
        *self == RXOKR::TRANSMIT
    }
}
#[doc = "Possible values of the field `EPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPASSR {
    #[doc = "The CAN controller is in the error active state"]
    ACTIVE,
    #[doc = "The CAN controller is in the error passive state as defined in the CAN 2.0 specification"]
    PASSIVE,
}
impl EPASSR {
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
            EPASSR::ACTIVE => false,
            EPASSR::PASSIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPASSR {
        match value {
            false => EPASSR::ACTIVE,
            true => EPASSR::PASSIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == EPASSR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `PASSIVE`"]
    #[inline]
    pub fn is_passive(&self) -> bool {
        *self == EPASSR::PASSIVE
    }
}
#[doc = "Possible values of the field `EWARN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWARNR {
    #[doc = "Both error counters are below the error warning limit of 96"]
    BELOWWARNINGLIM,
    #[doc = "At least one of the error counters in the EML has reached the error warning limit of 96"]
    WARNINGLIM,
}
impl EWARNR {
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
            EWARNR::BELOWWARNINGLIM => false,
            EWARNR::WARNINGLIM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWARNR {
        match value {
            false => EWARNR::BELOWWARNINGLIM,
            true => EWARNR::WARNINGLIM,
        }
    }
    #[doc = "Checks if the value of the field is `BELOWWARNINGLIM`"]
    #[inline]
    pub fn is_belowwarninglim(&self) -> bool {
        *self == EWARNR::BELOWWARNINGLIM
    }
    #[doc = "Checks if the value of the field is `WARNINGLIM`"]
    #[inline]
    pub fn is_warninglim(&self) -> bool {
        *self == EWARNR::WARNINGLIM
    }
}
#[doc = "Possible values of the field `BOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFR {
    #[doc = "The CAN module is not in busoff"]
    NOTBUSOFF,
    #[doc = "The CAN controller is in busoff state"]
    BUSOFF,
}
impl BOFFR {
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
            BOFFR::NOTBUSOFF => false,
            BOFFR::BUSOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFR {
        match value {
            false => BOFFR::NOTBUSOFF,
            true => BOFFR::BUSOFF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSOFF`"]
    #[inline]
    pub fn is_notbusoff(&self) -> bool {
        *self == BOFFR::NOTBUSOFF
    }
    #[doc = "Checks if the value of the field is `BUSOFF`"]
    #[inline]
    pub fn is_busoff(&self) -> bool {
        *self == BOFFR::BUSOFF
    }
}
#[doc = "Values that can be written to the field `LEC`"]
pub enum LECW {
    #[doc = "No error"]
    NO_ERROR_,
    #[doc = "Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed"]
    STUFF_ERROR,
    #[doc = "Form error: A fixed format part of a received frame has the wrong format"]
    FORM_ERROR,
    #[doc = "AckError: The message this CAN core transmitted was not acknowledged"]
    ACKERROR,
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant"]
    BIT1ERROR,
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)"]
    BIT0ERROR,
    #[doc = "CRCError: The CRC checksum was incorrect in the message received"]
    CRCERROR,
    #[doc = "Unused: No CAN bus event was detected (written by the CPU)"]
    UNUSED,
}
impl LECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LECW::NO_ERROR_ => 0,
            LECW::STUFF_ERROR => 1,
            LECW::FORM_ERROR => 2,
            LECW::ACKERROR => 3,
            LECW::BIT1ERROR => 4,
            LECW::BIT0ERROR => 5,
            LECW::CRCERROR => 6,
            LECW::UNUSED => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LECW<'a> {
    w: &'a mut W,
}
impl<'a> _LECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LECW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No error"]
    #[inline]
    pub fn no_error_(self) -> &'a mut W {
        self.variant(LECW::NO_ERROR_)
    }
    #[doc = "Stuff error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed"]
    #[inline]
    pub fn stuff_error(self) -> &'a mut W {
        self.variant(LECW::STUFF_ERROR)
    }
    #[doc = "Form error: A fixed format part of a received frame has the wrong format"]
    #[inline]
    pub fn form_error(self) -> &'a mut W {
        self.variant(LECW::FORM_ERROR)
    }
    #[doc = "AckError: The message this CAN core transmitted was not acknowledged"]
    #[inline]
    pub fn ackerror(self) -> &'a mut W {
        self.variant(LECW::ACKERROR)
    }
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a HIGH/recessive level (bit of logical value 1), but the monitored bus value was LOW/dominant"]
    #[inline]
    pub fn bit1error(self) -> &'a mut W {
        self.variant(LECW::BIT1ERROR)
    }
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a LOW/dominant level (data or identifier bit logical value 0), but the monitored Bus value was HIGH/recessive. During busoff recovery this status is set each time a sequence of 11 HIGH/recessive bits has been monitored. This enables the CPU to monitor the proceeding of the busoff recovery sequence (indicating the bus is not stuck at LOW/dominant or continuously disturbed)"]
    #[inline]
    pub fn bit0error(self) -> &'a mut W {
        self.variant(LECW::BIT0ERROR)
    }
    #[doc = "CRCError: The CRC checksum was incorrect in the message received"]
    #[inline]
    pub fn crcerror(self) -> &'a mut W {
        self.variant(LECW::CRCERROR)
    }
    #[doc = "Unused: No CAN bus event was detected (written by the CPU)"]
    #[inline]
    pub fn unused(self) -> &'a mut W {
        self.variant(LECW::UNUSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXOK`"]
pub enum TXOKW {
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted"]
    NOTRANSMIT,
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)"]
    TRANSMIT,
}
impl TXOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOKW::NOTRANSMIT => false,
            TXOKW::TRANSMIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted"]
    #[inline]
    pub fn notransmit(self) -> &'a mut W {
        self.variant(TXOKW::NOTRANSMIT)
    }
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully transmitted (error free and acknowledged by at least one other node)"]
    #[inline]
    pub fn transmit(self) -> &'a mut W {
        self.variant(TXOKW::TRANSMIT)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXOK`"]
pub enum RXOKW {
    #[doc = "Since this bit was last reset by the CPU, no message has been successfully transmitted"]
    NOTRANSMIT,
    #[doc = "Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering"]
    TRANSMIT,
}
impl RXOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOKW::NOTRANSMIT => false,
            RXOKW::TRANSMIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Since this bit was last reset by the CPU, no message has been successfully transmitted"]
    #[inline]
    pub fn notransmit(self) -> &'a mut W {
        self.variant(RXOKW::NOTRANSMIT)
    }
    #[doc = "Since this bit was last set to zero by the CPU, a message has been successfully received independent of the result of acceptance filtering"]
    #[inline]
    pub fn transmit(self) -> &'a mut W {
        self.variant(RXOKW::TRANSMIT)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPASS`"]
pub enum EPASSW {
    #[doc = "The CAN controller is in the error active state"]
    ACTIVE,
    #[doc = "The CAN controller is in the error passive state as defined in the CAN 2.0 specification"]
    PASSIVE,
}
impl EPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPASSW::ACTIVE => false,
            EPASSW::PASSIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CAN controller is in the error active state"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(EPASSW::ACTIVE)
    }
    #[doc = "The CAN controller is in the error passive state as defined in the CAN 2.0 specification"]
    #[inline]
    pub fn passive(self) -> &'a mut W {
        self.variant(EPASSW::PASSIVE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EWARN`"]
pub enum EWARNW {
    #[doc = "Both error counters are below the error warning limit of 96"]
    BELOWWARNINGLIM,
    #[doc = "At least one of the error counters in the EML has reached the error warning limit of 96"]
    WARNINGLIM,
}
impl EWARNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWARNW::BELOWWARNINGLIM => false,
            EWARNW::WARNINGLIM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _EWARNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWARNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Both error counters are below the error warning limit of 96"]
    #[inline]
    pub fn belowwarninglim(self) -> &'a mut W {
        self.variant(EWARNW::BELOWWARNINGLIM)
    }
    #[doc = "At least one of the error counters in the EML has reached the error warning limit of 96"]
    #[inline]
    pub fn warninglim(self) -> &'a mut W {
        self.variant(EWARNW::WARNINGLIM)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOFF`"]
pub enum BOFFW {
    #[doc = "The CAN module is not in busoff"]
    NOTBUSOFF,
    #[doc = "The CAN controller is in busoff state"]
    BUSOFF,
}
impl BOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFW::NOTBUSOFF => false,
            BOFFW::BUSOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CAN module is not in busoff"]
    #[inline]
    pub fn notbusoff(self) -> &'a mut W {
        self.variant(BOFFW::NOTBUSOFF)
    }
    #[doc = "The CAN controller is in busoff state"]
    #[inline]
    pub fn busoff(self) -> &'a mut W {
        self.variant(BOFFW::BUSOFF)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Last error code Type of the last error to occur on the CAN bus.The LEC field holds a code which indicates the type of the last error to occur on the CAN bus. This field will be cleared to 0 when a message has been transferred (reception or transmission) without error. The unused code 111 may be written by the CPU to check for updates"]
    #[inline]
    pub fn lec(&self) -> LECR {
        LECR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Transmitted a message successfully This bit is reset by the CPU. It is never reset by the CAN controller"]
    #[inline]
    pub fn txok(&self) -> TXOKR {
        TXOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Received a message successfully This bit is reset by the CPU. It is never reset by the CAN controller"]
    #[inline]
    pub fn rxok(&self) -> RXOKR {
        RXOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error passive"]
    #[inline]
    pub fn epass(&self) -> EPASSR {
        EPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Warning status"]
    #[inline]
    pub fn ewarn(&self) -> EWARNR {
        EWARNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Busoff status"]
    #[inline]
    pub fn boff(&self) -> BOFFR {
        BOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:2 - Last error code Type of the last error to occur on the CAN bus.The LEC field holds a code which indicates the type of the last error to occur on the CAN bus. This field will be cleared to 0 when a message has been transferred (reception or transmission) without error. The unused code 111 may be written by the CPU to check for updates"]
    #[inline]
    pub fn lec(&mut self) -> _LECW {
        _LECW { w: self }
    }
    #[doc = "Bit 3 - Transmitted a message successfully This bit is reset by the CPU. It is never reset by the CAN controller"]
    #[inline]
    pub fn txok(&mut self) -> _TXOKW {
        _TXOKW { w: self }
    }
    #[doc = "Bit 4 - Received a message successfully This bit is reset by the CPU. It is never reset by the CAN controller"]
    #[inline]
    pub fn rxok(&mut self) -> _RXOKW {
        _RXOKW { w: self }
    }
    #[doc = "Bit 5 - Error passive"]
    #[inline]
    pub fn epass(&mut self) -> _EPASSW {
        _EPASSW { w: self }
    }
    #[doc = "Bit 6 - Warning status"]
    #[inline]
    pub fn ewarn(&mut self) -> _EWARNW {
        _EWARNW { w: self }
    }
    #[doc = "Bit 7 - Busoff status"]
    #[inline]
    pub fn boff(&mut self) -> _BOFFW {
        _BOFFW { w: self }
    }
}
