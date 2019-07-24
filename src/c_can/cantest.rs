#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CANTEST {
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
#[doc = "Possible values of the field `BASIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASICR {
    #[doc = "Basic mode disabled"]
    BASIC_MODE_DISABLED_,
    #[doc = "IF1 registers used as TX buffer, IF2 registers used as RX buffer"]
    TXRX,
}
impl BASICR {
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
            BASICR::BASIC_MODE_DISABLED_ => false,
            BASICR::TXRX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BASICR {
        match value {
            false => BASICR::BASIC_MODE_DISABLED_,
            true => BASICR::TXRX,
        }
    }
    #[doc = "Checks if the value of the field is `BASIC_MODE_DISABLED_`"]
    #[inline]
    pub fn is_basic_mode_disabled_(&self) -> bool {
        *self == BASICR::BASIC_MODE_DISABLED_
    }
    #[doc = "Checks if the value of the field is `TXRX`"]
    #[inline]
    pub fn is_txrx(&self) -> bool {
        *self == BASICR::TXRX
    }
}
#[doc = "Possible values of the field `SILENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILENTR {
    #[doc = "Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "The module is in silent mode"]
    SILENT,
}
impl SILENTR {
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
            SILENTR::NORMAL_OPERATION_ => false,
            SILENTR::SILENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SILENTR {
        match value {
            false => SILENTR::NORMAL_OPERATION_,
            true => SILENTR::SILENT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline]
    pub fn is_normal_operation_(&self) -> bool {
        *self == SILENTR::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `SILENT`"]
    #[inline]
    pub fn is_silent(&self) -> bool {
        *self == SILENTR::SILENT
    }
}
#[doc = "Possible values of the field `LBACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBACKR {
    #[doc = "Loop back mode is disabled"]
    DISABLED,
    #[doc = "Loop back mode is enabled"]
    ENABLED,
}
impl LBACKR {
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
            LBACKR::DISABLED => false,
            LBACKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBACKR {
        match value {
            false => LBACKR::DISABLED,
            true => LBACKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LBACKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LBACKR::ENABLED
    }
}
#[doc = "Possible values of the field `TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXR {
    #[doc = "Level at the CAN_TXD pin is controlled by the CAN controller. This is the value at reset"]
    LEVEL,
    #[doc = "The sample point can be monitored at the CAN_TXD pin"]
    TXD,
    #[doc = "CAN_TXD pin is driven LOW/dominant"]
    LOW,
    #[doc = "CAN_TXD pin is driven HIGH/recessive"]
    HIGH,
}
impl TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXR::LEVEL => 0,
            TXR::TXD => 1,
            TXR::LOW => 2,
            TXR::HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXR {
        match value {
            0 => TXR::LEVEL,
            1 => TXR::TXD,
            2 => TXR::LOW,
            3 => TXR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == TXR::LEVEL
    }
    #[doc = "Checks if the value of the field is `TXD`"]
    #[inline]
    pub fn is_txd(&self) -> bool {
        *self == TXR::TXD
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == TXR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == TXR::HIGH
    }
}
#[doc = "Possible values of the field `RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXR {
    #[doc = "The CAN bus is recessive (CAN_RXD = 1)"]
    RECESSIVE,
    #[doc = "The CAN bus is dominant (CAN_RXD = 0)"]
    DORMANT,
}
impl RXR {
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
            RXR::RECESSIVE => false,
            RXR::DORMANT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXR {
        match value {
            false => RXR::RECESSIVE,
            true => RXR::DORMANT,
        }
    }
    #[doc = "Checks if the value of the field is `RECESSIVE`"]
    #[inline]
    pub fn is_recessive(&self) -> bool {
        *self == RXR::RECESSIVE
    }
    #[doc = "Checks if the value of the field is `DORMANT`"]
    #[inline]
    pub fn is_dormant(&self) -> bool {
        *self == RXR::DORMANT
    }
}
#[doc = "Values that can be written to the field `BASIC`"]
pub enum BASICW {
    #[doc = "Basic mode disabled"]
    BASIC_MODE_DISABLED_,
    #[doc = "IF1 registers used as TX buffer, IF2 registers used as RX buffer"]
    TXRX,
}
impl BASICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BASICW::BASIC_MODE_DISABLED_ => false,
            BASICW::TXRX => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BASICW<'a> {
    w: &'a mut W,
}
impl<'a> _BASICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BASICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Basic mode disabled"]
    #[inline]
    pub fn basic_mode_disabled_(self) -> &'a mut W {
        self.variant(BASICW::BASIC_MODE_DISABLED_)
    }
    #[doc = "IF1 registers used as TX buffer, IF2 registers used as RX buffer"]
    #[inline]
    pub fn txrx(self) -> &'a mut W {
        self.variant(BASICW::TXRX)
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
#[doc = "Values that can be written to the field `SILENT`"]
pub enum SILENTW {
    #[doc = "Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "The module is in silent mode"]
    SILENT,
}
impl SILENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SILENTW::NORMAL_OPERATION_ => false,
            SILENTW::SILENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SILENTW<'a> {
    w: &'a mut W,
}
impl<'a> _SILENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SILENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(SILENTW::NORMAL_OPERATION_)
    }
    #[doc = "The module is in silent mode"]
    #[inline]
    pub fn silent(self) -> &'a mut W {
        self.variant(SILENTW::SILENT)
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
#[doc = "Values that can be written to the field `LBACK`"]
pub enum LBACKW {
    #[doc = "Loop back mode is disabled"]
    DISABLED,
    #[doc = "Loop back mode is enabled"]
    ENABLED,
}
impl LBACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBACKW::DISABLED => false,
            LBACKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LBACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loop back mode is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBACKW::DISABLED)
    }
    #[doc = "Loop back mode is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBACKW::ENABLED)
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
#[doc = "Values that can be written to the field `TX`"]
pub enum TXW {
    #[doc = "Level at the CAN_TXD pin is controlled by the CAN controller. This is the value at reset"]
    LEVEL,
    #[doc = "The sample point can be monitored at the CAN_TXD pin"]
    TXD,
    #[doc = "CAN_TXD pin is driven LOW/dominant"]
    LOW,
    #[doc = "CAN_TXD pin is driven HIGH/recessive"]
    HIGH,
}
impl TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXW::LEVEL => 0,
            TXW::TXD => 1,
            TXW::LOW => 2,
            TXW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXW<'a> {
    w: &'a mut W,
}
impl<'a> _TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level at the CAN_TXD pin is controlled by the CAN controller. This is the value at reset"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(TXW::LEVEL)
    }
    #[doc = "The sample point can be monitored at the CAN_TXD pin"]
    #[inline]
    pub fn txd(self) -> &'a mut W {
        self.variant(TXW::TXD)
    }
    #[doc = "CAN_TXD pin is driven LOW/dominant"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(TXW::LOW)
    }
    #[doc = "CAN_TXD pin is driven HIGH/recessive"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(TXW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX`"]
