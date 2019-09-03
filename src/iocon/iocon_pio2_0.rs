#[doc = "Reader of register IOCON_PIO2_0"]
pub type R = crate::R<u32, super::IOCON_PIO2_0>;
#[doc = "Writer for register IOCON_PIO2_0"]
pub type W = crate::W<u32, super::IOCON_PIO2_0>;
#[doc = "Register IOCON_PIO2_0 `reset()`'s with value 0xd0"]
impl crate::ResetValue for super::IOCON_PIO2_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xd0
    }
}
#[doc = "Configure pin function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNC_A {
    #[doc = "0: Pin function PIO2_0"]
    PIO2_0,
    #[doc = "1: Pin function DTR"]
    DTR,
    #[doc = "2: Pin function SSEL1"]
    SSEL1,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        match variant {
            FUNC_A::PIO2_0 => 0,
            FUNC_A::DTR => 1,
            FUNC_A::SSEL1 => 2,
        }
    }
}
#[doc = "Reader of field `FUNC`"]
pub type FUNC_R = crate::R<u8, FUNC_A>;
impl FUNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FUNC_A::PIO2_0),
            1 => Val(FUNC_A::DTR),
            2 => Val(FUNC_A::SSEL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO2_0`"]
    #[inline(always)]
    pub fn is_pio2_0(&self) -> bool {
        *self == FUNC_A::PIO2_0
    }
    #[doc = "Checks if the value of the field is `DTR`"]
    #[inline(always)]
    pub fn is_dtr(&self) -> bool {
        *self == FUNC_A::DTR
    }
    #[doc = "Checks if the value of the field is `SSEL1`"]
    #[inline(always)]
    pub fn is_ssel1(&self) -> bool {
        *self == FUNC_A::SSEL1
    }
}
#[doc = "Write proxy for field `FUNC`"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin function PIO2_0"]
    #[inline(always)]
    pub fn pio2_0(self) -> &'a mut W {
        self.variant(FUNC_A::PIO2_0)
    }
    #[doc = "Pin function DTR"]
    #[inline(always)]
    pub fn dtr(self) -> &'a mut W {
        self.variant(FUNC_A::DTR)
    }
    #[doc = "Pin function SSEL1"]
    #[inline(always)]
    pub fn ssel1(self) -> &'a mut W {
        self.variant(FUNC_A::SSEL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Pin function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)"]
    INACTIVE_NO_PULL_DO,
    #[doc = "1: Pull-down resistor enabled"]
    PULL_DOWN_RESISTOR_E,
    #[doc = "2: Pull-up resistor enabled"]
    PULL_UP_RESISTOR_ENA,
    #[doc = "3: Repeater mode"]
    REPEATER_MODE_,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::INACTIVE_NO_PULL_DO => 0,
            MODE_A::PULL_DOWN_RESISTOR_E => 1,
            MODE_A::PULL_UP_RESISTOR_ENA => 2,
            MODE_A::REPEATER_MODE_ => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::INACTIVE_NO_PULL_DO,
            1 => MODE_A::PULL_DOWN_RESISTOR_E,
            2 => MODE_A::PULL_UP_RESISTOR_ENA,
            3 => MODE_A::REPEATER_MODE_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_NO_PULL_DO`"]
    #[inline(always)]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == MODE_A::INACTIVE_NO_PULL_DO
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_RESISTOR_E`"]
    #[inline(always)]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == MODE_A::PULL_DOWN_RESISTOR_E
    }
    #[doc = "Checks if the value of the field is `PULL_UP_RESISTOR_ENA`"]
    #[inline(always)]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == MODE_A::PULL_UP_RESISTOR_ENA
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE_`"]
    #[inline(always)]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == MODE_A::REPEATER_MODE_
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)"]
    #[inline(always)]
    pub fn inactive_no_pull_do(self) -> &'a mut W {
        self.variant(MODE_A::INACTIVE_NO_PULL_DO)
    }
    #[doc = "Pull-down resistor enabled"]
    #[inline(always)]
    pub fn pull_down_resistor_e(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN_RESISTOR_E)
    }
    #[doc = "Pull-up resistor enabled"]
    #[inline(always)]
    pub fn pull_up_resistor_ena(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP_RESISTOR_ENA)
    }
    #[doc = "Repeater mode"]
    #[inline(always)]
    pub fn repeater_mode_(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER_MODE_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTERESIS_A {
    #[doc = "0: Disable"]
    DISABLE,
    #[doc = "1: Enable"]
    ENABLE,
}
impl From<HYSTERESIS_A> for bool {
    #[inline(always)]
    fn from(variant: HYSTERESIS_A) -> Self {
        match variant {
            HYSTERESIS_A::DISABLE => false,
            HYSTERESIS_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `HYSTERESIS`"]
pub type HYSTERESIS_R = crate::R<bool, HYSTERESIS_A>;
impl HYSTERESIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTERESIS_A {
        match self.bits {
            false => HYSTERESIS_A::DISABLE,
            true => HYSTERESIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYSTERESIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYSTERESIS_A::ENABLE
    }
}
#[doc = "Write proxy for field `HYSTERESIS`"]
pub struct HYSTERESIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTERESIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYSTERESIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYSTERESIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYSTERESIS_A::ENABLE)
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
#[doc = "Selects pseudo open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPEN_DRAIN_A {
    #[doc = "0: Standard GPIO output"]
    GPIO_OUTPUT,
    #[doc = "1: Open-drain output"]
    OPEN_DRAIN_OUTPUT,
}
impl From<OPEN_DRAIN_A> for bool {
    #[inline(always)]
    fn from(variant: OPEN_DRAIN_A) -> Self {
        match variant {
            OPEN_DRAIN_A::GPIO_OUTPUT => false,
            OPEN_DRAIN_A::OPEN_DRAIN_OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `OPEN_DRAIN`"]
pub type OPEN_DRAIN_R = crate::R<bool, OPEN_DRAIN_A>;
impl OPEN_DRAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPEN_DRAIN_A {
        match self.bits {
            false => OPEN_DRAIN_A::GPIO_OUTPUT,
            true => OPEN_DRAIN_A::OPEN_DRAIN_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_OUTPUT`"]
    #[inline(always)]
    pub fn is_gpio_output(&self) -> bool {
        *self == OPEN_DRAIN_A::GPIO_OUTPUT
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_OUTPUT`"]
    #[inline(always)]
    pub fn is_open_drain_output(&self) -> bool {
        *self == OPEN_DRAIN_A::OPEN_DRAIN_OUTPUT
    }
}
#[doc = "Write proxy for field `OPEN_DRAIN`"]
pub struct OPEN_DRAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPEN_DRAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPEN_DRAIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard GPIO output"]
    #[inline(always)]
    pub fn gpio_output(self) -> &'a mut W {
        self.variant(OPEN_DRAIN_A::GPIO_OUTPUT)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn open_drain_output(self) -> &'a mut W {
        self.variant(OPEN_DRAIN_A::OPEN_DRAIN_OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Configure pin function."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Pin function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hysteresis(&self) -> HYSTERESIS_R {
        HYSTERESIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode."]
    #[inline(always)]
    pub fn open_drain(&self) -> OPEN_DRAIN_R {
        OPEN_DRAIN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configure pin function."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 3:4 - Pin function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hysteresis(&mut self) -> HYSTERESIS_W {
        HYSTERESIS_W { w: self }
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode."]
    #[inline(always)]
    pub fn open_drain(&mut self) -> OPEN_DRAIN_W {
        OPEN_DRAIN_W { w: self }
    }
}
