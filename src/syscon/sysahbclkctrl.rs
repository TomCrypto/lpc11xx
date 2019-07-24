#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSAHBCLKCTRL {
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
#[doc = "Possible values of the field `SYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSR {
    #[doc = "Enable"]
    ENABLE,
}
impl SYSR {
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
            SYSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSR {
        match value {
            true => SYSR::ENABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SYSR::ENABLE
    }
}
#[doc = "Possible values of the field `ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl ROMR {
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
            ROMR::DISABLE => false,
            ROMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROMR {
        match value {
            false => ROMR::DISABLE,
            true => ROMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ROMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ROMR::ENABLE
    }
}
#[doc = "Possible values of the field `RAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl RAMR {
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
            RAMR::DISABLE => false,
            RAMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMR {
        match value {
            false => RAMR::DISABLE,
            true => RAMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAMR::ENABLE
    }
}
#[doc = "Possible values of the field `FLASHREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREGR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl FLASHREGR {
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
            FLASHREGR::DISABLED => false,
            FLASHREGR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHREGR {
        match value {
            false => FLASHREGR::DISABLED,
            true => FLASHREGR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHREGR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHREGR::ENABLED
    }
}
#[doc = "Possible values of the field `FLASHARRAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHARRAYR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl FLASHARRAYR {
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
            FLASHARRAYR::DISABLED => false,
            FLASHARRAYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHARRAYR {
        match value {
            false => FLASHARRAYR::DISABLED,
            true => FLASHARRAYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHARRAYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHARRAYR::ENABLED
    }
}
#[doc = "Possible values of the field `I2C`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl I2CR {
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
            I2CR::DISABLE => false,
            I2CR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2CR {
        match value {
            false => I2CR::DISABLE,
            true => I2CR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == I2CR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == I2CR::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl GPIOR {
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
            GPIOR::DISABLE => false,
            GPIOR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIOR {
        match value {
            false => GPIOR::DISABLE,
            true => GPIOR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIOR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIOR::ENABLE
    }
}
#[doc = "Possible values of the field `CT16B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT16B0R {
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
            CT16B0R::DISABLE => false,
            CT16B0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT16B0R {
        match value {
            false => CT16B0R::DISABLE,
            true => CT16B0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CT16B0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CT16B0R::ENABLE
    }
}
#[doc = "Possible values of the field `CT16B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT16B1R {
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
            CT16B1R::DISABLE => false,
            CT16B1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT16B1R {
        match value {
            false => CT16B1R::DISABLE,
            true => CT16B1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CT16B1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CT16B1R::ENABLE
    }
}
#[doc = "Possible values of the field `CT32B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT32B0R {
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
            CT32B0R::DISABLE => false,
            CT32B0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT32B0R {
        match value {
            false => CT32B0R::DISABLE,
            true => CT32B0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CT32B0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CT32B0R::ENABLE
    }
}
#[doc = "Possible values of the field `CT32B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT32B1R {
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
            CT32B1R::DISABLE => false,
            CT32B1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT32B1R {
        match value {
            false => CT32B1R::DISABLE,
            true => CT32B1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CT32B1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CT32B1R::ENABLE
    }
}
#[doc = "Possible values of the field `SSP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SSP0R {
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
            SSP0R::DISABLE => false,
            SSP0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP0R {
        match value {
            false => SSP0R::DISABLE,
            true => SSP0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SSP0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SSP0R::ENABLE
    }
}
#[doc = "Possible values of the field `UART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl UARTR {
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
            UARTR::DISABLE => false,
            UARTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UARTR {
        match value {
            false => UARTR::DISABLE,
            true => UARTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UARTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UARTR::ENABLE
    }
}
#[doc = "Possible values of the field `ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl ADCR {
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
            ADCR::DISABLE => false,
            ADCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCR {
        match value {
            false => ADCR::DISABLE,
            true => ADCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADCR::ENABLE
    }
}
#[doc = "Possible values of the field `WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WDTR {
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
            WDTR::DISABLE => false,
            WDTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTR {
        match value {
            false => WDTR::DISABLE,
            true => WDTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WDTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WDTR::ENABLE
    }
}
#[doc = "Possible values of the field `IOCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCONR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl IOCONR {
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
            IOCONR::DISABLE => false,
            IOCONR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOCONR {
        match value {
            false => IOCONR::DISABLE,
            true => IOCONR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IOCONR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IOCONR::ENABLE
    }
}
#[doc = "Possible values of the field `CAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CANR {
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
            CANR::DISABLE => false,
            CANR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CANR {
        match value {
            false => CANR::DISABLE,
            true => CANR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CANR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CANR::ENABLE
    }
}
#[doc = "Possible values of the field `SSP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SSP1R {
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
            SSP1R::DISABLE => false,
            SSP1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP1R {
        match value {
            false => SSP1R::DISABLE,
            true => SSP1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SSP1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SSP1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `SYS`"]
pub enum SYSW {
    #[doc = "Enable"]
    ENABLE,
}
impl SYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSW::ENABLE)
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
#[doc = "Values that can be written to the field `ROM`"]
pub enum ROMW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl ROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROMW::DISABLE => false,
            ROMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROMW<'a> {
    w: &'a mut W,
}
impl<'a> _ROMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROMW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROMW::ENABLE)
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
#[doc = "Values that can be written to the field `RAM`"]
pub enum RAMW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl RAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAMW::DISABLE => false,
            RAMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAMW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAMW::ENABLE)
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
#[doc = "Values that can be written to the field `FLASHREG`"]
pub enum FLASHREGW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl FLASHREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHREGW::DISABLED => false,
            FLASHREGW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHREGW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHREGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHREGW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHREGW::ENABLED)
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
#[doc = "Values that can be written to the field `FLASHARRAY`"]
pub enum FLASHARRAYW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl FLASHARRAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHARRAYW::DISABLED => false,
            FLASHARRAYW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHARRAYW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHARRAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHARRAYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHARRAYW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHARRAYW::ENABLED)
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
#[doc = "Values that can be written to the field `I2C`"]
pub enum I2CW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl I2CW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2CW::DISABLE => false,
            I2CW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2CW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2CW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2CW::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO`"]
pub enum GPIOW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl GPIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOW::DISABLE => false,
            GPIOW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIOW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIOW::ENABLE)
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
#[doc = "Values that can be written to the field `CT16B0`"]
pub enum CT16B0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT16B0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT16B0W::DISABLE => false,
            CT16B0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT16B0W<'a> {
    w: &'a mut W,
}
impl<'a> _CT16B0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT16B0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B0W::ENABLE)
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
#[doc = "Values that can be written to the field `CT16B1`"]
pub enum CT16B1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT16B1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT16B1W::DISABLE => false,
            CT16B1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT16B1W<'a> {
    w: &'a mut W,
}
impl<'a> _CT16B1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT16B1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B1W::ENABLE)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CT32B0`"]
pub enum CT32B0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT32B0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT32B0W::DISABLE => false,
            CT32B0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT32B0W<'a> {
    w: &'a mut W,
}
impl<'a> _CT32B0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT32B0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32B0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32B0W::ENABLE)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CT32B1`"]
pub enum CT32B1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT32B1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT32B1W::DISABLE => false,
            CT32B1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT32B1W<'a> {
    w: &'a mut W,
}
impl<'a> _CT32B1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT32B1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32B1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32B1W::ENABLE)
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
#[doc = "Values that can be written to the field `SSP0`"]
pub enum SSP0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SSP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP0W::DISABLE => false,
            SSP0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP0W<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSP0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSP0W::ENABLE)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART`"]
pub enum UARTW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl UARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UARTW::DISABLE => false,
            UARTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UARTW<'a> {
    w: &'a mut W,
}
impl<'a> _UARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UARTW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UARTW::ENABLE)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC`"]
pub enum ADCW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl ADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCW::DISABLE => false,
            ADCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADCW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADCW::ENABLE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDT`"]
pub enum WDTW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTW::DISABLE => false,
            WDTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDTW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDTW::ENABLE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOCON`"]
