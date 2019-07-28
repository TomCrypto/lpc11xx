#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DATA {
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
#[doc = "Possible values of the field `DATA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA0R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA0R {
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
            DATA0R::LOW => false,
            DATA0R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA0R {
        match value {
            false => DATA0R::LOW,
            true => DATA0R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA0R::HIGH
    }
}
#[doc = "Possible values of the field `DATA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA1R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA1R {
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
            DATA1R::LOW => false,
            DATA1R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA1R {
        match value {
            false => DATA1R::LOW,
            true => DATA1R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA1R::HIGH
    }
}
#[doc = "Possible values of the field `DATA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA2R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA2R {
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
            DATA2R::LOW => false,
            DATA2R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA2R {
        match value {
            false => DATA2R::LOW,
            true => DATA2R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA2R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA2R::HIGH
    }
}
#[doc = "Possible values of the field `DATA3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA3R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA3R {
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
            DATA3R::LOW => false,
            DATA3R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA3R {
        match value {
            false => DATA3R::LOW,
            true => DATA3R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA3R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA3R::HIGH
    }
}
#[doc = "Possible values of the field `DATA4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA4R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA4R {
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
            DATA4R::LOW => false,
            DATA4R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA4R {
        match value {
            false => DATA4R::LOW,
            true => DATA4R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA4R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA4R::HIGH
    }
}
#[doc = "Possible values of the field `DATA5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA5R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA5R {
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
            DATA5R::LOW => false,
            DATA5R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA5R {
        match value {
            false => DATA5R::LOW,
            true => DATA5R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA5R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA5R::HIGH
    }
}
#[doc = "Possible values of the field `DATA6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA6R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA6R {
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
            DATA6R::LOW => false,
            DATA6R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA6R {
        match value {
            false => DATA6R::LOW,
            true => DATA6R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA6R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA6R::HIGH
    }
}
#[doc = "Possible values of the field `DATA7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA7R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA7R {
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
            DATA7R::LOW => false,
            DATA7R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA7R {
        match value {
            false => DATA7R::LOW,
            true => DATA7R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA7R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA7R::HIGH
    }
}
#[doc = "Possible values of the field `DATA8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA8R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA8R {
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
            DATA8R::LOW => false,
            DATA8R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA8R {
        match value {
            false => DATA8R::LOW,
            true => DATA8R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA8R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA8R::HIGH
    }
}
#[doc = "Possible values of the field `DATA9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA9R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA9R {
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
            DATA9R::LOW => false,
            DATA9R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA9R {
        match value {
            false => DATA9R::LOW,
            true => DATA9R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA9R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA9R::HIGH
    }
}
#[doc = "Possible values of the field `DATA10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA10R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA10R {
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
            DATA10R::LOW => false,
            DATA10R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA10R {
        match value {
            false => DATA10R::LOW,
            true => DATA10R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA10R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA10R::HIGH
    }
}
#[doc = "Possible values of the field `DATA11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA11R {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA11R {
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
            DATA11R::LOW => false,
            DATA11R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA11R {
        match value {
            false => DATA11R::LOW,
            true => DATA11R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DATA11R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DATA11R::HIGH
    }
}
#[doc = "Values that can be written to the field `DATA0`"]
pub enum DATA0W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA0W::LOW => false,
            DATA0W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA0W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA0W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA0W::HIGH)
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
#[doc = "Values that can be written to the field `DATA1`"]
pub enum DATA1W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA1W::LOW => false,
            DATA1W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA1W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA1W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA1W::HIGH)
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
#[doc = "Values that can be written to the field `DATA2`"]
pub enum DATA2W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA2W::LOW => false,
            DATA2W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA2W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA2W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA2W::HIGH)
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
#[doc = "Values that can be written to the field `DATA3`"]
pub enum DATA3W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA3W::LOW => false,
            DATA3W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA3W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA3W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA3W::HIGH)
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
#[doc = "Values that can be written to the field `DATA4`"]
pub enum DATA4W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA4W::LOW => false,
            DATA4W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA4W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA4W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA4W::HIGH)
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
#[doc = "Values that can be written to the field `DATA5`"]
pub enum DATA5W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA5W::LOW => false,
            DATA5W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA5W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA5W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA5W::HIGH)
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
#[doc = "Values that can be written to the field `DATA6`"]
pub enum DATA6W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA6W::LOW => false,
            DATA6W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA6W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA6W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA6W::HIGH)
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
#[doc = "Values that can be written to the field `DATA7`"]
pub enum DATA7W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA7W::LOW => false,
            DATA7W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA7W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA7W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA7W::HIGH)
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
#[doc = "Values that can be written to the field `DATA8`"]
pub enum DATA8W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA8W::LOW => false,
            DATA8W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA8W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA8W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA8W::HIGH)
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
#[doc = "Values that can be written to the field `DATA9`"]
pub enum DATA9W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA9W::LOW => false,
            DATA9W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA9W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA9W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA9W::HIGH)
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
#[doc = "Values that can be written to the field `DATA10`"]
pub enum DATA10W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA10W::LOW => false,
            DATA10W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA10W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA10W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA10W::HIGH)
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
#[doc = "Values that can be written to the field `DATA11`"]
pub enum DATA11W {
    #[doc = "Pin is driven low"]
    LOW,
    #[doc = "Pin is driven high"]
    HIGH,
}
impl DATA11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA11W::LOW => false,
            DATA11W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA11W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA11W::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA11W::HIGH)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - PIOn_0."]
    #[inline]
    pub fn data0(&self) -> DATA0R {
        DATA0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline]
    pub fn data1(&self) -> DATA1R {
        DATA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline]
    pub fn data2(&self) -> DATA2R {
        DATA2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline]
    pub fn data3(&self) -> DATA3R {
        DATA3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline]
    pub fn data4(&self) -> DATA4R {
        DATA4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline]
    pub fn data5(&self) -> DATA5R {
        DATA5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline]
    pub fn data6(&self) -> DATA6R {
        DATA6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline]
    pub fn data7(&self) -> DATA7R {
        DATA7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline]
    pub fn data8(&self) -> DATA8R {
        DATA8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline]
    pub fn data9(&self) -> DATA9R {
        DATA9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline]
    pub fn data10(&self) -> DATA10R {
        DATA10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline]
    pub fn data11(&self) -> DATA11R {
        DATA11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - PIOn_0."]
    #[inline]
    pub fn data0(&mut self) -> _DATA0W {
        _DATA0W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline]
    pub fn data1(&mut self) -> _DATA1W {
        _DATA1W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline]
    pub fn data2(&mut self) -> _DATA2W {
        _DATA2W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline]
    pub fn data3(&mut self) -> _DATA3W {
        _DATA3W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline]
    pub fn data4(&mut self) -> _DATA4W {
        _DATA4W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline]
    pub fn data5(&mut self) -> _DATA5W {
        _DATA5W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline]
    pub fn data6(&mut self) -> _DATA6W {
        _DATA6W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline]
    pub fn data7(&mut self) -> _DATA7W {
        _DATA7W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline]
    pub fn data8(&mut self) -> _DATA8W {
        _DATA8W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline]
    pub fn data9(&mut self) -> _DATA9W {
        _DATA9W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline]
    pub fn data10(&mut self) -> _DATA10W {
        _DATA10W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline]
    pub fn data11(&mut self) -> _DATA11W {
        _DATA11W { w: self }
    }
}
