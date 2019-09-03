#[doc = "Reader of register CANIF%s_CMDMSK_R"]
pub type R = crate::R<u32, super::CANIF_CMDMSK_R>;
#[doc = "Writer for register CANIF%s_CMDMSK_R"]
pub type W = crate::W<u32, super::CANIF_CMDMSK_R>;
#[doc = "Register CANIF%s_CMDMSK_R `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIF_CMDMSK_R {
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
    UNCHANGED,
    #[doc = "1: Transfer data bytes 4-7 to IFx message buffer register"]
    TRANSFER_DATA_BYTES_,
}
impl From<DATA_B_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_B_A) -> Self {
        match variant {
            DATA_B_A::UNCHANGED => false,
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
            false => DATA_B_A::UNCHANGED,
            true => DATA_B_A::TRANSFER_DATA_BYTES_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == DATA_B_A::UNCHANGED
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
    pub fn unchanged(self) -> &'a mut W {
        self.variant(DATA_B_A::UNCHANGED)
    }
    #[doc = "Transfer data bytes 4-7 to IFx message buffer register"]
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
    UNCHANGED,
    #[doc = "1: Transfer data bytes 0-3 to IFx message buffer"]
    TRANSFER_DATA_BYTES_,
}
impl From<DATA_A_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_A_A) -> Self {
        match variant {
            DATA_A_A::UNCHANGED => false,
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
            false => DATA_A_A::UNCHANGED,
            true => DATA_A_A::TRANSFER_DATA_BYTES_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == DATA_A_A::UNCHANGED
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
    pub fn unchanged(self) -> &'a mut W {
        self.variant(DATA_A_A::UNCHANGED)
    }
    #[doc = "Transfer data bytes 0-3 to IFx message buffer"]
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
#[doc = "Access new data bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEWDAT_A {
    #[doc = "0: NEWDAT bit remains unchanged. A read access to a message object can be combined with the reset of the control bits INTPND and NEWDAT in IF1/2_MCTRL. The values of these bits transferred to the IFx Message Control Register always reflect the status before resetting these bits"]
    UNCHANGED,
    #[doc = "1: Clear NEWDAT bit in the message object"]
    CLEAR_NEWDAT_BIT_IN_,
}
impl From<NEWDAT_A> for bool {
    #[inline(always)]
    fn from(variant: NEWDAT_A) -> Self {
        match variant {
            NEWDAT_A::UNCHANGED => false,
            NEWDAT_A::CLEAR_NEWDAT_BIT_IN_ => true,
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
            false => NEWDAT_A::UNCHANGED,
            true => NEWDAT_A::CLEAR_NEWDAT_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == NEWDAT_A::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `CLEAR_NEWDAT_BIT_IN_`"]
    #[inline(always)]
    pub fn is_clear_newdat_bit_in_(&self) -> bool {
        *self == NEWDAT_A::CLEAR_NEWDAT_BIT_IN_
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
    #[doc = "NEWDAT bit remains unchanged. A read access to a message object can be combined with the reset of the control bits INTPND and NEWDAT in IF1/2_MCTRL. The values of these bits transferred to the IFx Message Control Register always reflect the status before resetting these bits"]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(NEWDAT_A::UNCHANGED)
    }
    #[doc = "Clear NEWDAT bit in the message object"]
    #[inline(always)]
    pub fn clear_newdat_bit_in_(self) -> &'a mut W {
        self.variant(NEWDAT_A::CLEAR_NEWDAT_BIT_IN_)
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
#[doc = "Clear interrupt pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRINTPND_A {
    #[doc = "0: INTPND bit remains unchanged"]
    UNCHANGED,
    #[doc = "1: Clear INTPND bit in the message object"]
    CLEAR_INTPND_BIT_IN_,
}
impl From<CLRINTPND_A> for bool {
    #[inline(always)]
    fn from(variant: CLRINTPND_A) -> Self {
        match variant {
            CLRINTPND_A::UNCHANGED => false,
            CLRINTPND_A::CLEAR_INTPND_BIT_IN_ => true,
        }
    }
}
#[doc = "Reader of field `CLRINTPND`"]
pub type CLRINTPND_R = crate::R<bool, CLRINTPND_A>;
impl CLRINTPND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRINTPND_A {
        match self.bits {
            false => CLRINTPND_A::UNCHANGED,
            true => CLRINTPND_A::CLEAR_INTPND_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == CLRINTPND_A::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `CLEAR_INTPND_BIT_IN_`"]
    #[inline(always)]
    pub fn is_clear_intpnd_bit_in_(&self) -> bool {
        *self == CLRINTPND_A::CLEAR_INTPND_BIT_IN_
    }
}
#[doc = "Write proxy for field `CLRINTPND`"]
pub struct CLRINTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRINTPND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRINTPND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "INTPND bit remains unchanged"]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(CLRINTPND_A::UNCHANGED)
    }
    #[doc = "Clear INTPND bit in the message object"]
    #[inline(always)]
    pub fn clear_intpnd_bit_in_(self) -> &'a mut W {
        self.variant(CLRINTPND_A::CLEAR_INTPND_BIT_IN_)
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
#[doc = "Access control bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_A {
    #[doc = "0: Control bits unchanged"]
    UNCHANGED,
    #[doc = "1: Transfer control bits to IFx message buffer"]
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
    #[doc = "Transfer control bits to IFx message buffer"]
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
    #[doc = "1: Transfer Identifier, DIR, XTD, and MSGVAL bits to IFx message buffer register"]
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
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to IFx message buffer register"]
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
    #[doc = "1: Transfer Identifier MASK + MDIR + MXTD to IFx message buffer register"]
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
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to IFx message buffer register"]
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
    #[doc = "Bit 2 - Access new data bit."]
    #[inline(always)]
    pub fn newdat(&self) -> NEWDAT_R {
        NEWDAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear interrupt pending bit."]
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
    #[doc = "Bit 7 - Read transfer Transfer data from the message object addressed by the command request register to the selected message buffer registers CANIFn_CMDREQ."]
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
    #[doc = "Bit 2 - Access new data bit."]
    #[inline(always)]
    pub fn newdat(&mut self) -> NEWDAT_W {
        NEWDAT_W { w: self }
    }
    #[doc = "Bit 3 - Clear interrupt pending bit."]
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
    #[doc = "Bit 7 - Read transfer Transfer data from the message object addressed by the command request register to the selected message buffer registers CANIFn_CMDREQ."]
    #[inline(always)]
    pub fn wr_rd(&mut self) -> WR_RD_W {
        WR_RD_W { w: self }
    }
}
