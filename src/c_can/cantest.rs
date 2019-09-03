#[doc = "Reader of register CANTEST"]
pub type R = crate::R<u32, super::CANTEST>;
#[doc = "Writer for register CANTEST"]
pub type W = crate::W<u32, super::CANTEST>;
#[doc = "Register CANTEST `reset()`'s with value 0"]
impl crate::ResetValue for super::CANTEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Basic mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASIC_A {
    #[doc = "0: Basic mode disabled"]
    BASIC_MODE_DISABLED_,
    #[doc = "1: IF1 registers used as TX buffer, IF2 registers used as RX buffer"]
    TXRX,
}
impl From<BASIC_A> for bool {
    #[inline(always)]
    fn from(variant: BASIC_A) -> Self {
        match variant {
            BASIC_A::BASIC_MODE_DISABLED_ => false,
            BASIC_A::TXRX => true,
        }
    }
}
#[doc = "Reader of field `BASIC`"]
pub type BASIC_R = crate::R<bool, BASIC_A>;
impl BASIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BASIC_A {
        match self.bits {
            false => BASIC_A::BASIC_MODE_DISABLED_,
            true => BASIC_A::TXRX,
        }
    }
    #[doc = "Checks if the value of the field is `BASIC_MODE_DISABLED_`"]
    #[inline(always)]
    pub fn is_basic_mode_disabled_(&self) -> bool {
        *self == BASIC_A::BASIC_MODE_DISABLED_
    }
    #[doc = "Checks if the value of the field is `TXRX`"]
    #[inline(always)]
    pub fn is_txrx(&self) -> bool {
        *self == BASIC_A::TXRX
    }
}
#[doc = "Write proxy for field `BASIC`"]
pub struct BASIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BASIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BASIC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Basic mode disabled"]
    #[inline(always)]
    pub fn basic_mode_disabled_(self) -> &'a mut W {
        self.variant(BASIC_A::BASIC_MODE_DISABLED_)
    }
    #[doc = "IF1 registers used as TX buffer, IF2 registers used as RX buffer"]
    #[inline(always)]
    pub fn txrx(self) -> &'a mut W {
        self.variant(BASIC_A::TXRX)
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
#[doc = "Silent mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILENT_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPERATION_,
    #[doc = "1: The module is in silent mode"]
    SILENT,
}
impl From<SILENT_A> for bool {
    #[inline(always)]
    fn from(variant: SILENT_A) -> Self {
        match variant {
            SILENT_A::NORMAL_OPERATION_ => false,
            SILENT_A::SILENT => true,
        }
    }
}
#[doc = "Reader of field `SILENT`"]
pub type SILENT_R = crate::R<bool, SILENT_A>;
impl SILENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SILENT_A {
        match self.bits {
            false => SILENT_A::NORMAL_OPERATION_,
            true => SILENT_A::SILENT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline(always)]
    pub fn is_normal_operation_(&self) -> bool {
        *self == SILENT_A::NORMAL_OPERATION_
    }
    #[doc = "Checks if the value of the field is `SILENT`"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == SILENT_A::SILENT
    }
}
#[doc = "Write proxy for field `SILENT`"]
pub struct SILENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SILENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SILENT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(SILENT_A::NORMAL_OPERATION_)
    }
    #[doc = "The module is in silent mode"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut W {
        self.variant(SILENT_A::SILENT)
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
#[doc = "Loop back mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBACK_A {
    #[doc = "0: Loop back mode is disabled"]
    DISABLED,
    #[doc = "1: Loop back mode is enabled"]
    ENABLED,
}
impl From<LBACK_A> for bool {
    #[inline(always)]
    fn from(variant: LBACK_A) -> Self {
        match variant {
            LBACK_A::DISABLED => false,
            LBACK_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LBACK`"]
pub type LBACK_R = crate::R<bool, LBACK_A>;
impl LBACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBACK_A {
        match self.bits {
            false => LBACK_A::DISABLED,
            true => LBACK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBACK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBACK_A::ENABLED
    }
}
#[doc = "Write proxy for field `LBACK`"]
pub struct LBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loop back mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBACK_A::DISABLED)
    }
    #[doc = "Loop back mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBACK_A::ENABLED)
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
#[doc = "Control of CAN_TXD pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: Level at the CAN_TXD pin is controlled by the CAN controller. This is the value at reset"]
    LEVEL,
    #[doc = "1: The sample point can be monitored at the CAN_TXD pin"]
    TXD,
    #[doc = "2: CAN_TXD pin is driven LOW/dominant"]
    LOW,
    #[doc = "3: CAN_TXD pin is driven HIGH/recessive"]
    HIGH,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        match variant {
            TX_A::LEVEL => 0,
            TX_A::TXD => 1,
            TX_A::LOW => 2,
            TX_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<u8, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::LEVEL,
            1 => TX_A::TXD,
            2 => TX_A::LOW,
            3 => TX_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TX_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `TXD`"]
    #[inline(always)]
    pub fn is_txd(&self) -> bool {
        *self == TX_A::TXD
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TX_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TX_A::HIGH
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level at the CAN_TXD pin is controlled by the CAN controller. This is the value at reset"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(TX_A::LEVEL)
    }
    #[doc = "The sample point can be monitored at the CAN_TXD pin"]
    #[inline(always)]
    pub fn txd(self) -> &'a mut W {
        self.variant(TX_A::TXD)
    }
    #[doc = "CAN_TXD pin is driven LOW/dominant"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TX_A::LOW)
    }
    #[doc = "CAN_TXD pin is driven HIGH/recessive"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TX_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Monitors the actual value of the CAN_RXD pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: The CAN bus is recessive (CAN_RXD = 1)"]
    RECESSIVE,
    #[doc = "1: The CAN bus is dominant (CAN_RXD = 0)"]
    DORMANT,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        match variant {
            RX_A::RECESSIVE => false,
            RX_A::DORMANT => true,
        }
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, RX_A>;
impl RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::RECESSIVE,
            true => RX_A::DORMANT,
        }
    }
    #[doc = "Checks if the value of the field is `RECESSIVE`"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == RX_A::RECESSIVE
    }
    #[doc = "Checks if the value of the field is `DORMANT`"]
    #[inline(always)]
    pub fn is_dormant(&self) -> bool {
        *self == RX_A::DORMANT
    }
}
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CAN bus is recessive (CAN_RXD = 1)"]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut W {
        self.variant(RX_A::RECESSIVE)
    }
    #[doc = "The CAN bus is dominant (CAN_RXD = 0)"]
    #[inline(always)]
    pub fn dormant(self) -> &'a mut W {
        self.variant(RX_A::DORMANT)
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
    #[doc = "Bit 2 - Basic mode."]
    #[inline(always)]
    pub fn basic(&self) -> BASIC_R {
        BASIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Silent mode."]
    #[inline(always)]
    pub fn silent(&self) -> SILENT_R {
        SILENT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loop back mode."]
    #[inline(always)]
    pub fn lback(&self) -> LBACK_R {
        LBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Control of CAN_TXD pins."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Monitors the actual value of the CAN_RXD pin."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic mode."]
    #[inline(always)]
    pub fn basic(&mut self) -> BASIC_W {
        BASIC_W { w: self }
    }
    #[doc = "Bit 3 - Silent mode."]
    #[inline(always)]
    pub fn silent(&mut self) -> SILENT_W {
        SILENT_W { w: self }
    }
    #[doc = "Bit 4 - Loop back mode."]
    #[inline(always)]
    pub fn lback(&mut self) -> LBACK_W {
        LBACK_W { w: self }
    }
    #[doc = "Bits 5:6 - Control of CAN_TXD pins."]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 7 - Monitors the actual value of the CAN_RXD pin."]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
