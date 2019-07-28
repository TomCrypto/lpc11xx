#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `Status`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUSR {
    #[doc = "Bus error during an I2C serial transfer."]
    BUS_ERROR,
    #[doc = "A START condition has been transmitted."]
    START_CONDITION_TRANSMITTED,
    #[doc = "A Repeated START condition has been transmitted."]
    REPEATED_START_CONDITION_TRANSMITTED,
    #[doc = "SLA+W has been transmitted; ACK has been received."]
    WRITE_ADDRESS_TRANSMITTED_ACK,
    #[doc = "SLA+W has been transmitted; NOT ACK has been received."]
    WRITE_ADDRESS_TRANSMITTED_NOT_ACK,
    #[doc = "Data byte in DAT has been transmitted; ACK has been received."]
    DATA_BYTE_TRANSMITTED_ACK,
    #[doc = "Data byte in DAT has been transmitted; NOT ACK has been received."]
    DATA_BYTE_TRANSMITTED_NOT_ACK,
    #[doc = "In Master Transmitter mode, arbitration lost in SLA+R/W or Data bytes. In Master Receiver mode, arbitration lost in NOT ACK bit."]
    ARBITRATION_LOST,
    #[doc = "SLA+R has been transmitted; ACK has been received."]
    READ_ADDRESS_TRANSMITTED_ACK,
    #[doc = "SLA+R has been transmitted; NOT ACK has been received."]
    READ_ADDRESS_TRANSMITTED_NOT_ACK,
    #[doc = "Data byte has been received; ACK has been returned."]
    DATA_BYTE_RECEIVED_ACK,
    #[doc = "Data byte has been received; NOT ACK has been returned."]
    DATA_BYTE_RECEIVED_NOT_ACK,
    #[doc = "Own SLA+W has been received; ACK has been returned."]
    OWN_WRITE_ADDRESS_RECEIVED_ACK,
    #[doc = "Arbitration lost in SLA+R/W as master; Own SLA+W has been received, ACK returned."]
    ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK,
    #[doc = "General call address (0x00) has been received; ACK has been returned."]
    GENERAL_CALL_RECEIVED_ACK,
    #[doc = "Arbitration lost in SLA+R/W as master; General call address has been received; ACK has been returned."]
    ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK,
    #[doc = "Previously addressed with own SLV address; Data byte has been received; ACK has been returned."]
    OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK,
    #[doc = "Previously addressed with own SLV address; Data byte has been received; NOT ACK has been returned."]
    OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK,
    #[doc = "Previously addressed with General Call; Data byte has been received; ACK has been returned."]
    GENERAL_CALL_DATA_BYTE_RECEIVED_ACK,
    #[doc = "Previously addressed with General Call; Data byte has been received; NOT ACK has been returned."]
    GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK,
    #[doc = "A STOP condition or Repeated START condition has been received while still addressed as SLV/REC or SLV/TRX."]
    STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED,
    #[doc = "Own SLA+R has been received; ACK has been returned."]
    OWN_READ_ADDRESS_ACK,
    #[doc = "Arbitration lost in SLA+R/W as master; Own SLA+R has been received; ACK has been returned."]
    ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK,
    #[doc = "Data byte in DAT has been transmitted; ACK has been received."]
    SLAVE_DATA_BYTE_TRANSMITTED_ACK,
    #[doc = "Data byte in DAT has been transmitted; NOT ACK has been received."]
    SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK,
    #[doc = "Last data byte in DAT has been transmitted (AA = 0); ACK has been received."]
    LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED,
    #[doc = "There is no information available yet."]
    NOT_AVAILABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATUSR::BUS_ERROR => 0,
            STATUSR::START_CONDITION_TRANSMITTED => 1,
            STATUSR::REPEATED_START_CONDITION_TRANSMITTED => 2,
            STATUSR::WRITE_ADDRESS_TRANSMITTED_ACK => 3,
            STATUSR::WRITE_ADDRESS_TRANSMITTED_NOT_ACK => 4,
            STATUSR::DATA_BYTE_TRANSMITTED_ACK => 5,
            STATUSR::DATA_BYTE_TRANSMITTED_NOT_ACK => 6,
            STATUSR::ARBITRATION_LOST => 7,
            STATUSR::READ_ADDRESS_TRANSMITTED_ACK => 8,
            STATUSR::READ_ADDRESS_TRANSMITTED_NOT_ACK => 9,
            STATUSR::DATA_BYTE_RECEIVED_ACK => 10,
            STATUSR::DATA_BYTE_RECEIVED_NOT_ACK => 11,
            STATUSR::OWN_WRITE_ADDRESS_RECEIVED_ACK => 12,
            STATUSR::ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK => 13,
            STATUSR::GENERAL_CALL_RECEIVED_ACK => 14,
            STATUSR::ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK => 15,
            STATUSR::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK => 16,
            STATUSR::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK => 17,
            STATUSR::GENERAL_CALL_DATA_BYTE_RECEIVED_ACK => 18,
            STATUSR::GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK => 19,
            STATUSR::STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED => 20,
            STATUSR::OWN_READ_ADDRESS_ACK => 21,
            STATUSR::ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK => 22,
            STATUSR::SLAVE_DATA_BYTE_TRANSMITTED_ACK => 23,
            STATUSR::SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK => 24,
            STATUSR::LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED => 25,
            STATUSR::NOT_AVAILABLE => 31,
            STATUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATUSR {
        match value {
            0 => STATUSR::BUS_ERROR,
            1 => STATUSR::START_CONDITION_TRANSMITTED,
            2 => STATUSR::REPEATED_START_CONDITION_TRANSMITTED,
            3 => STATUSR::WRITE_ADDRESS_TRANSMITTED_ACK,
            4 => STATUSR::WRITE_ADDRESS_TRANSMITTED_NOT_ACK,
            5 => STATUSR::DATA_BYTE_TRANSMITTED_ACK,
            6 => STATUSR::DATA_BYTE_TRANSMITTED_NOT_ACK,
            7 => STATUSR::ARBITRATION_LOST,
            8 => STATUSR::READ_ADDRESS_TRANSMITTED_ACK,
            9 => STATUSR::READ_ADDRESS_TRANSMITTED_NOT_ACK,
            10 => STATUSR::DATA_BYTE_RECEIVED_ACK,
            11 => STATUSR::DATA_BYTE_RECEIVED_NOT_ACK,
            12 => STATUSR::OWN_WRITE_ADDRESS_RECEIVED_ACK,
            13 => STATUSR::ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK,
            14 => STATUSR::GENERAL_CALL_RECEIVED_ACK,
            15 => STATUSR::ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK,
            16 => STATUSR::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK,
            17 => STATUSR::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK,
            18 => STATUSR::GENERAL_CALL_DATA_BYTE_RECEIVED_ACK,
            19 => STATUSR::GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK,
            20 => STATUSR::STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED,
            21 => STATUSR::OWN_READ_ADDRESS_ACK,
            22 => STATUSR::ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK,
            23 => STATUSR::SLAVE_DATA_BYTE_TRANSMITTED_ACK,
            24 => STATUSR::SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK,
            25 => STATUSR::LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED,
            31 => STATUSR::NOT_AVAILABLE,
            i => STATUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == STATUSR::BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `START_CONDITION_TRANSMITTED`"]
    #[inline]
    pub fn is_start_condition_transmitted(&self) -> bool {
        *self == STATUSR::START_CONDITION_TRANSMITTED
    }
    #[doc = "Checks if the value of the field is `REPEATED_START_CONDITION_TRANSMITTED`"]
    #[inline]
    pub fn is_repeated_start_condition_transmitted(&self) -> bool {
        *self == STATUSR::REPEATED_START_CONDITION_TRANSMITTED
    }
    #[doc = "Checks if the value of the field is `WRITE_ADDRESS_TRANSMITTED_ACK`"]
    #[inline]
    pub fn is_write_address_transmitted_ack(&self) -> bool {
        *self == STATUSR::WRITE_ADDRESS_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `WRITE_ADDRESS_TRANSMITTED_NOT_ACK`"]
    #[inline]
    pub fn is_write_address_transmitted_not_ack(&self) -> bool {
        *self == STATUSR::WRITE_ADDRESS_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_TRANSMITTED_ACK`"]
    #[inline]
    pub fn is_data_byte_transmitted_ack(&self) -> bool {
        *self == STATUSR::DATA_BYTE_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_TRANSMITTED_NOT_ACK`"]
    #[inline]
    pub fn is_data_byte_transmitted_not_ack(&self) -> bool {
        *self == STATUSR::DATA_BYTE_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST`"]
    #[inline]
    pub fn is_arbitration_lost(&self) -> bool {
        *self == STATUSR::ARBITRATION_LOST
    }
    #[doc = "Checks if the value of the field is `READ_ADDRESS_TRANSMITTED_ACK`"]
    #[inline]
    pub fn is_read_address_transmitted_ack(&self) -> bool {
        *self == STATUSR::READ_ADDRESS_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `READ_ADDRESS_TRANSMITTED_NOT_ACK`"]
    #[inline]
    pub fn is_read_address_transmitted_not_ack(&self) -> bool {
        *self == STATUSR::READ_ADDRESS_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_RECEIVED_ACK`"]
    #[inline]
    pub fn is_data_byte_received_ack(&self) -> bool {
        *self == STATUSR::DATA_BYTE_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_RECEIVED_NOT_ACK`"]
    #[inline]
    pub fn is_data_byte_received_not_ack(&self) -> bool {
        *self == STATUSR::DATA_BYTE_RECEIVED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `OWN_WRITE_ADDRESS_RECEIVED_ACK`"]
    #[inline]
    pub fn is_own_write_address_received_ack(&self) -> bool {
        *self == STATUSR::OWN_WRITE_ADDRESS_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK`"]
    #[inline]
    pub fn is_arbitration_lost_own_write_address_received_ack(&self) -> bool {
        *self == STATUSR::ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL_RECEIVED_ACK`"]
    #[inline]
    pub fn is_general_call_received_ack(&self) -> bool {
        *self == STATUSR::GENERAL_CALL_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK`"]
    #[inline]
    pub fn is_arbitration_lost_general_call_received_ack(&self) -> bool {
        *self == STATUSR::ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK`"]
    #[inline]
    pub fn is_own_write_address_data_byte_received_ack(&self) -> bool {
        *self == STATUSR::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK`"]
    #[inline]
    pub fn is_own_write_address_data_byte_received_not_ack(&self) -> bool {
        *self == STATUSR::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL_DATA_BYTE_RECEIVED_ACK`"]
    #[inline]
    pub fn is_general_call_data_byte_received_ack(&self) -> bool {
        *self == STATUSR::GENERAL_CALL_DATA_BYTE_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK`"]
    #[inline]
    pub fn is_general_call_data_byte_received_not_ack(&self) -> bool {
        *self == STATUSR::GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED`"]
    #[inline]
    pub fn is_stop_or_repeated_start_condition_received_while_addressed(&self) -> bool {
        *self == STATUSR::STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED
    }
    #[doc = "Checks if the value of the field is `OWN_READ_ADDRESS_ACK`"]
    #[inline]
    pub fn is_own_read_address_ack(&self) -> bool {
        *self == STATUSR::OWN_READ_ADDRESS_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK`"]
    #[inline]
    pub fn is_arbitration_lost_own_read_address_received_ack(&self) -> bool {
        *self == STATUSR::ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `SLAVE_DATA_BYTE_TRANSMITTED_ACK`"]
    #[inline]
    pub fn is_slave_data_byte_transmitted_ack(&self) -> bool {
        *self == STATUSR::SLAVE_DATA_BYTE_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK`"]
    #[inline]
    pub fn is_slave_data_byte_transmitted_not_ack(&self) -> bool {
        *self == STATUSR::SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED`"]
    #[inline]
    pub fn is_last_data_byte_transmitted_ack_received(&self) -> bool {
        *self == STATUSR::LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED
    }
    #[doc = "Checks if the value of the field is `NOT_AVAILABLE`"]
    #[inline]
    pub fn is_not_available(&self) -> bool {
        *self == STATUSR::NOT_AVAILABLE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 3:7 - These bits give the actual status information about the I2C interface."]
    #[inline]
    pub fn status(&self) -> STATUSR {
        STATUSR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
