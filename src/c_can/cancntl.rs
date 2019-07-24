#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANCNTL {
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
#[doc = "Possible values of the field `INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITR {
    #[doc = "Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "Initialization is started. On reset, software needs to initialize the CAN controller"]
    INITIALIZATION,
}
impl INITR {
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
            INITR::NORMAL_OPERATION_ => false,
            INITR::INITIALIZATION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INITR {
        match value {
            false => INITR::NORMAL_OPERATION_,
            true => INITR::INITIALIZATION,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline]
    pub fn is_normal_operation_(&self) -> bool {
        *self == INITR::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `INITIALIZATION`"]
    #[inline]
    pub fn is_initialization(&self) -> bool {
        *self == INITR::INITIALIZATION
    }
}
#[doc = "Possible values of the field `IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IER {
    #[doc = "Disable CAN interrupts. The interrupt line is always HIGH"]
    DISABLE_CAN_INTERRUP,
    #[doc = "Enable CAN interrupts. The interrupt line is set to LOW and remains LOW until all pending interrupts are cleared"]
    ENABLE_CAN_INTERRUPT,
}
impl IER {
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
            IER::DISABLE_CAN_INTERRUP => false,
            IER::ENABLE_CAN_INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IER {
        match value {
            false => IER::DISABLE_CAN_INTERRUP,
            true => IER::ENABLE_CAN_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_CAN_INTERRUP`"]
    #[inline]
    pub fn is_disable_can_interrup(&self) -> bool {
        *self == IER::DISABLE_CAN_INTERRUP
    }
    #[doc = "Checks if the value of the field is `ENABLE_CAN_INTERRUPT`"]
    #[inline]
    pub fn is_enable_can_interrupt(&self) -> bool {
        *self == IER::ENABLE_CAN_INTERRUPT
    }
}
#[doc = "Possible values of the field `SIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIER {
    #[doc = "Disable status change interrupts. No status change interrupt will be generated"]
    DISABLE_STATUS_CHANG,
    #[doc = "Enable status change interrupts. A status change interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected"]
    ENABLE_STATUS_CHANGE,
}
impl SIER {
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
            SIER::DISABLE_STATUS_CHANG => false,
            SIER::ENABLE_STATUS_CHANGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIER {
        match value {
            false => SIER::DISABLE_STATUS_CHANG,
            true => SIER::ENABLE_STATUS_CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_STATUS_CHANG`"]
    #[inline]
    pub fn is_disable_status_chang(&self) -> bool {
        *self == SIER::DISABLE_STATUS_CHANG
    }
    #[doc = "Checks if the value of the field is `ENABLE_STATUS_CHANGE`"]
    #[inline]
    pub fn is_enable_status_change(&self) -> bool {
        *self == SIER::ENABLE_STATUS_CHANGE
    }
}
#[doc = "Possible values of the field `EIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIER {
    #[doc = "Disable error interrupt. No error status interrupt will be generated"]
    DISABLE_ERROR_INTERR,
    #[doc = "Enable error interrupt. A change in the bits BOFF or EWARN in the CANSTAT registers will generate an interrupt"]
    ENABLE_ERROR_INTERRU,
}
impl EIER {
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
            EIER::DISABLE_ERROR_INTERR => false,
            EIER::ENABLE_ERROR_INTERRU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EIER {
        match value {
            false => EIER::DISABLE_ERROR_INTERR,
            true => EIER::ENABLE_ERROR_INTERRU,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_ERROR_INTERR`"]
    #[inline]
    pub fn is_disable_error_interr(&self) -> bool {
        *self == EIER::DISABLE_ERROR_INTERR
    }
    #[doc = "Checks if the value of the field is `ENABLE_ERROR_INTERRU`"]
    #[inline]
    pub fn is_enable_error_interru(&self) -> bool {
        *self == EIER::ENABLE_ERROR_INTERRU
    }
}
#[doc = "Possible values of the field `DAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DARR {
    #[doc = "Automatic retransmission of disturbed messages enabled"]
    ENABLED,
    #[doc = "Automatic retransmission disabled"]
    DISABLED,
}
impl DARR {
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
            DARR::ENABLED => false,
            DARR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DARR {
        match value {
            false => DARR::ENABLED,
            true => DARR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DARR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DARR::DISABLED
    }
}
#[doc = "Possible values of the field `CCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCER {
    #[doc = "The CPU has no write access to the bit timing register"]
    NOACCESS,
    #[doc = "The CPU has write access to the CANBT register while the INIT bit is one"]
    ACCESS,
}
impl CCER {
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
            CCER::NOACCESS => false,
            CCER::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCER {
        match value {
            false => CCER::NOACCESS,
            true => CCER::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_noaccess(&self) -> bool {
        *self == CCER::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == CCER::ACCESS
    }
}
#[doc = "Possible values of the field `TEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TESTR {
    #[doc = "Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "Test mode"]
    TEST_MODE_,
}
impl TESTR {
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
            TESTR::NORMAL_OPERATION_ => false,
            TESTR::TEST_MODE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TESTR {
        match value {
            false => TESTR::NORMAL_OPERATION_,
            true => TESTR::TEST_MODE_,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline]
    pub fn is_normal_operation_(&self) -> bool {
        *self == TESTR::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `TEST_MODE_`"]
    #[inline]
    pub fn is_test_mode_(&self) -> bool {
        *self == TESTR::TEST_MODE_
    }
}
#[doc = "Values that can be written to the field `INIT`"]
pub enum INITW {
    #[doc = "Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "Initialization is started. On reset, software needs to initialize the CAN controller"]
    INITIALIZATION,
}
impl INITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INITW::NORMAL_OPERATION_ => false,
            INITW::INITIALIZATION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITW<'a> {
    w: &'a mut W,
}
impl<'a> _INITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(INITW::NORMAL_OPERATION_)
    }
    #[doc = "Initialization is started. On reset, software needs to initialize the CAN controller"]
    #[inline]
    pub fn initialization(self) -> &'a mut W {
        self.variant(INITW::INITIALIZATION)
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
#[doc = "Values that can be written to the field `IE`"]
pub enum IEW {
    #[doc = "Disable CAN interrupts. The interrupt line is always HIGH"]
    DISABLE_CAN_INTERRUP,
    #[doc = "Enable CAN interrupts. The interrupt line is set to LOW and remains LOW until all pending interrupts are cleared"]
    ENABLE_CAN_INTERRUPT,
}
impl IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEW::DISABLE_CAN_INTERRUP => false,
            IEW::ENABLE_CAN_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEW<'a> {
    w: &'a mut W,
}
impl<'a> _IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CAN interrupts. The interrupt line is always HIGH"]
    #[inline]
    pub fn disable_can_interrup(self) -> &'a mut W {
        self.variant(IEW::DISABLE_CAN_INTERRUP)
    }
    #[doc = "Enable CAN interrupts. The interrupt line is set to LOW and remains LOW until all pending interrupts are cleared"]
    #[inline]
    pub fn enable_can_interrupt(self) -> &'a mut W {
        self.variant(IEW::ENABLE_CAN_INTERRUPT)
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
#[doc = "Values that can be written to the field `SIE`"]
pub enum SIEW {
    #[doc = "Disable status change interrupts. No status change interrupt will be generated"]
    DISABLE_STATUS_CHANG,
    #[doc = "Enable status change interrupts. A status change interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected"]
    ENABLE_STATUS_CHANGE,
}
impl SIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIEW::DISABLE_STATUS_CHANG => false,
            SIEW::ENABLE_STATUS_CHANGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable status change interrupts. No status change interrupt will be generated"]
    #[inline]
    pub fn disable_status_chang(self) -> &'a mut W {
        self.variant(SIEW::DISABLE_STATUS_CHANG)
    }
    #[doc = "Enable status change interrupts. A status change interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected"]
    #[inline]
    pub fn enable_status_change(self) -> &'a mut W {
        self.variant(SIEW::ENABLE_STATUS_CHANGE)
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
#[doc = "Values that can be written to the field `EIE`"]
pub enum EIEW {
    #[doc = "Disable error interrupt. No error status interrupt will be generated"]
    DISABLE_ERROR_INTERR,
    #[doc = "Enable error interrupt. A change in the bits BOFF or EWARN in the CANSTAT registers will generate an interrupt"]
    ENABLE_ERROR_INTERRU,
}
impl EIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EIEW::DISABLE_ERROR_INTERR => false,
            EIEW::ENABLE_ERROR_INTERRU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable error interrupt. No error status interrupt will be generated"]
    #[inline]
    pub fn disable_error_interr(self) -> &'a mut W {
        self.variant(EIEW::DISABLE_ERROR_INTERR)
    }
    #[doc = "Enable error interrupt. A change in the bits BOFF or EWARN in the CANSTAT registers will generate an interrupt"]
    #[inline]
    pub fn enable_error_interru(self) -> &'a mut W {
        self.variant(EIEW::ENABLE_ERROR_INTERRU)
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
#[doc = "Values that can be written to the field `DAR`"]
pub enum DARW {
    #[doc = "Automatic retransmission of disturbed messages enabled"]
    ENABLED,
    #[doc = "Automatic retransmission disabled"]
    DISABLED,
}
impl DARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DARW::ENABLED => false,
            DARW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DARW<'a> {
    w: &'a mut W,
}
impl<'a> _DARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic retransmission of disturbed messages enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DARW::ENABLED)
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DARW::DISABLED)
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
#[doc = "Values that can be written to the field `CCE`"]
pub enum CCEW {
    #[doc = "The CPU has no write access to the bit timing register"]
    NOACCESS,
    #[doc = "The CPU has write access to the CANBT register while the INIT bit is one"]
    ACCESS,
}
impl CCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCEW::NOACCESS => false,
            CCEW::ACCESS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CPU has no write access to the bit timing register"]
    #[inline]
    pub fn noaccess(self) -> &'a mut W {
        self.variant(CCEW::NOACCESS)
    }
    #[doc = "The CPU has write access to the CANBT register while the INIT bit is one"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(CCEW::ACCESS)
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
#[doc = "Values that can be written to the field `TEST`"]
pub enum TESTW {
    #[doc = "Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "Test mode"]
    TEST_MODE_,
}
impl TESTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TESTW::NORMAL_OPERATION_ => false,
            TESTW::TEST_MODE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _TESTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TESTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(TESTW::NORMAL_OPERATION_)
    }
    #[doc = "Test mode"]
    #[inline]
    pub fn test_mode_(self) -> &'a mut W {
        self.variant(TESTW::TEST_MODE_)
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
    #[doc = "Bit 0 - Initialization"]
    #[inline]
    pub fn init(&self) -> INITR {
        INITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Module interrupt enable"]
    #[inline]
    pub fn ie(&self) -> IER {
        IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Status change interrupt enable"]
    #[inline]
    pub fn sie(&self) -> SIER {
        SIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline]
    pub fn eie(&self) -> EIER {
        EIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Disable automatic retransmission"]
    #[inline]
    pub fn dar(&self) -> DARR {
        DARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Configuration change enable"]
    #[inline]
    pub fn cce(&self) -> CCER {
        CCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline]
    pub fn test(&self) -> TESTR {
        TESTR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Initialization"]
    #[inline]
    pub fn init(&mut self) -> _INITW {
        _INITW { w: self }
    }
    #[doc = "Bit 1 - Module interrupt enable"]
    #[inline]
    pub fn ie(&mut self) -> _IEW {
        _IEW { w: self }
    }
    #[doc = "Bit 2 - Status change interrupt enable"]
    #[inline]
    pub fn sie(&mut self) -> _SIEW {
        _SIEW { w: self }
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline]
    pub fn eie(&mut self) -> _EIEW {
        _EIEW { w: self }
    }
    #[doc = "Bit 5 - Disable automatic retransmission"]
    #[inline]
    pub fn dar(&mut self) -> _DARW {
        _DARW { w: self }
    }
    #[doc = "Bit 6 - Configuration change enable"]
    #[inline]
    pub fn cce(&mut self) -> _CCEW {
        _CCEW { w: self }
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline]
    pub fn test(&mut self) -> _TESTW {
        _TESTW { w: self }
    }
}
