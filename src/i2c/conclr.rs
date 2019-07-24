#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONCLR {
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
#[doc = "Values that can be written to the field `AA`"]
pub enum AAW {
    #[doc = "Clear the flag"]
    CLEAR,
}
impl AAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AAW<'a> {
    w: &'a mut W,
}
impl<'a> _AAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(AAW::CLEAR)
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
#[doc = "Values that can be written to the field `SI`"]
pub enum SIW {
    #[doc = "Clear the flag"]
    CLEAR,
}
impl SIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIW<'a> {
    w: &'a mut W,
}
impl<'a> _SIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SIW::CLEAR)
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
#[doc = "Values that can be written to the field `STA`"]
pub enum STAW {
    #[doc = "Clear the flag"]
    CLEAR,
}
impl STAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STAW<'a> {
    w: &'a mut W,
}
impl<'a> _STAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(STAW::CLEAR)
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
#[doc = "Values that can be written to the field `I2EN`"]
pub enum I2ENW {
    #[doc = "Disable the I2C interface"]
    DISABLE,
}
impl I2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2ENW::DISABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the I2C interface"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2ENW::DISABLE)
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
    #[doc = "Bit 2 - Assert Acknowledge flag"]
    #[inline]
    pub fn aa(&mut self) -> _AAW {
        _AAW { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag"]
    #[inline]
    pub fn si(&mut self) -> _SIW {
        _SIW { w: self }
    }
    #[doc = "Bit 5 - START flag"]
    #[inline]
    pub fn sta(&mut self) -> _STAW {
        _STAW { w: self }
    }
    #[doc = "Bit 6 - I2C Interface Disable bit"]
    #[inline]
    pub fn i2en(&mut self) -> _I2ENW {
        _I2ENW { w: self }
    }
}
