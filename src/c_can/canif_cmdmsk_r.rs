#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANIF_CMDMSK_R {
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
#[doc = "Possible values of the field `DATA_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_BR {
    #[doc = "Data bytes 4-7 unchanged"]
    UNCHANGED,
    #[doc = "Transfer data bytes 4-7 to IFx message buffer register"]
    TRANSFER_DATA_BYTES_,
}
impl DATA_BR {
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
            DATA_BR::UNCHANGED => false,
            DATA_BR::TRANSFER_DATA_BYTES_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_BR {
        match value {
            false => DATA_BR::UNCHANGED,
            true => DATA_BR::TRANSFER_DATA_BYTES_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == DATA_BR::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DATA_BYTES_`"]
    #[inline]
    pub fn is_transfer_data_bytes_(&self) -> bool {
        *self == DATA_BR::TRANSFER_DATA_BYTES_
    }
}
#[doc = "Possible values of the field `DATA_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_AR {
    #[doc = "Data bytes 0-3 unchanged"]
    UNCHANGED,
    #[doc = "Transfer data bytes 0-3 to IFx message buffer"]
    TRANSFER_DATA_BYTES_,
}
impl DATA_AR {
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
            DATA_AR::UNCHANGED => false,
            DATA_AR::TRANSFER_DATA_BYTES_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_AR {
        match value {
            false => DATA_AR::UNCHANGED,
            true => DATA_AR::TRANSFER_DATA_BYTES_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == DATA_AR::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DATA_BYTES_`"]
    #[inline]
    pub fn is_transfer_data_bytes_(&self) -> bool {
        *self == DATA_AR::TRANSFER_DATA_BYTES_
    }
}
#[doc = "Possible values of the field `NEWDAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEWDATR {
    #[doc = "NEWDAT bit remains unchanged. A read access to a message object can be combined with the reset of the control bits INTPND and NEWDAT in IF1/2_MCTRL. The values of these bits transferred to the IFx Message Control Register always reflect the status before resetting these bits"]
    UNCHANGED,
    #[doc = "Clear NEWDAT bit in the message object"]
    CLEAR_NEWDAT_BIT_IN_,
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
            NEWDATR::UNCHANGED => false,
            NEWDATR::CLEAR_NEWDAT_BIT_IN_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEWDATR {
        match value {
            false => NEWDATR::UNCHANGED,
            true => NEWDATR::CLEAR_NEWDAT_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == NEWDATR::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `CLEAR_NEWDAT_BIT_IN_`"]
    #[inline]
    pub fn is_clear_newdat_bit_in_(&self) -> bool {
        *self == NEWDATR::CLEAR_NEWDAT_BIT_IN_
    }
}
#[doc = "Possible values of the field `CLRINTPND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRINTPNDR {
    #[doc = "INTPND bit remains unchanged"]
    UNCHANGED,
    #[doc = "Clear INTPND bit in the message object"]
    CLEAR_INTPND_BIT_IN_,
}
impl CLRINTPNDR {
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
            CLRINTPNDR::UNCHANGED => false,
            CLRINTPNDR::CLEAR_INTPND_BIT_IN_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLRINTPNDR {
        match value {
            false => CLRINTPNDR::UNCHANGED,
            true => CLRINTPNDR::CLEAR_INTPND_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == CLRINTPNDR::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `CLEAR_INTPND_BIT_IN_`"]
    #[inline]
    pub fn is_clear_intpnd_bit_in_(&self) -> bool {
        *self == CLRINTPNDR::CLEAR_INTPND_BIT_IN_
    }
}
#[doc = "Possible values of the field `CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLR {
    #[doc = "Control bits unchanged"]
    UNCHANGED,
    #[doc = "Transfer control bits to IFx message buffer"]
    TRANSFER_CONTROL_BIT,
}
impl CTRLR {
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
            CTRLR::UNCHANGED => false,
            CTRLR::TRANSFER_CONTROL_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRLR {
        match value {
            false => CTRLR::UNCHANGED,
            true => CTRLR::TRANSFER_CONTROL_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == CTRLR::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_CONTROL_BIT`"]
    #[inline]
    pub fn is_transfer_control_bit(&self) -> bool {
        *self == CTRLR::TRANSFER_CONTROL_BIT
    }
}
#[doc = "Possible values of the field `ARB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBR {
    #[doc = "Arbitration bits unchanged"]
    UNCHANGED,
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to IFx message buffer register"]
    TRANSFER_IDENTIFIER,
}
impl ARBR {
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
            ARBR::UNCHANGED => false,
            ARBR::TRANSFER_IDENTIFIER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBR {
        match value {
            false => ARBR::UNCHANGED,
            true => ARBR::TRANSFER_IDENTIFIER,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == ARBR::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_IDENTIFIER`"]
    #[inline]
    pub fn is_transfer_identifier(&self) -> bool {
        *self == ARBR::TRANSFER_IDENTIFIER
    }
}
#[doc = "Possible values of the field `MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKR {
    #[doc = "Mask bits unchanged"]
    UNCHANGED,
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to IFx message buffer register"]
    TRANSFER_IDENTIFIER_,
}
impl MASKR {
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
            MASKR::UNCHANGED => false,
            MASKR::TRANSFER_IDENTIFIER_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASKR {
        match value {
            false => MASKR::UNCHANGED,
            true => MASKR::TRANSFER_IDENTIFIER_,
        }
    }
    #[doc = "Checks if the value of the field is `UNCHANGED`"]
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        *self == MASKR::UNCHANGED
    }
    #[doc = "Checks if the value of the field is `TRANSFER_IDENTIFIER_`"]
    #[inline]
    pub fn is_transfer_identifier_(&self) -> bool {
        *self == MASKR::TRANSFER_IDENTIFIER_
    }
}
#[doc = r" Value of the field"]
pub struct WR_RDR {
    bits: bool,
}
impl WR_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `DATA_B`"]
pub enum DATA_BW {
    #[doc = "Data bytes 4-7 unchanged"]
    UNCHANGED,
    #[doc = "Transfer data bytes 4-7 to IFx message buffer register"]
    TRANSFER_DATA_BYTES_,
}
impl DATA_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_BW::UNCHANGED => false,
            DATA_BW::TRANSFER_DATA_BYTES_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_BW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data bytes 4-7 unchanged"]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(DATA_BW::UNCHANGED)
    }
    #[doc = "Transfer data bytes 4-7 to IFx message buffer register"]
    #[inline]
    pub fn transfer_data_bytes_(self) -> &'a mut W {
        self.variant(DATA_BW::TRANSFER_DATA_BYTES_)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_A`"]
pub enum DATA_AW {
    #[doc = "Data bytes 0-3 unchanged"]
    UNCHANGED,
    #[doc = "Transfer data bytes 0-3 to IFx message buffer"]
    TRANSFER_DATA_BYTES_,
}
impl DATA_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_AW::UNCHANGED => false,
            DATA_AW::TRANSFER_DATA_BYTES_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_AW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data bytes 0-3 unchanged"]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(DATA_AW::UNCHANGED)
    }
    #[doc = "Transfer data bytes 0-3 to IFx message buffer"]
    #[inline]
    pub fn transfer_data_bytes_(self) -> &'a mut W {
        self.variant(DATA_AW::TRANSFER_DATA_BYTES_)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NEWDAT`"]
pub enum NEWDATW {
    #[doc = "NEWDAT bit remains unchanged. A read access to a message object can be combined with the reset of the control bits INTPND and NEWDAT in IF1/2_MCTRL. The values of these bits transferred to the IFx Message Control Register always reflect the status before resetting these bits"]
    UNCHANGED,
    #[doc = "Clear NEWDAT bit in the message object"]
    CLEAR_NEWDAT_BIT_IN_,
}
impl NEWDATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NEWDATW::UNCHANGED => false,
            NEWDATW::CLEAR_NEWDAT_BIT_IN_ => true,
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
    #[doc = "NEWDAT bit remains unchanged. A read access to a message object can be combined with the reset of the control bits INTPND and NEWDAT in IF1/2_MCTRL. The values of these bits transferred to the IFx Message Control Register always reflect the status before resetting these bits"]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(NEWDATW::UNCHANGED)
    }
    #[doc = "Clear NEWDAT bit in the message object"]
    #[inline]
    pub fn clear_newdat_bit_in_(self) -> &'a mut W {
        self.variant(NEWDATW::CLEAR_NEWDAT_BIT_IN_)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLRINTPND`"]
pub enum CLRINTPNDW {
    #[doc = "INTPND bit remains unchanged"]
    UNCHANGED,
    #[doc = "Clear INTPND bit in the message object"]
    CLEAR_INTPND_BIT_IN_,
}
impl CLRINTPNDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRINTPNDW::UNCHANGED => false,
            CLRINTPNDW::CLEAR_INTPND_BIT_IN_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRINTPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRINTPNDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRINTPNDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INTPND bit remains unchanged"]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(CLRINTPNDW::UNCHANGED)
    }
    #[doc = "Clear INTPND bit in the message object"]
    #[inline]
    pub fn clear_intpnd_bit_in_(self) -> &'a mut W {
        self.variant(CLRINTPNDW::CLEAR_INTPND_BIT_IN_)
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
#[doc = "Values that can be written to the field `CTRL`"]
pub enum CTRLW {
    #[doc = "Control bits unchanged"]
    UNCHANGED,
    #[doc = "Transfer control bits to IFx message buffer"]
    TRANSFER_CONTROL_BIT,
}
impl CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTRLW::UNCHANGED => false,
            CTRLW::TRANSFER_CONTROL_BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Control bits unchanged"]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(CTRLW::UNCHANGED)
    }
    #[doc = "Transfer control bits to IFx message buffer"]
    #[inline]
    pub fn transfer_control_bit(self) -> &'a mut W {
        self.variant(CTRLW::TRANSFER_CONTROL_BIT)
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
#[doc = "Values that can be written to the field `ARB`"]
pub enum ARBW {
    #[doc = "Arbitration bits unchanged"]
    UNCHANGED,
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to IFx message buffer register"]
    TRANSFER_IDENTIFIER,
}
impl ARBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBW::UNCHANGED => false,
            ARBW::TRANSFER_IDENTIFIER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Arbitration bits unchanged"]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(ARBW::UNCHANGED)
    }
    #[doc = "Transfer Identifier, DIR, XTD, and MSGVAL bits to IFx message buffer register"]
    #[inline]
    pub fn transfer_identifier(self) -> &'a mut W {
        self.variant(ARBW::TRANSFER_IDENTIFIER)
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
#[doc = "Values that can be written to the field `MASK`"]
pub enum MASKW {
    #[doc = "Mask bits unchanged"]
    UNCHANGED,
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to IFx message buffer register"]
    TRANSFER_IDENTIFIER_,
}
impl MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASKW::UNCHANGED => false,
            MASKW::TRANSFER_IDENTIFIER_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mask bits unchanged"]
    #[inline]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(MASKW::UNCHANGED)
    }
    #[doc = "Transfer Identifier MASK + MDIR + MXTD to IFx message buffer register"]
    #[inline]
    pub fn transfer_identifier_(self) -> &'a mut W {
        self.variant(MASKW::TRANSFER_IDENTIFIER_)
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
#[doc = r" Proxy"]
pub struct _WR_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _WR_RDW<'a> {
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
    #[doc = "Bit 0 - Access data bytes 4-7"]
    #[inline]
    pub fn data_b(&self) -> DATA_BR {
        DATA_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Access data bytes 0-3"]
    #[inline]
    pub fn data_a(&self) -> DATA_AR {
        DATA_AR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Access new data bit"]
    #[inline]
    pub fn newdat(&self) -> NEWDATR {
        NEWDATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Clear interrupt pending bit"]
    #[inline]
    pub fn clrintpnd(&self) -> CLRINTPNDR {
        CLRINTPNDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Access control bits"]
    #[inline]
    pub fn ctrl(&self) -> CTRLR {
        CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Access arbitration bits"]
    #[inline]
    pub fn arb(&self) -> ARBR {
        ARBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Access mask bits"]
    #[inline]
    pub fn mask(&self) -> MASKR {
        MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Read transfer Transfer data from the message object addressed by the command request register to the selected message buffer registers CANIFn_CMDREQ"]
    #[inline]
    pub fn wr_rd(&self) -> WR_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WR_RDR { bits }
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
    #[doc = "Bit 0 - Access data bytes 4-7"]
    #[inline]
    pub fn data_b(&mut self) -> _DATA_BW {
        _DATA_BW { w: self }
    }
    #[doc = "Bit 1 - Access data bytes 0-3"]
    #[inline]
    pub fn data_a(&mut self) -> _DATA_AW {
        _DATA_AW { w: self }
    }
    #[doc = "Bit 2 - Access new data bit"]
    #[inline]
    pub fn newdat(&mut self) -> _NEWDATW {
        _NEWDATW { w: self }
    }
    #[doc = "Bit 3 - Clear interrupt pending bit"]
    #[inline]
    pub fn clrintpnd(&mut self) -> _CLRINTPNDW {
        _CLRINTPNDW { w: self }
    }
    #[doc = "Bit 4 - Access control bits"]
    #[inline]
    pub fn ctrl(&mut self) -> _CTRLW {
        _CTRLW { w: self }
    }
    #[doc = "Bit 5 - Access arbitration bits"]
    #[inline]
    pub fn arb(&mut self) -> _ARBW {
        _ARBW { w: self }
    }
    #[doc = "Bit 6 - Access mask bits"]
    #[inline]
    pub fn mask(&mut self) -> _MASKW {
        _MASKW { w: self }
    }
    #[doc = "Bit 7 - Read transfer Transfer data from the message object addressed by the command request register to the selected message buffer registers CANIFn_CMDREQ"]
    #[inline]
    pub fn wr_rd(&mut self) -> _WR_RDW {
        _WR_RDW { w: self }
    }
}
