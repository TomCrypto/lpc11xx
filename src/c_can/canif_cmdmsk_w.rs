#[doc = "Reader of register CANIF%s_CMDMSK_W"]
pub type R = crate::R<u32, super::CANIF_CMDMSK_W>;
#[doc = "Writer for register CANIF%s_CMDMSK_W"]
pub type W = crate::W<u32, super::CANIF_CMDMSK_W>;
#[doc = "Register CANIF%s_CMDMSK_W `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIF_CMDMSK_W {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access data bytes 4-7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_B_A {
    #[doc = "0: Data bytes 4-7 unchanged"]
    DATA_BYTES_4_7_UNCHA,
    #[doc = "1: Transfer data bytes 4-7 to message object"]
    TRANSFER_DATA_BYTES_,
}
impl From<DATA_B_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_B_A) -> Self {
        match variant {
            DATA_B_A::DATA_BYTES_4_7_UNCHA => false,
            DATA_B_A::TRANSFER_DATA_BYTES_ => true,
        }
    }
}
#[doc = "Reader of field `DATA_B`"]
pub type DATA_B_R = crate::R<bool, DATA_B_A>;
impl DATA_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_B_A {
        match self.bits {
            false => DATA_B_A::DATA_BYTES_4_7_UNCHA,
            true => DATA_B_A::TRANSFER_DATA_BYTES_,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_BYTES_4_7_UNCHA`"]
    #[inline(always)]
    pub fn is_data_bytes_4_7_uncha(&self) -> bool {
        *self == DATA_B_A::DATA_BYTES_4_7_UNCHA
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DATA_BYTES_`"]
    #[inline(always)]
    pub fn is_transfer_data_bytes_(&self) -> bool {
        *self == DATA_B_A::TRANSFER_DATA_BYTES_
    }
}
#[doc = "Write proxy for field `DATA_B`"]
pub struct DATA_B_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data bytes 4-7 unchanged"]
    #[inline(always)]
    pub fn data_bytes_4_7_uncha(self) -> &'a mut W {
        self.variant(DATA_B_A::DATA_BYTES_4_7_UNCHA)
    }
    #[doc = "Transfer data bytes 4-7 to message object"]
    #[inline(always)]
    pub fn transfer_data_bytes_(self) -> &'a mut W {
        self.variant(DATA_B_A::TRANSFER_DATA_BYTES_)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Access data bytes 0-3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_A_A {
    #[doc = "0: Data bytes 0-3 unchanged"]
    DATA_BYTES_0_3_UNCHA,
    #[doc = "1: Transfer data bytes 0-3 to message object"]
    TRANSFER_DATA_BYTES_,
}
impl From<DATA_A_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_A_A) -> Self {
        match variant {
            DATA_A_A::DATA_BYTES_0_3_UNCHA => false,
            DATA_A_A::TRANSFER_DATA_BYTES_ => true,
        }
    }
}
#[doc = "Reader of field `DATA_A`"]
pub type DATA_A_R = crate::R<bool, DATA_A_A>;
impl DATA_A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_A_A {
        match self.bits {
            false => DATA_A_A::DATA_BYTES_0_3_UNCHA,
            true => DATA_A_A::TRANSFER_DATA_BYTES_,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_BYTES_0_3_UNCHA`"]
    #[inline(always)]
    pub fn is_data_bytes_0_3_uncha(&self) -> bool {
        *self == DATA_A_A::DATA_BYTES_0_3_UNCHA
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DATA_BYTES_`"]
    #[inline(always)]
    pub fn is_transfer_data_bytes_(&self) -> bool {
        *self == DATA_A_A::TRANSFER_DATA_BYTES_
    }
}
#[doc = "Write proxy for field `DATA_A`"]
pub struct DATA_A_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data bytes 0-3 unchanged"]
    #[inline(always)]
    pub fn data_bytes_0_3_uncha(self) -> &'a mut W {
        self.variant(DATA_A_A::DATA_BYTES_0_3_UNCHA)
    }
    #[doc = "Transfer data bytes 0-3 to message object"]
    #[inline(always)]
    pub fn transfer_data_bytes_(self) -> &'a mut W {
        self.variant(DATA_A_A::TRANSFER_DATA_BYTES_)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Access transmission request bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQST_A {
    #[doc = "0: No transmission request. TXRQSRT bit unchanged in IF1/2_MCTRL. If a transmission is requested by programming this bit, the TXRQST bit in the CANIFn_MCTRL register is ignored"]
    NO_TRANSMISSION_REQU,
    #[doc = "1: Request a transmission. Set the TXRQST bit IF1/2_MCTRL"]
    REQUEST_A_TRANSMISSI,
}
impl From<TXRQST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRQST_A) -> Self {
        match variant {
            TXRQST_A::NO_TRANSMISSION_REQU => false,
            TXRQST_A::REQUEST_A_TRANSMISSI => true,
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
            false => TXRQST_A::NO_TRANSMISSION_REQU,
            true => TXRQST_A::REQUEST_A_TRANSMISSI,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRANSMISSION_REQU`"]
    #[inline(always)]
    pub fn is_no_transmission_requ(&self) -> bool {
        *self == TXRQST_A::NO_TRANSMISSION_REQU
    }
    #[doc = "Checks if the value of the field is `REQUEST_A_TRANSMISSI`"]
    #[inline(always)]
    pub fn is_request_a_transmissi(&self) -> bool {
        *self == TXRQST_A::REQUEST_A_TRANSMISSI
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
    #[doc = "No transmission request. TXRQSRT bit unchanged in IF1/2_MCTRL. If a transmission is requested by programming this bit, the TXRQST bit in the CANIFn_MCTRL register is ignored"]
    #[inline(always)]
    pub fn no_transmission_requ(self) -> &'a mut W {
        self.variant(TXRQST_A::NO_TRANSMISSION_REQU)
    }
    #[doc = "Request a transmission. Set the TXRQST bit IF1/2_MCTRL"]
    #[inline(always)]
    pub fn request_a_transmissi(self) -> &'a mut W {
        self.variant(TXRQST_A::REQUEST_A_TRANSMISSI)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLRINTPND`"]
pub type CLRINTPND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRINTPND`"]
pub struct CLRINTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRINTPND_W<'a> {
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
#[doc = "Access control bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_A {
    #[doc = "0: Control bits unchanged"]
    UNCHANGED,
    #[doc = "1: Transfer control bits to message object"]
    TRANSFER_CONTROL_BIT,
}
impl From<CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_A) -> Self {
        match variant {
            CTRL_A::UNCHANGED => false,
            CTRL_A::TRANSFER_CONTROL_BIT => true,
        }
    }
}
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<bool, CTRL_A>;
impl CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_A {
        match self.bits {
            false => CTRL_A::UNCHANGED,
            true => CTRL_A::TRANSFER_CONTROL_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == CTRL_A::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_CONTROL_BIT`"]
    #[inline(always)]
    pub fn is_transfer_control_bit(&self) -> bool {
        *self == CTRL_A::TRANSFER_CONTROL_BIT
    }
}
#[doc = "Write proxy for field `CTRL`"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Control bits unchanged"]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(CTRL_A::UNCHANGED)
    }
    #[doc = "Transfer control bits to message object"]
    #[inline(always)]
    pub fn transfer_control_bit(self) -> &'a mut W {
        self.variant(CTRL_A::TRANSFER_CONTROL_BIT)
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
#[doc = "Access arbitration bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_A {
    #[doc = "0: Arbitration bits unchanged"]
    UNCHANGED,
    #[doc = "1: Transfer Identifier, DIR, XTD, and MSGVAL bits to message object"]
    TRANSFER_IDENTIFIER,
}
impl From<ARB_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_A) -> Self {
        match variant {
            ARB_A::UNCHANGED => false,
            ARB_A::TRANSFER_IDENTIFIER => true,
        }
    }
}
#[doc = "Reader of field `ARB`"]
pub type ARB_R = crate::R<bool, ARB_A>;
impl ARB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_A {
        match self.bits {
            false => ARB_A::UNCHANGED,
            true => ARB_A::TRANSFER_IDENTIFIER,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == ARB_A::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_IDENTIFIER`"]
    #[inline(always)]
    pub fn is_transfer_identifier(&self) -> bool {
        *self == ARB_A::TRANSFER_IDENTIFIER
    }
}
#[doc = "Write proxy for field `ARB`"]
pub struct ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Arbitration bits unchanged"]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(ARB_A::UNCHANGED)
    }
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to message object"]
    #[inline(always)]
    pub fn transfer_identifier(self) -> &'a mut W {
        self.variant(ARB_A::TRANSFER_IDENTIFIER)
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
#[doc = "Access mask bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_A {
    #[doc = "0: Mask bits unchanged"]
    UNCHANGED,
    #[doc = "1: Transfer Identifier MASK + MDIR + MXTD to message object"]
    TRANSFER_IDENTIFIER_,
}
impl From<MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_A) -> Self {
        match variant {
            MASK_A::UNCHANGED => false,
            MASK_A::TRANSFER_IDENTIFIER_ => true,
        }
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<bool, MASK_A>;
impl MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_A {
        match self.bits {
            false => MASK_A::UNCHANGED,
            true => MASK_A::TRANSFER_IDENTIFIER_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == MASK_A::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_IDENTIFIER_`"]
    #[inline(always)]
    pub fn is_transfer_identifier_(&self) -> bool {
        *self == MASK_A::TRANSFER_IDENTIFIER_
    }
}
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mask bits unchanged"]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(MASK_A::UNCHANGED)
    }
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to message object"]
    #[inline(always)]
    pub fn transfer_identifier_(self) -> &'a mut W {
        self.variant(MASK_A::TRANSFER_IDENTIFIER_)
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
#[doc = "Reader of field `WR_RD`"]
pub type WR_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_RD`"]
pub struct WR_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_RD_W<'a> {
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
    #[doc = "Bit 0 - Access data bytes 4-7."]
    #[inline(always)]
    pub fn data_b(&self) -> DATA_B_R {
        DATA_B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Access data bytes 0-3."]
    #[inline(always)]
    pub fn data_a(&self) -> DATA_A_R {
        DATA_A_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Access transmission request bit."]
    #[inline(always)]
    pub fn txrqst(&self) -> TXRQST_R {
        TXRQST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is ignored in the write direction."]
    #[inline(always)]
    pub fn clrintpnd(&self) -> CLRINTPND_R {
        CLRINTPND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Access control bits."]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Access arbitration bits."]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Access mask bits."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write transfer Transfer data from the selected message buffer registers to the message object addressed by the command request register CANIFn_CMDREQ."]
    #[inline(always)]
    pub fn wr_rd(&self) -> WR_RD_R {
        WR_RD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access data bytes 4-7."]
    #[inline(always)]
    pub fn data_b(&mut self) -> DATA_B_W {
        DATA_B_W { w: self }
    }
    #[doc = "Bit 1 - Access data bytes 0-3."]
    #[inline(always)]
    pub fn data_a(&mut self) -> DATA_A_W {
        DATA_A_W { w: self }
    }
    #[doc = "Bit 2 - Access transmission request bit."]
    #[inline(always)]
    pub fn txrqst(&mut self) -> TXRQST_W {
        TXRQST_W { w: self }
    }
    #[doc = "Bit 3 - This bit is ignored in the write direction."]
    #[inline(always)]
    pub fn clrintpnd(&mut self) -> CLRINTPND_W {
        CLRINTPND_W { w: self }
    }
    #[doc = "Bit 4 - Access control bits."]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
    #[doc = "Bit 5 - Access arbitration bits."]
    #[inline(always)]
    pub fn arb(&mut self) -> ARB_W {
        ARB_W { w: self }
    }
    #[doc = "Bit 6 - Access mask bits."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bit 7 - Write transfer Transfer data from the selected message buffer registers to the message object addressed by the command request register CANIFn_CMDREQ."]
    #[inline(always)]
    pub fn wr_rd(&mut self) -> WR_RD_W {
        WR_RD_W { w: self }
    }
}
