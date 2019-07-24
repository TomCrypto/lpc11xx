#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IC {
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
}
#[doc = "Values that can be written to the field `CLR0`"]
pub enum CLR0W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR0W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR0W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR1`"]
pub enum CLR1W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR1W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR1W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR2`"]
pub enum CLR2W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR2W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR2W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR3`"]
pub enum CLR3W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR3W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR3W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR4`"]
pub enum CLR4W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR4W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR4W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR5`"]
pub enum CLR5W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR5W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR5W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR6`"]
pub enum CLR6W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR6W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR6W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR6W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR7`"]
pub enum CLR7W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR7W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR7W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR7W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR8`"]
pub enum CLR8W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR8W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR8W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR8W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR9`"]
pub enum CLR9W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR9W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR9W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR9W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR10`"]
pub enum CLR10W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR10W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR10W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR10W::CLEAR_INTERRUPT)
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
#[doc = "Values that can be written to the field `CLR11`"]
pub enum CLR11W {
    #[doc = "Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl CLR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR11W::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR11W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR11W::CLEAR_INTERRUPT)
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
    pub fn clr0(&mut self) -> _CLR0W {
        _CLR0W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1"]
    #[inline]
    pub fn clr1(&mut self) -> _CLR1W {
        _CLR1W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2"]
    #[inline]
    pub fn clr2(&mut self) -> _CLR2W {
        _CLR2W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3"]
    #[inline]
    pub fn clr3(&mut self) -> _CLR3W {
        _CLR3W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4"]
    #[inline]
    pub fn clr4(&mut self) -> _CLR4W {
        _CLR4W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5"]
    #[inline]
    pub fn clr5(&mut self) -> _CLR5W {
        _CLR5W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6"]
    #[inline]
    pub fn clr6(&mut self) -> _CLR6W {
        _CLR6W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7"]
    #[inline]
    pub fn clr7(&mut self) -> _CLR7W {
        _CLR7W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8"]
    #[inline]
    pub fn clr8(&mut self) -> _CLR8W {
        _CLR8W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9"]
    #[inline]
    pub fn clr9(&mut self) -> _CLR9W {
        _CLR9W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10"]
    #[inline]
    pub fn clr10(&mut self) -> _CLR10W {
        _CLR10W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11"]
    #[inline]
    pub fn clr11(&mut self) -> _CLR11W {
        _CLR11W { w: self }
    }
}
