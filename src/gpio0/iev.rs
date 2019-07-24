#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IEV {
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
#[doc = "Possible values of the field `IEV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV0R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV0R {
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
            IEV0R::FALLING => false,
            IEV0R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV0R {
        match value {
            false => IEV0R::FALLING,
            true => IEV0R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV0R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV0R::RISING
    }
}
#[doc = "Possible values of the field `IEV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV1R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV1R {
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
            IEV1R::FALLING => false,
            IEV1R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV1R {
        match value {
            false => IEV1R::FALLING,
            true => IEV1R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV1R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV1R::RISING
    }
}
#[doc = "Possible values of the field `IEV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV2R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV2R {
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
            IEV2R::FALLING => false,
            IEV2R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV2R {
        match value {
            false => IEV2R::FALLING,
            true => IEV2R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV2R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV2R::RISING
    }
}
#[doc = "Possible values of the field `IEV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV3R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV3R {
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
            IEV3R::FALLING => false,
            IEV3R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV3R {
        match value {
            false => IEV3R::FALLING,
            true => IEV3R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV3R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV3R::RISING
    }
}
#[doc = "Possible values of the field `IEV4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV4R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV4R {
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
            IEV4R::FALLING => false,
            IEV4R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV4R {
        match value {
            false => IEV4R::FALLING,
            true => IEV4R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV4R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV4R::RISING
    }
}
#[doc = "Possible values of the field `IEV5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV5R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV5R {
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
            IEV5R::FALLING => false,
            IEV5R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV5R {
        match value {
            false => IEV5R::FALLING,
            true => IEV5R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV5R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV5R::RISING
    }
}
#[doc = "Possible values of the field `IEV6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV6R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV6R {
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
            IEV6R::FALLING => false,
            IEV6R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV6R {
        match value {
            false => IEV6R::FALLING,
            true => IEV6R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV6R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV6R::RISING
    }
}
#[doc = "Possible values of the field `IEV7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV7R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV7R {
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
            IEV7R::FALLING => false,
            IEV7R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV7R {
        match value {
            false => IEV7R::FALLING,
            true => IEV7R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV7R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV7R::RISING
    }
}
#[doc = "Possible values of the field `IEV8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV8R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV8R {
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
            IEV8R::FALLING => false,
            IEV8R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV8R {
        match value {
            false => IEV8R::FALLING,
            true => IEV8R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV8R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV8R::RISING
    }
}
#[doc = "Possible values of the field `IEV9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV9R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV9R {
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
            IEV9R::FALLING => false,
            IEV9R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV9R {
        match value {
            false => IEV9R::FALLING,
            true => IEV9R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV9R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV9R::RISING
    }
}
#[doc = "Possible values of the field `IEV10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV10R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV10R {
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
            IEV10R::FALLING => false,
            IEV10R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV10R {
        match value {
            false => IEV10R::FALLING,
            true => IEV10R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV10R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV10R::RISING
    }
}
#[doc = "Possible values of the field `IEV11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV11R {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV11R {
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
            IEV11R::FALLING => false,
            IEV11R::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEV11R {
        match value {
            false => IEV11R::FALLING,
            true => IEV11R::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == IEV11R::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == IEV11R::RISING
    }
}
#[doc = "Values that can be written to the field `IEV0`"]
pub enum IEV0W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV0W::FALLING => false,
            IEV0W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV0W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV0W::RISING)
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
#[doc = "Values that can be written to the field `IEV1`"]
pub enum IEV1W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV1W::FALLING => false,
            IEV1W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV1W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV1W::RISING)
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
#[doc = "Values that can be written to the field `IEV2`"]
pub enum IEV2W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV2W::FALLING => false,
            IEV2W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV2W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV2W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV2W::RISING)
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
#[doc = "Values that can be written to the field `IEV3`"]
pub enum IEV3W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV3W::FALLING => false,
            IEV3W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV3W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV3W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV3W::RISING)
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
#[doc = "Values that can be written to the field `IEV4`"]
pub enum IEV4W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV4W::FALLING => false,
            IEV4W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV4W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV4W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV4W::RISING)
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
#[doc = "Values that can be written to the field `IEV5`"]
pub enum IEV5W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV5W::FALLING => false,
            IEV5W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV5W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV5W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV5W::RISING)
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
#[doc = "Values that can be written to the field `IEV6`"]
pub enum IEV6W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV6W::FALLING => false,
            IEV6W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV6W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV6W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV6W::RISING)
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
#[doc = "Values that can be written to the field `IEV7`"]
pub enum IEV7W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV7W::FALLING => false,
            IEV7W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV7W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV7W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV7W::RISING)
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
#[doc = "Values that can be written to the field `IEV8`"]
pub enum IEV8W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV8W::FALLING => false,
            IEV8W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV8W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV8W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV8W::RISING)
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
#[doc = "Values that can be written to the field `IEV9`"]
pub enum IEV9W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV9W::FALLING => false,
            IEV9W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV9W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV9W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV9W::RISING)
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
#[doc = "Values that can be written to the field `IEV10`"]
pub enum IEV10W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV10W::FALLING => false,
            IEV10W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV10W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV10W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV10W::RISING)
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
#[doc = "Values that can be written to the field `IEV11`"]
pub enum IEV11W {
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl IEV11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEV11W::FALLING => false,
            IEV11W::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEV11W<'a> {
    w: &'a mut W,
}
impl<'a> _IEV11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEV11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV11W::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV11W::RISING)
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
    #[doc = "Bit 0 - PIOn_0"]
    #[inline]
    pub fn iev0(&self) -> IEV0R {
        IEV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PIOn_1"]
    #[inline]
    pub fn iev1(&self) -> IEV1R {
        IEV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PIOn_2"]
    #[inline]
    pub fn iev2(&self) -> IEV2R {
        IEV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PIOn_3"]
    #[inline]
    pub fn iev3(&self) -> IEV3R {
        IEV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PIOn_4"]
    #[inline]
    pub fn iev4(&self) -> IEV4R {
        IEV4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PIOn_5"]
    #[inline]
    pub fn iev5(&self) -> IEV5R {
        IEV5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PIOn_6"]
    #[inline]
    pub fn iev6(&self) -> IEV6R {
        IEV6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PIOn_7"]
    #[inline]
    pub fn iev7(&self) -> IEV7R {
        IEV7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PIOn_8"]
    #[inline]
    pub fn iev8(&self) -> IEV8R {
        IEV8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PIOn_9"]
    #[inline]
    pub fn iev9(&self) -> IEV9R {
        IEV9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PIOn_10"]
    #[inline]
    pub fn iev10(&self) -> IEV10R {
        IEV10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PIOn_11"]
    #[inline]
    pub fn iev11(&self) -> IEV11R {
        IEV11R::_from({
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
    #[doc = "Bit 0 - PIOn_0"]
    #[inline]
    pub fn iev0(&mut self) -> _IEV0W {
        _IEV0W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1"]
    #[inline]
    pub fn iev1(&mut self) -> _IEV1W {
        _IEV1W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2"]
    #[inline]
    pub fn iev2(&mut self) -> _IEV2W {
        _IEV2W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3"]
    #[inline]
    pub fn iev3(&mut self) -> _IEV3W {
        _IEV3W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4"]
    #[inline]
    pub fn iev4(&mut self) -> _IEV4W {
        _IEV4W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5"]
    #[inline]
    pub fn iev5(&mut self) -> _IEV5W {
        _IEV5W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6"]
    #[inline]
    pub fn iev6(&mut self) -> _IEV6W {
        _IEV6W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7"]
    #[inline]
    pub fn iev7(&mut self) -> _IEV7W {
        _IEV7W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8"]
    #[inline]
    pub fn iev8(&mut self) -> _IEV8W {
        _IEV8W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9"]
    #[inline]
    pub fn iev9(&mut self) -> _IEV9W {
        _IEV9W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10"]
    #[inline]
    pub fn iev10(&mut self) -> _IEV10W {
        _IEV10W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11"]
    #[inline]
    pub fn iev11(&mut self) -> _IEV11W {
        _IEV11W { w: self }
    }
}
