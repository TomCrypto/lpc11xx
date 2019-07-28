#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IS {
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
#[doc = "Possible values of the field `ISENSE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE0R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE0R {
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
            ISENSE0R::EDGE => false,
            ISENSE0R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE0R {
        match value {
            false => ISENSE0R::EDGE,
            true => ISENSE0R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE0R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE0R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE1R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE1R {
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
            ISENSE1R::EDGE => false,
            ISENSE1R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE1R {
        match value {
            false => ISENSE1R::EDGE,
            true => ISENSE1R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE1R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE1R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE2R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE2R {
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
            ISENSE2R::EDGE => false,
            ISENSE2R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE2R {
        match value {
            false => ISENSE2R::EDGE,
            true => ISENSE2R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE2R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE2R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE3R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE3R {
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
            ISENSE3R::EDGE => false,
            ISENSE3R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE3R {
        match value {
            false => ISENSE3R::EDGE,
            true => ISENSE3R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE3R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE3R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE4R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE4R {
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
            ISENSE4R::EDGE => false,
            ISENSE4R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE4R {
        match value {
            false => ISENSE4R::EDGE,
            true => ISENSE4R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE4R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE4R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE5R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE5R {
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
            ISENSE5R::EDGE => false,
            ISENSE5R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE5R {
        match value {
            false => ISENSE5R::EDGE,
            true => ISENSE5R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE5R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE5R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE6R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE6R {
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
            ISENSE6R::EDGE => false,
            ISENSE6R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE6R {
        match value {
            false => ISENSE6R::EDGE,
            true => ISENSE6R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE6R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE6R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE7R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE7R {
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
            ISENSE7R::EDGE => false,
            ISENSE7R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE7R {
        match value {
            false => ISENSE7R::EDGE,
            true => ISENSE7R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE7R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE7R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE8R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE8R {
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
            ISENSE8R::EDGE => false,
            ISENSE8R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE8R {
        match value {
            false => ISENSE8R::EDGE,
            true => ISENSE8R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE8R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE8R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE9R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE9R {
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
            ISENSE9R::EDGE => false,
            ISENSE9R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE9R {
        match value {
            false => ISENSE9R::EDGE,
            true => ISENSE9R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE9R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE9R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE10R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE10R {
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
            ISENSE10R::EDGE => false,
            ISENSE10R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE10R {
        match value {
            false => ISENSE10R::EDGE,
            true => ISENSE10R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE10R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE10R::LEVEL
    }
}
#[doc = "Possible values of the field `ISENSE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE11R {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE11R {
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
            ISENSE11R::EDGE => false,
            ISENSE11R::LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISENSE11R {
        match value {
            false => ISENSE11R::EDGE,
            true => ISENSE11R::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE11R::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENSE11R::LEVEL
    }
}
#[doc = "Values that can be written to the field `ISENSE0`"]
pub enum ISENSE0W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE0W::EDGE => false,
            ISENSE0W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE0W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE0W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE0W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE1`"]
pub enum ISENSE1W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE1W::EDGE => false,
            ISENSE1W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE1W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE1W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE1W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE2`"]
pub enum ISENSE2W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE2W::EDGE => false,
            ISENSE2W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE2W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE2W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE2W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE3`"]
pub enum ISENSE3W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE3W::EDGE => false,
            ISENSE3W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE3W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE3W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE3W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE4`"]
pub enum ISENSE4W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE4W::EDGE => false,
            ISENSE4W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE4W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE4W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE4W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE5`"]
pub enum ISENSE5W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE5W::EDGE => false,
            ISENSE5W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE5W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE5W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE5W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE6`"]
pub enum ISENSE6W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE6W::EDGE => false,
            ISENSE6W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE6W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE6W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE6W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE7`"]
pub enum ISENSE7W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE7W::EDGE => false,
            ISENSE7W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE7W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE7W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE7W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE8`"]
pub enum ISENSE8W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE8W::EDGE => false,
            ISENSE8W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE8W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE8W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE8W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE9`"]
pub enum ISENSE9W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE9W::EDGE => false,
            ISENSE9W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE9W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE9W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE9W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE10`"]
pub enum ISENSE10W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE10W::EDGE => false,
            ISENSE10W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE10W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE10W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE10W::LEVEL)
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
#[doc = "Values that can be written to the field `ISENSE11`"]
pub enum ISENSE11W {
    #[doc = "Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "Pin interrupt is level-sensitive"]
    LEVEL,
}
impl ISENSE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISENSE11W::EDGE => false,
            ISENSE11W::LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENSE11W<'a> {
    w: &'a mut W,
}
impl<'a> _ISENSE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENSE11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE11W::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE11W::LEVEL)
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
    pub fn isense0(&self) -> ISENSE0R {
        ISENSE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline]
    pub fn isense1(&self) -> ISENSE1R {
        ISENSE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline]
    pub fn isense2(&self) -> ISENSE2R {
        ISENSE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline]
    pub fn isense3(&self) -> ISENSE3R {
        ISENSE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline]
    pub fn isense4(&self) -> ISENSE4R {
        ISENSE4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline]
    pub fn isense5(&self) -> ISENSE5R {
        ISENSE5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline]
    pub fn isense6(&self) -> ISENSE6R {
        ISENSE6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline]
    pub fn isense7(&self) -> ISENSE7R {
        ISENSE7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline]
    pub fn isense8(&self) -> ISENSE8R {
        ISENSE8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline]
    pub fn isense9(&self) -> ISENSE9R {
        ISENSE9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline]
    pub fn isense10(&self) -> ISENSE10R {
        ISENSE10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline]
    pub fn isense11(&self) -> ISENSE11R {
        ISENSE11R::_from({
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
    pub fn isense0(&mut self) -> _ISENSE0W {
        _ISENSE0W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline]
    pub fn isense1(&mut self) -> _ISENSE1W {
        _ISENSE1W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline]
    pub fn isense2(&mut self) -> _ISENSE2W {
        _ISENSE2W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline]
    pub fn isense3(&mut self) -> _ISENSE3W {
        _ISENSE3W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline]
    pub fn isense4(&mut self) -> _ISENSE4W {
        _ISENSE4W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline]
    pub fn isense5(&mut self) -> _ISENSE5W {
        _ISENSE5W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline]
    pub fn isense6(&mut self) -> _ISENSE6W {
        _ISENSE6W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline]
    pub fn isense7(&mut self) -> _ISENSE7W {
        _ISENSE7W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline]
    pub fn isense8(&mut self) -> _ISENSE8W {
        _ISENSE8W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline]
    pub fn isense9(&mut self) -> _ISENSE9W {
        _ISENSE9W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline]
    pub fn isense10(&mut self) -> _ISENSE10W {
        _ISENSE10W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline]
    pub fn isense11(&mut self) -> _ISENSE11W {
        _ISENSE11W { w: self }
    }
}
