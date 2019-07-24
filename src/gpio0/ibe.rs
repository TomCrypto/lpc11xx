#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IBE {
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
#[doc = "Possible values of the field `IBE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE0R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE0R {
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
            IBE0R::IEV => false,
            IBE0R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE0R {
        match value {
            false => IBE0R::IEV,
            true => IBE0R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE0R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE0R::BOTH
    }
}
#[doc = "Possible values of the field `IBE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE1R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE1R {
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
            IBE1R::IEV => false,
            IBE1R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE1R {
        match value {
            false => IBE1R::IEV,
            true => IBE1R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE1R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE1R::BOTH
    }
}
#[doc = "Possible values of the field `IBE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE2R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE2R {
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
            IBE2R::IEV => false,
            IBE2R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE2R {
        match value {
            false => IBE2R::IEV,
            true => IBE2R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE2R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE2R::BOTH
    }
}
#[doc = "Possible values of the field `IBE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE3R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE3R {
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
            IBE3R::IEV => false,
            IBE3R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE3R {
        match value {
            false => IBE3R::IEV,
            true => IBE3R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE3R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE3R::BOTH
    }
}
#[doc = "Possible values of the field `IBE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE4R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE4R {
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
            IBE4R::IEV => false,
            IBE4R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE4R {
        match value {
            false => IBE4R::IEV,
            true => IBE4R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE4R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE4R::BOTH
    }
}
#[doc = "Possible values of the field `IBE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE5R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE5R {
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
            IBE5R::IEV => false,
            IBE5R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE5R {
        match value {
            false => IBE5R::IEV,
            true => IBE5R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE5R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE5R::BOTH
    }
}
#[doc = "Possible values of the field `IBE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE6R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE6R {
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
            IBE6R::IEV => false,
            IBE6R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE6R {
        match value {
            false => IBE6R::IEV,
            true => IBE6R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE6R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE6R::BOTH
    }
}
#[doc = "Possible values of the field `IBE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE7R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE7R {
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
            IBE7R::IEV => false,
            IBE7R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE7R {
        match value {
            false => IBE7R::IEV,
            true => IBE7R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE7R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE7R::BOTH
    }
}
#[doc = "Possible values of the field `IBE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE8R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE8R {
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
            IBE8R::IEV => false,
            IBE8R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE8R {
        match value {
            false => IBE8R::IEV,
            true => IBE8R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE8R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE8R::BOTH
    }
}
#[doc = "Possible values of the field `IBE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE9R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE9R {
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
            IBE9R::IEV => false,
            IBE9R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE9R {
        match value {
            false => IBE9R::IEV,
            true => IBE9R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE9R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE9R::BOTH
    }
}
#[doc = "Possible values of the field `IBE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE10R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE10R {
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
            IBE10R::IEV => false,
            IBE10R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE10R {
        match value {
            false => IBE10R::IEV,
            true => IBE10R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE10R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE10R::BOTH
    }
}
#[doc = "Possible values of the field `IBE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE11R {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE11R {
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
            IBE11R::IEV => false,
            IBE11R::BOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBE11R {
        match value {
            false => IBE11R::IEV,
            true => IBE11R::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline]
    pub fn is_iev(&self) -> bool {
        *self == IBE11R::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == IBE11R::BOTH
    }
}
#[doc = "Values that can be written to the field `IBE0`"]
pub enum IBE0W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE0W::IEV => false,
            IBE0W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE0W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE0W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE0W::BOTH)
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
#[doc = "Values that can be written to the field `IBE1`"]
pub enum IBE1W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE1W::IEV => false,
            IBE1W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE1W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE1W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE1W::BOTH)
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
#[doc = "Values that can be written to the field `IBE2`"]
pub enum IBE2W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE2W::IEV => false,
            IBE2W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE2W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE2W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE2W::BOTH)
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
#[doc = "Values that can be written to the field `IBE3`"]
pub enum IBE3W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE3W::IEV => false,
            IBE3W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE3W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE3W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE3W::BOTH)
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
#[doc = "Values that can be written to the field `IBE4`"]
pub enum IBE4W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE4W::IEV => false,
            IBE4W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE4W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE4W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE4W::BOTH)
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
#[doc = "Values that can be written to the field `IBE5`"]
pub enum IBE5W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE5W::IEV => false,
            IBE5W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE5W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE5W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE5W::BOTH)
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
#[doc = "Values that can be written to the field `IBE6`"]
pub enum IBE6W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE6W::IEV => false,
            IBE6W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE6W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE6W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE6W::BOTH)
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
#[doc = "Values that can be written to the field `IBE7`"]
pub enum IBE7W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE7W::IEV => false,
            IBE7W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE7W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE7W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE7W::BOTH)
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
#[doc = "Values that can be written to the field `IBE8`"]
pub enum IBE8W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE8W::IEV => false,
            IBE8W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE8W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE8W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE8W::BOTH)
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
#[doc = "Values that can be written to the field `IBE9`"]
pub enum IBE9W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE9W::IEV => false,
            IBE9W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE9W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE9W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE9W::BOTH)
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
#[doc = "Values that can be written to the field `IBE10`"]
pub enum IBE10W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE10W::IEV => false,
            IBE10W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE10W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE10W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE10W::BOTH)
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
#[doc = "Values that can be written to the field `IBE11`"]
pub enum IBE11W {
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl IBE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBE11W::IEV => false,
            IBE11W::BOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBE11W<'a> {
    w: &'a mut W,
}
impl<'a> _IBE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBE11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE11W::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE11W::BOTH)
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
    pub fn ibe0(&self) -> IBE0R {
        IBE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PIOn_1"]
    #[inline]
    pub fn ibe1(&self) -> IBE1R {
        IBE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PIOn_2"]
    #[inline]
    pub fn ibe2(&self) -> IBE2R {
        IBE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PIOn_3"]
    #[inline]
    pub fn ibe3(&self) -> IBE3R {
        IBE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PIOn_4"]
    #[inline]
    pub fn ibe4(&self) -> IBE4R {
        IBE4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PIOn_5"]
    #[inline]
    pub fn ibe5(&self) -> IBE5R {
        IBE5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PIOn_6"]
    #[inline]
    pub fn ibe6(&self) -> IBE6R {
        IBE6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PIOn_7"]
    #[inline]
    pub fn ibe7(&self) -> IBE7R {
        IBE7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PIOn_8"]
    #[inline]
    pub fn ibe8(&self) -> IBE8R {
        IBE8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PIOn_9"]
    #[inline]
    pub fn ibe9(&self) -> IBE9R {
        IBE9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PIOn_10"]
    #[inline]
    pub fn ibe10(&self) -> IBE10R {
        IBE10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PIOn_11"]
    #[inline]
    pub fn ibe11(&self) -> IBE11R {
        IBE11R::_from({
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
    pub fn ibe0(&mut self) -> _IBE0W {
        _IBE0W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1"]
    #[inline]
    pub fn ibe1(&mut self) -> _IBE1W {
        _IBE1W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2"]
    #[inline]
    pub fn ibe2(&mut self) -> _IBE2W {
        _IBE2W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3"]
    #[inline]
    pub fn ibe3(&mut self) -> _IBE3W {
        _IBE3W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4"]
    #[inline]
    pub fn ibe4(&mut self) -> _IBE4W {
        _IBE4W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5"]
    #[inline]
    pub fn ibe5(&mut self) -> _IBE5W {
        _IBE5W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6"]
    #[inline]
    pub fn ibe6(&mut self) -> _IBE6W {
        _IBE6W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7"]
    #[inline]
    pub fn ibe7(&mut self) -> _IBE7W {
        _IBE7W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8"]
    #[inline]
    pub fn ibe8(&mut self) -> _IBE8W {
        _IBE8W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9"]
    #[inline]
    pub fn ibe9(&mut self) -> _IBE9W {
        _IBE9W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10"]
    #[inline]
    pub fn ibe10(&mut self) -> _IBE10W {
        _IBE10W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11"]
    #[inline]
    pub fn ibe11(&mut self) -> _IBE11W {
        _IBE11W { w: self }
    }
}