pub enum RXW {
    #[doc = "The CAN bus is recessive (CAN_RXD = 1)"]
    RECESSIVE,
    #[doc = "The CAN bus is dominant (CAN_RXD = 0)"]
    DORMANT,
}
impl RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXW::RECESSIVE => false,
            RXW::DORMANT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXW<'a> {
    w: &'a mut W,
}
impl<'a> _RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CAN bus is recessive (CAN_RXD = 1)"]
    #[inline]
    pub fn recessive(self) -> &'a mut W {
        self.variant(RXW::RECESSIVE)
    }
    #[doc = "The CAN bus is dominant (CAN_RXD = 0)"]
    #[inline]
    pub fn dormant(self) -> &'a mut W {
        self.variant(RXW::DORMANT)
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
    #[doc = "Bit 2 - Basic mode"]
    #[inline]
    pub fn basic(&self) -> BASICR {
        BASICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Silent mode"]
    #[inline]
    pub fn silent(&self) -> SILENTR {
        SILENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Loop back mode"]
    #[inline]
    pub fn lback(&self) -> LBACKR {
        LBACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Control of CAN_TXD pins"]
    #[inline]
    pub fn tx(&self) -> TXR {
        TXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Monitors the actual value of the CAN_RXD pin"]
    #[inline]
    pub fn rx(&self) -> RXR {
        RXR::_from({
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
    #[doc = "Bit 2 - Basic mode"]
    #[inline]
    pub fn basic(&mut self) -> _BASICW {
        _BASICW { w: self }
    }
    #[doc = "Bit 3 - Silent mode"]
    #[inline]
    pub fn silent(&mut self) -> _SILENTW {
        _SILENTW { w: self }
    }
    #[doc = "Bit 4 - Loop back mode"]
    #[inline]
    pub fn lback(&mut self) -> _LBACKW {
        _LBACKW { w: self }
    }
    #[doc = "Bits 5:6 - Control of CAN_TXD pins"]
    #[inline]
    pub fn tx(&mut self) -> _TXW {
        _TXW { w: self }
    }
    #[doc = "Bit 7 - Monitors the actual value of the CAN_RXD pin"]
    #[inline]
    pub fn rx(&mut self) -> _RXW {
        _RXW { w: self }
    }
}
