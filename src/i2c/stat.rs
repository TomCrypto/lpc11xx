#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "These bits give the actual status information about the I2C interface.\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Bus error during an I2C serial transfer."]
    BUS_ERROR,
    #[doc = "1: A START condition has been transmitted."]
    START_CONDITION_TRANSMITTED,
    #[doc = "2: A Repeated START condition has been transmitted."]
    REPEATED_START_CONDITION_TRANSMITTED,
    #[doc = "3: SLA+W has been transmitted; ACK has been received."]
    WRITE_ADDRESS_TRANSMITTED_ACK,
    #[doc = "4: SLA+W has been transmitted; NOT ACK has been received."]
    WRITE_ADDRESS_TRANSMITTED_NOT_ACK,
    #[doc = "5: Data byte in DAT has been transmitted; ACK has been received."]
    DATA_BYTE_TRANSMITTED_ACK,
    #[doc = "6: Data byte in DAT has been transmitted; NOT ACK has been received."]
    DATA_BYTE_TRANSMITTED_NOT_ACK,
    #[doc = "7: In Master Transmitter mode, arbitration lost in SLA+R/W or Data bytes. In Master Receiver mode, arbitration lost in NOT ACK bit."]
    ARBITRATION_LOST,
    #[doc = "8: SLA+R has been transmitted; ACK has been received."]
    READ_ADDRESS_TRANSMITTED_ACK,
    #[doc = "9: SLA+R has been transmitted; NOT ACK has been received."]
    READ_ADDRESS_TRANSMITTED_NOT_ACK,
    #[doc = "10: Data byte has been received; ACK has been returned."]
    DATA_BYTE_RECEIVED_ACK,
    #[doc = "11: Data byte has been received; NOT ACK has been returned."]
    DATA_BYTE_RECEIVED_NOT_ACK,
    #[doc = "12: Own SLA+W has been received; ACK has been returned."]
    OWN_WRITE_ADDRESS_RECEIVED_ACK,
    #[doc = "13: Arbitration lost in SLA+R/W as master; Own SLA+W has been received, ACK returned."]
    ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK,
    #[doc = "14: General call address (0x00) has been received; ACK has been returned."]
    GENERAL_CALL_RECEIVED_ACK,
    #[doc = "15: Arbitration lost in SLA+R/W as master; General call address has been received; ACK has been returned."]
    ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK,
    #[doc = "16: Previously addressed with own SLV address; Data byte has been received; ACK has been returned."]
    OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK,
    #[doc = "17: Previously addressed with own SLV address; Data byte has been received; NOT ACK has been returned."]
    OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK,
    #[doc = "18: Previously addressed with General Call; Data byte has been received; ACK has been returned."]
    GENERAL_CALL_DATA_BYTE_RECEIVED_ACK,
    #[doc = "19: Previously addressed with General Call; Data byte has been received; NOT ACK has been returned."]
    GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK,
    #[doc = "20: A STOP condition or Repeated START condition has been received while still addressed as SLV/REC or SLV/TRX."]
    STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED,
    #[doc = "21: Own SLA+R has been received; ACK has been returned."]
    OWN_READ_ADDRESS_ACK,
    #[doc = "22: Arbitration lost in SLA+R/W as master; Own SLA+R has been received; ACK has been returned."]
    ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK,
    #[doc = "23: Data byte in DAT has been transmitted; ACK has been received."]
    SLAVE_DATA_BYTE_TRANSMITTED_ACK,
    #[doc = "24: Data byte in DAT has been transmitted; NOT ACK has been received."]
    SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK,
    #[doc = "25: Last data byte in DAT has been transmitted (AA = 0); ACK has been received."]
    LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED,
    #[doc = "31: There is no information available yet."]
    NOT_AVAILABLE,
}
impl From<STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        match variant {
            STATUS_A::BUS_ERROR => 0,
            STATUS_A::START_CONDITION_TRANSMITTED => 1,
            STATUS_A::REPEATED_START_CONDITION_TRANSMITTED => 2,
            STATUS_A::WRITE_ADDRESS_TRANSMITTED_ACK => 3,
            STATUS_A::WRITE_ADDRESS_TRANSMITTED_NOT_ACK => 4,
            STATUS_A::DATA_BYTE_TRANSMITTED_ACK => 5,
            STATUS_A::DATA_BYTE_TRANSMITTED_NOT_ACK => 6,
            STATUS_A::ARBITRATION_LOST => 7,
            STATUS_A::READ_ADDRESS_TRANSMITTED_ACK => 8,
            STATUS_A::READ_ADDRESS_TRANSMITTED_NOT_ACK => 9,
            STATUS_A::DATA_BYTE_RECEIVED_ACK => 10,
            STATUS_A::DATA_BYTE_RECEIVED_NOT_ACK => 11,
            STATUS_A::OWN_WRITE_ADDRESS_RECEIVED_ACK => 12,
            STATUS_A::ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK => 13,
            STATUS_A::GENERAL_CALL_RECEIVED_ACK => 14,
            STATUS_A::ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK => 15,
            STATUS_A::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK => 16,
            STATUS_A::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK => 17,
            STATUS_A::GENERAL_CALL_DATA_BYTE_RECEIVED_ACK => 18,
            STATUS_A::GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK => 19,
            STATUS_A::STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED => 20,
            STATUS_A::OWN_READ_ADDRESS_ACK => 21,
            STATUS_A::ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK => 22,
            STATUS_A::SLAVE_DATA_BYTE_TRANSMITTED_ACK => 23,
            STATUS_A::SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK => 24,
            STATUS_A::LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED => 25,
            STATUS_A::NOT_AVAILABLE => 31,
        }
    }
}
#[doc = "Reader of field `Status`"]
pub type STATUS_R = crate::R<u8, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATUS_A::BUS_ERROR),
            1 => Val(STATUS_A::START_CONDITION_TRANSMITTED),
            2 => Val(STATUS_A::REPEATED_START_CONDITION_TRANSMITTED),
            3 => Val(STATUS_A::WRITE_ADDRESS_TRANSMITTED_ACK),
            4 => Val(STATUS_A::WRITE_ADDRESS_TRANSMITTED_NOT_ACK),
            5 => Val(STATUS_A::DATA_BYTE_TRANSMITTED_ACK),
            6 => Val(STATUS_A::DATA_BYTE_TRANSMITTED_NOT_ACK),
            7 => Val(STATUS_A::ARBITRATION_LOST),
            8 => Val(STATUS_A::READ_ADDRESS_TRANSMITTED_ACK),
            9 => Val(STATUS_A::READ_ADDRESS_TRANSMITTED_NOT_ACK),
            10 => Val(STATUS_A::DATA_BYTE_RECEIVED_ACK),
            11 => Val(STATUS_A::DATA_BYTE_RECEIVED_NOT_ACK),
            12 => Val(STATUS_A::OWN_WRITE_ADDRESS_RECEIVED_ACK),
            13 => Val(STATUS_A::ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK),
            14 => Val(STATUS_A::GENERAL_CALL_RECEIVED_ACK),
            15 => Val(STATUS_A::ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK),
            16 => Val(STATUS_A::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK),
            17 => Val(STATUS_A::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK),
            18 => Val(STATUS_A::GENERAL_CALL_DATA_BYTE_RECEIVED_ACK),
            19 => Val(STATUS_A::GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK),
            20 => Val(STATUS_A::STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED),
            21 => Val(STATUS_A::OWN_READ_ADDRESS_ACK),
            22 => Val(STATUS_A::ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK),
            23 => Val(STATUS_A::SLAVE_DATA_BYTE_TRANSMITTED_ACK),
            24 => Val(STATUS_A::SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK),
            25 => Val(STATUS_A::LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED),
            31 => Val(STATUS_A::NOT_AVAILABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline(always)]
    pub fn is_bus_error(&self) -> bool {
        *self == STATUS_A::BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `START_CONDITION_TRANSMITTED`"]
    #[inline(always)]
    pub fn is_start_condition_transmitted(&self) -> bool {
        *self == STATUS_A::START_CONDITION_TRANSMITTED
    }
    #[doc = "Checks if the value of the field is `REPEATED_START_CONDITION_TRANSMITTED`"]
    #[inline(always)]
    pub fn is_repeated_start_condition_transmitted(&self) -> bool {
        *self == STATUS_A::REPEATED_START_CONDITION_TRANSMITTED
    }
    #[doc = "Checks if the value of the field is `WRITE_ADDRESS_TRANSMITTED_ACK`"]
    #[inline(always)]
    pub fn is_write_address_transmitted_ack(&self) -> bool {
        *self == STATUS_A::WRITE_ADDRESS_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `WRITE_ADDRESS_TRANSMITTED_NOT_ACK`"]
    #[inline(always)]
    pub fn is_write_address_transmitted_not_ack(&self) -> bool {
        *self == STATUS_A::WRITE_ADDRESS_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_TRANSMITTED_ACK`"]
    #[inline(always)]
    pub fn is_data_byte_transmitted_ack(&self) -> bool {
        *self == STATUS_A::DATA_BYTE_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_TRANSMITTED_NOT_ACK`"]
    #[inline(always)]
    pub fn is_data_byte_transmitted_not_ack(&self) -> bool {
        *self == STATUS_A::DATA_BYTE_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST`"]
    #[inline(always)]
    pub fn is_arbitration_lost(&self) -> bool {
        *self == STATUS_A::ARBITRATION_LOST
    }
    #[doc = "Checks if the value of the field is `READ_ADDRESS_TRANSMITTED_ACK`"]
    #[inline(always)]
    pub fn is_read_address_transmitted_ack(&self) -> bool {
        *self == STATUS_A::READ_ADDRESS_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `READ_ADDRESS_TRANSMITTED_NOT_ACK`"]
    #[inline(always)]
    pub fn is_read_address_transmitted_not_ack(&self) -> bool {
        *self == STATUS_A::READ_ADDRESS_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_data_byte_received_ack(&self) -> bool {
        *self == STATUS_A::DATA_BYTE_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `DATA_BYTE_RECEIVED_NOT_ACK`"]
    #[inline(always)]
    pub fn is_data_byte_received_not_ack(&self) -> bool {
        *self == STATUS_A::DATA_BYTE_RECEIVED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `OWN_WRITE_ADDRESS_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_own_write_address_received_ack(&self) -> bool {
        *self == STATUS_A::OWN_WRITE_ADDRESS_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_arbitration_lost_own_write_address_received_ack(&self) -> bool {
        *self == STATUS_A::ARBITRATION_LOST_OWN_WRITE_ADDRESS_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_general_call_received_ack(&self) -> bool {
        *self == STATUS_A::GENERAL_CALL_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_arbitration_lost_general_call_received_ack(&self) -> bool {
        *self == STATUS_A::ARBITRATION_LOST_GENERAL_CALL_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_own_write_address_data_byte_received_ack(&self) -> bool {
        *self == STATUS_A::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK`"]
    #[inline(always)]
    pub fn is_own_write_address_data_byte_received_not_ack(&self) -> bool {
        *self == STATUS_A::OWN_WRITE_ADDRESS_DATA_BYTE_RECEIVED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL_DATA_BYTE_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_general_call_data_byte_received_ack(&self) -> bool {
        *self == STATUS_A::GENERAL_CALL_DATA_BYTE_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK`"]
    #[inline(always)]
    pub fn is_general_call_data_byte_received_not_ack(&self) -> bool {
        *self == STATUS_A::GENERAL_CALL_DATA_BYTE_RECEIVED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED`"]
    #[inline(always)]
    pub fn is_stop_or_repeated_start_condition_received_while_addressed(&self) -> bool {
        *self == STATUS_A::STOP_OR_REPEATED_START_CONDITION_RECEIVED_WHILE_ADDRESSED
    }
    #[doc = "Checks if the value of the field is `OWN_READ_ADDRESS_ACK`"]
    #[inline(always)]
    pub fn is_own_read_address_ack(&self) -> bool {
        *self == STATUS_A::OWN_READ_ADDRESS_ACK
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK`"]
    #[inline(always)]
    pub fn is_arbitration_lost_own_read_address_received_ack(&self) -> bool {
        *self == STATUS_A::ARBITRATION_LOST_OWN_READ_ADDRESS_RECEIVED_ACK
    }
    #[doc = "Checks if the value of the field is `SLAVE_DATA_BYTE_TRANSMITTED_ACK`"]
    #[inline(always)]
    pub fn is_slave_data_byte_transmitted_ack(&self) -> bool {
        *self == STATUS_A::SLAVE_DATA_BYTE_TRANSMITTED_ACK
    }
    #[doc = "Checks if the value of the field is `SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK`"]
    #[inline(always)]
    pub fn is_slave_data_byte_transmitted_not_ack(&self) -> bool {
        *self == STATUS_A::SLAVE_DATA_BYTE_TRANSMITTED_NOT_ACK
    }
    #[doc = "Checks if the value of the field is `LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED`"]
    #[inline(always)]
    pub fn is_last_data_byte_transmitted_ack_received(&self) -> bool {
        *self == STATUS_A::LAST_DATA_BYTE_TRANSMITTED_ACK_RECEIVED
    }
    #[doc = "Checks if the value of the field is `NOT_AVAILABLE`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == STATUS_A::NOT_AVAILABLE
    }
}
impl R {
    #[doc = "Bits 3:7 - These bits give the actual status information about the I2C interface."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