pub enum IOCONW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl IOCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCONW::DISABLE => false,
            IOCONW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCONW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCONW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCONW::ENABLE)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAN`"]
pub enum CANW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CANW::DISABLE => false,
            CANW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CANW<'a> {
    w: &'a mut W,
}
impl<'a> _CANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CANW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CANW::ENABLE)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSP1`"]
pub enum SSP1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SSP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP1W::DISABLE => false,
            SSP1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSP1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSP1W::ENABLE)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Clock control for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU (this bit is read-only)"]
    #[inline]
    pub fn sys(&self) -> SYSR {
        SYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock control for ROM"]
    #[inline]
    pub fn rom(&self) -> ROMR {
        ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Clock control for RAM"]
    #[inline]
    pub fn ram(&self) -> RAMR {
        RAMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Clock control for the flash register interface"]
    #[inline]
    pub fn flashreg(&self) -> FLASHREGR {
        FLASHREGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Clock control for flash array access"]
    #[inline]
    pub fn flasharray(&self) -> FLASHARRAYR {
        FLASHARRAYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Clock control for I2C"]
    #[inline]
    pub fn i2c(&self) -> I2CR {
        I2CR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Clock control for GPIO"]
    #[inline]
    pub fn gpio(&self) -> GPIOR {
        GPIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Clock control for CT16B0"]
    #[inline]
    pub fn ct16b0(&self) -> CT16B0R {
        CT16B0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Clock control for CT16B1"]
    #[inline]
    pub fn ct16b1(&self) -> CT16B1R {
        CT16B1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Clock control for CT32B0"]
    #[inline]
    pub fn ct32b0(&self) -> CT32B0R {
        CT32B0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Clock control for CT32B1"]
    #[inline]
    pub fn ct32b1(&self) -> CT32B1R {
        CT32B1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Clock control for SPI0"]
    #[inline]
    pub fn ssp0(&self) -> SSP0R {
        SSP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Clock control for UART"]
    #[inline]
    pub fn uart(&self) -> UARTR {
        UARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Clock control for ADC"]
    #[inline]
    pub fn adc(&self) -> ADCR {
        ADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Clock control for WDT"]
    #[inline]
    pub fn wdt(&self) -> WDTR {
        WDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Clock control for the I/O configuration block"]
    #[inline]
    pub fn iocon(&self) -> IOCONR {
        IOCONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Clock control for C_CAN"]
    #[inline]
    pub fn can(&self) -> CANR {
        CANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Clock control for SPI1"]
    #[inline]
    pub fn ssp1(&self) -> SSP1R {
        SSP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2143 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Clock control for AHB to APB bridge, to the AHB matrix, to the Cortex-M0 FCLK and HCLK, to the SysCon, and to the PMU (this bit is read-only)"]
    #[inline]
    pub fn sys(&mut self) -> _SYSW {
        _SYSW { w: self }
    }
    #[doc = "Bit 1 - Clock control for ROM"]
    #[inline]
    pub fn rom(&mut self) -> _ROMW {
        _ROMW { w: self }
    }
    #[doc = "Bit 2 - Clock control for RAM"]
    #[inline]
    pub fn ram(&mut self) -> _RAMW {
        _RAMW { w: self }
    }
    #[doc = "Bit 3 - Clock control for the flash register interface"]
    #[inline]
    pub fn flashreg(&mut self) -> _FLASHREGW {
        _FLASHREGW { w: self }
    }
    #[doc = "Bit 4 - Clock control for flash array access"]
    #[inline]
    pub fn flasharray(&mut self) -> _FLASHARRAYW {
        _FLASHARRAYW { w: self }
    }
    #[doc = "Bit 5 - Clock control for I2C"]
    #[inline]
    pub fn i2c(&mut self) -> _I2CW {
        _I2CW { w: self }
    }
    #[doc = "Bit 6 - Clock control for GPIO"]
    #[inline]
    pub fn gpio(&mut self) -> _GPIOW {
        _GPIOW { w: self }
    }
    #[doc = "Bit 7 - Clock control for CT16B0"]
    #[inline]
    pub fn ct16b0(&mut self) -> _CT16B0W {
        _CT16B0W { w: self }
    }
    #[doc = "Bit 8 - Clock control for CT16B1"]
    #[inline]
    pub fn ct16b1(&mut self) -> _CT16B1W {
        _CT16B1W { w: self }
    }
    #[doc = "Bit 9 - Clock control for CT32B0"]
    #[inline]
    pub fn ct32b0(&mut self) -> _CT32B0W {
        _CT32B0W { w: self }
    }
    #[doc = "Bit 10 - Clock control for CT32B1"]
    #[inline]
    pub fn ct32b1(&mut self) -> _CT32B1W {
        _CT32B1W { w: self }
    }
    #[doc = "Bit 11 - Clock control for SPI0"]
    #[inline]
    pub fn ssp0(&mut self) -> _SSP0W {
        _SSP0W { w: self }
    }
    #[doc = "Bit 12 - Clock control for UART"]
    #[inline]
    pub fn uart(&mut self) -> _UARTW {
        _UARTW { w: self }
    }
    #[doc = "Bit 13 - Clock control for ADC"]
    #[inline]
    pub fn adc(&mut self) -> _ADCW {
        _ADCW { w: self }
    }
    #[doc = "Bit 15 - Clock control for WDT"]
    #[inline]
    pub fn wdt(&mut self) -> _WDTW {
        _WDTW { w: self }
    }
    #[doc = "Bit 16 - Clock control for the I/O configuration block"]
    #[inline]
    pub fn iocon(&mut self) -> _IOCONW {
        _IOCONW { w: self }
    }
    #[doc = "Bit 17 - Clock control for C_CAN"]
    #[inline]
    pub fn can(&mut self) -> _CANW {
        _CANW { w: self }
    }
    #[doc = "Bit 18 - Clock control for SPI1"]
    #[inline]
    pub fn ssp1(&mut self) -> _SSP1W {
        _SSP1W { w: self }
    }
}
