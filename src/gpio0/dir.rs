#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIR {
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
#[doc = "Possible values of the field `DIR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR0R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR0R {
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
            DIR0R::INPUT => false,
            DIR0R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR0R {
        match value {
            false => DIR0R::INPUT,
            true => DIR0R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR0R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR0R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR1R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR1R {
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
            DIR1R::INPUT => false,
            DIR1R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR1R {
        match value {
            false => DIR1R::INPUT,
            true => DIR1R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR1R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR1R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR2R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR2R {
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
            DIR2R::INPUT => false,
            DIR2R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR2R {
        match value {
            false => DIR2R::INPUT,
            true => DIR2R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR2R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR2R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR3R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR3R {
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
            DIR3R::INPUT => false,
            DIR3R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR3R {
        match value {
            false => DIR3R::INPUT,
            true => DIR3R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR3R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR3R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR4R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR4R {
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
            DIR4R::INPUT => false,
            DIR4R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR4R {
        match value {
            false => DIR4R::INPUT,
            true => DIR4R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR4R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR4R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR5R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR5R {
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
            DIR5R::INPUT => false,
            DIR5R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR5R {
        match value {
            false => DIR5R::INPUT,
            true => DIR5R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR5R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR5R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR6R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR6R {
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
            DIR6R::INPUT => false,
            DIR6R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR6R {
        match value {
            false => DIR6R::INPUT,
            true => DIR6R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR6R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR6R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR7R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR7R {
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
            DIR7R::INPUT => false,
            DIR7R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR7R {
        match value {
            false => DIR7R::INPUT,
            true => DIR7R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR7R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR7R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR8R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR8R {
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
            DIR8R::INPUT => false,
            DIR8R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR8R {
        match value {
            false => DIR8R::INPUT,
            true => DIR8R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR8R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR8R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR9R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR9R {
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
            DIR9R::INPUT => false,
            DIR9R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR9R {
        match value {
            false => DIR9R::INPUT,
            true => DIR9R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR9R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR9R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR10R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR10R {
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
            DIR10R::INPUT => false,
            DIR10R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR10R {
        match value {
            false => DIR10R::INPUT,
            true => DIR10R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR10R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR10R::OUTPUT
    }
}
#[doc = "Possible values of the field `DIR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR11R {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR11R {
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
            DIR11R::INPUT => false,
            DIR11R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR11R {
        match value {
            false => DIR11R::INPUT,
            true => DIR11R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIR11R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIR11R::OUTPUT
    }
}
#[doc = "Values that can be written to the field `DIR0`"]
pub enum DIR0W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR0W::INPUT => false,
            DIR0W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR0W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR0W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR0W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR1`"]
pub enum DIR1W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR1W::INPUT => false,
            DIR1W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR1W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR1W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR1W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR2`"]
pub enum DIR2W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR2W::INPUT => false,
            DIR2W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR2W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR2W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR2W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR3`"]
pub enum DIR3W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR3W::INPUT => false,
            DIR3W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR3W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR3W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR3W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR4`"]
pub enum DIR4W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR4W::INPUT => false,
            DIR4W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR4W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR4W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR4W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR5`"]
pub enum DIR5W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR5W::INPUT => false,
            DIR5W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR5W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR5W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR5W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR6`"]
pub enum DIR6W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR6W::INPUT => false,
            DIR6W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR6W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR6W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR6W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR7`"]
pub enum DIR7W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR7W::INPUT => false,
            DIR7W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR7W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR7W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR7W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR8`"]
pub enum DIR8W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR8W::INPUT => false,
            DIR8W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR8W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR8W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR8W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR9`"]
pub enum DIR9W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR9W::INPUT => false,
            DIR9W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR9W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR9W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR9W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR10`"]
pub enum DIR10W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR10W::INPUT => false,
            DIR10W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR10W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR10W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR10W::OUTPUT)
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
#[doc = "Values that can be written to the field `DIR11`"]
pub enum DIR11W {
    #[doc = "Pin is configured as an input"]
    INPUT,
    #[doc = "Pin is configured as an output"]
    OUTPUT,
}
impl DIR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIR11W::INPUT => false,
            DIR11W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIR11W<'a> {
    w: &'a mut W,
}
impl<'a> _DIR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIR11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR11W::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR11W::OUTPUT)
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
    pub fn dir0(&self) -> DIR0R {
        DIR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PIOn_1"]
    #[inline]
    pub fn dir1(&self) -> DIR1R {
        DIR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PIOn_2"]
    #[inline]
    pub fn dir2(&self) -> DIR2R {
        DIR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PIOn_3"]
    #[inline]
    pub fn dir3(&self) -> DIR3R {
        DIR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PIOn_4"]
    #[inline]
    pub fn dir4(&self) -> DIR4R {
        DIR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PIOn_5"]
    #[inline]
    pub fn dir5(&self) -> DIR5R {
        DIR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PIOn_6"]
    #[inline]
    pub fn dir6(&self) -> DIR6R {
        DIR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PIOn_7"]
    #[inline]
    pub fn dir7(&self) -> DIR7R {
        DIR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PIOn_8"]
    #[inline]
    pub fn dir8(&self) -> DIR8R {
        DIR8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PIOn_9"]
    #[inline]
    pub fn dir9(&self) -> DIR9R {
        DIR9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PIOn_10"]
    #[inline]
    pub fn dir10(&self) -> DIR10R {
        DIR10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PIOn_11"]
    #[inline]
    pub fn dir11(&self) -> DIR11R {
        DIR11R::_from({
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
    pub fn dir0(&mut self) -> _DIR0W {
        _DIR0W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1"]
    #[inline]
    pub fn dir1(&mut self) -> _DIR1W {
        _DIR1W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2"]
    #[inline]
    pub fn dir2(&mut self) -> _DIR2W {
        _DIR2W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3"]
    #[inline]
    pub fn dir3(&mut self) -> _DIR3W {
        _DIR3W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4"]
    #[inline]
    pub fn dir4(&mut self) -> _DIR4W {
        _DIR4W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5"]
    #[inline]
    pub fn dir5(&mut self) -> _DIR5W {
        _DIR5W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6"]
    #[inline]
    pub fn dir6(&mut self) -> _DIR6W {
        _DIR6W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7"]
    #[inline]
    pub fn dir7(&mut self) -> _DIR7W {
        _DIR7W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8"]
    #[inline]
    pub fn dir8(&mut self) -> _DIR8W {
        _DIR8W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9"]
    #[inline]
    pub fn dir9(&mut self) -> _DIR9W {
        _DIR9W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10"]
    #[inline]
    pub fn dir10(&mut self) -> _DIR10W {
        _DIR10W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11"]
    #[inline]
    pub fn dir11(&mut self) -> _DIR11W {
        _DIR11W { w: self }
    }
}
