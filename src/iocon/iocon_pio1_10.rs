#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_PIO1_10 {
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
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "Pin function PIO1_10"]
    PIO1_10,
    #[doc = "Pin function AD6"]
    AD6,
    #[doc = "Pin function CT16B1_MAT1"]
    CT16B1_MAT1,
    #[doc = "Pin function MISO1"]
    MISO1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::PIO1_10 => 0,
            FUNCR::AD6 => 1,
            FUNCR::CT16B1_MAT1 => 2,
            FUNCR::MISO1 => 3,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::PIO1_10,
            1 => FUNCR::AD6,
            2 => FUNCR::CT16B1_MAT1,
            3 => FUNCR::MISO1,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO1_10`"]
    #[inline]
    pub fn is_pio1_10(&self) -> bool {
        *self == FUNCR::PIO1_10
    }
    #[doc = "Checks if the value of the field is `AD6`"]
    #[inline]
    pub fn is_ad6(&self) -> bool {
        *self == FUNCR::AD6
    }
    #[doc = "Checks if the value of the field is `CT16B1_MAT1`"]
    #[inline]
    pub fn is_ct16b1_mat1(&self) -> bool {
        *self == FUNCR::CT16B1_MAT1
    }
    #[doc = "Checks if the value of the field is `MISO1`"]
    #[inline]
    pub fn is_miso1(&self) -> bool {
        *self == FUNCR::MISO1
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)"]
    INACTIVE_NO_PULL_DO,
    #[doc = "Pull-down resistor enabled"]
    PULL_DOWN_RESISTOR_E,
    #[doc = "Pull-up resistor enabled"]
    PULL_UP_RESISTOR_ENA,
    #[doc = "Repeater mode"]
    REPEATER_MODE_,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE_NO_PULL_DO => 0,
            MODER::PULL_DOWN_RESISTOR_E => 1,
            MODER::PULL_UP_RESISTOR_ENA => 2,
            MODER::REPEATER_MODE_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::INACTIVE_NO_PULL_DO,
            1 => MODER::PULL_DOWN_RESISTOR_E,
            2 => MODER::PULL_UP_RESISTOR_ENA,
            3 => MODER::REPEATER_MODE_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_NO_PULL_DO`"]
    #[inline]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == MODER::INACTIVE_NO_PULL_DO
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_RESISTOR_E`"]
    #[inline]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == MODER::PULL_DOWN_RESISTOR_E
    }
    #[doc = "Checks if the value of the field is `PULL_UP_RESISTOR_ENA`"]
    #[inline]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == MODER::PULL_UP_RESISTOR_ENA
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE_`"]
    #[inline]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == MODER::REPEATER_MODE_
    }
}
#[doc = "Possible values of the field `HYSTERESIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTERESISR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl HYSTERESISR {
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
            HYSTERESISR::DISABLE => false,
            HYSTERESISR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSTERESISR {
        match value {
            false => HYSTERESISR::DISABLE,
            true => HYSTERESISR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HYSTERESISR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HYSTERESISR::ENABLE
    }
}
#[doc = "Possible values of the field `ADMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMODER {
    #[doc = "Analog input mode"]
    ANALOG_INPUT_MODE,
    #[doc = "Digital functional mode"]
    DIGITAL_FUNCTIONAL_M,
}
impl ADMODER {
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
            ADMODER::ANALOG_INPUT_MODE => false,
            ADMODER::DIGITAL_FUNCTIONAL_M => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMODER {
        match value {
            false => ADMODER::ANALOG_INPUT_MODE,
            true => ADMODER::DIGITAL_FUNCTIONAL_M,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT_MODE`"]
    #[inline]
    pub fn is_analog_input_mode(&self) -> bool {
        *self == ADMODER::ANALOG_INPUT_MODE
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTIONAL_M`"]
    #[inline]
    pub fn is_digital_functional_m(&self) -> bool {
        *self == ADMODER::DIGITAL_FUNCTIONAL_M
    }
}
#[doc = "Possible values of the field `OPEN_DRAIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPEN_DRAINR {
    #[doc = "Standard GPIO output"]
    GPIO_OUTPUT,
    #[doc = "Open-drain output"]
    OPEN_DRAIN_OUTPUT,
}
impl OPEN_DRAINR {
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
            OPEN_DRAINR::GPIO_OUTPUT => false,
            OPEN_DRAINR::OPEN_DRAIN_OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OPEN_DRAINR {
        match value {
            false => OPEN_DRAINR::GPIO_OUTPUT,
            true => OPEN_DRAINR::OPEN_DRAIN_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_OUTPUT`"]
    #[inline]
    pub fn is_gpio_output(&self) -> bool {
        *self == OPEN_DRAINR::GPIO_OUTPUT
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_OUTPUT`"]
    #[inline]
    pub fn is_open_drain_output(&self) -> bool {
        *self == OPEN_DRAINR::OPEN_DRAIN_OUTPUT
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "Pin function PIO1_10"]
    PIO1_10,
    #[doc = "Pin function AD6"]
    AD6,
    #[doc = "Pin function CT16B1_MAT1"]
    CT16B1_MAT1,
    #[doc = "Pin function MISO1"]
    MISO1,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::PIO1_10 => 0,
            FUNCW::AD6 => 1,
            FUNCW::CT16B1_MAT1 => 2,
            FUNCW::MISO1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Pin function PIO1_10"]
    #[inline]
    pub fn pio1_10(self) -> &'a mut W {
        self.variant(FUNCW::PIO1_10)
    }
    #[doc = "Pin function AD6"]
    #[inline]
    pub fn ad6(self) -> &'a mut W {
        self.variant(FUNCW::AD6)
    }
    #[doc = "Pin function CT16B1_MAT1"]
    #[inline]
    pub fn ct16b1_mat1(self) -> &'a mut W {
        self.variant(FUNCW::CT16B1_MAT1)
    }
    #[doc = "Pin function MISO1"]
    #[inline]
    pub fn miso1(self) -> &'a mut W {
        self.variant(FUNCW::MISO1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)"]
    INACTIVE_NO_PULL_DO,
    #[doc = "Pull-down resistor enabled"]
    PULL_DOWN_RESISTOR_E,
    #[doc = "Pull-up resistor enabled"]
    PULL_UP_RESISTOR_ENA,
    #[doc = "Repeater mode"]
    REPEATER_MODE_,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE_NO_PULL_DO => 0,
            MODEW::PULL_DOWN_RESISTOR_E => 1,
            MODEW::PULL_UP_RESISTOR_ENA => 2,
            MODEW::REPEATER_MODE_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)"]
    #[inline]
    pub fn inactive_no_pull_do(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE_NO_PULL_DO)
    }
    #[doc = "Pull-down resistor enabled"]
    #[inline]
    pub fn pull_down_resistor_e(self) -> &'a mut W {
        self.variant(MODEW::PULL_DOWN_RESISTOR_E)
    }
    #[doc = "Pull-up resistor enabled"]
    #[inline]
    pub fn pull_up_resistor_ena(self) -> &'a mut W {
        self.variant(MODEW::PULL_UP_RESISTOR_ENA)
    }
    #[doc = "Repeater mode"]
    #[inline]
    pub fn repeater_mode_(self) -> &'a mut W {
        self.variant(MODEW::REPEATER_MODE_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HYSTERESIS`"]
pub enum HYSTERESISW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl HYSTERESISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSTERESISW::DISABLE => false,
            HYSTERESISW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTERESISW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTERESISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTERESISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYSTERESISW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYSTERESISW::ENABLE)
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
#[doc = "Values that can be written to the field `ADMODE`"]
pub enum ADMODEW {
    #[doc = "Analog input mode"]
    ANALOG_INPUT_MODE,
    #[doc = "Digital functional mode"]
    DIGITAL_FUNCTIONAL_M,
}
impl ADMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADMODEW::ANALOG_INPUT_MODE => false,
            ADMODEW::DIGITAL_FUNCTIONAL_M => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog input mode"]
    #[inline]
    pub fn analog_input_mode(self) -> &'a mut W {
        self.variant(ADMODEW::ANALOG_INPUT_MODE)
    }
    #[doc = "Digital functional mode"]
    #[inline]
    pub fn digital_functional_m(self) -> &'a mut W {
        self.variant(ADMODEW::DIGITAL_FUNCTIONAL_M)
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
#[doc = "Values that can be written to the field `OPEN_DRAIN`"]
pub enum OPEN_DRAINW {
    #[doc = "Standard GPIO output"]
    GPIO_OUTPUT,
    #[doc = "Open-drain output"]
    OPEN_DRAIN_OUTPUT,
}
impl OPEN_DRAINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPEN_DRAINW::GPIO_OUTPUT => false,
            OPEN_DRAINW::OPEN_DRAIN_OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPEN_DRAINW<'a> {
    w: &'a mut W,
}
impl<'a> _OPEN_DRAINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPEN_DRAINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard GPIO output"]
    #[inline]
    pub fn gpio_output(self) -> &'a mut W {
        self.variant(OPEN_DRAINW::GPIO_OUTPUT)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn open_drain_output(self) -> &'a mut W {
        self.variant(OPEN_DRAINW::OPEN_DRAIN_OUTPUT)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:2 - Configure pin function"]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Pin function mode (on-chip pull-up/pull-down resistor control)"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Hysteresis"]
    #[inline]
    pub fn hysteresis(&self) -> HYSTERESISR {
        HYSTERESISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode"]
    #[inline]
    pub fn admode(&self) -> ADMODER {
        ADMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode"]
    #[inline]
    pub fn open_drain(&self) -> OPEN_DRAINR {
        OPEN_DRAINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 208 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Configure pin function"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 3:4 - Pin function mode (on-chip pull-up/pull-down resistor control)"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 5 - Hysteresis"]
    #[inline]
    pub fn hysteresis(&mut self) -> _HYSTERESISW {
        _HYSTERESISW { w: self }
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode"]
    #[inline]
    pub fn admode(&mut self) -> _ADMODEW {
        _ADMODEW { w: self }
    }
    #[doc = "Bit 10 - Selects pseudo open-drain mode"]
    #[inline]
    pub fn open_drain(&mut self) -> _OPEN_DRAINW {
        _OPEN_DRAINW { w: self }
    }
}
