#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONSET {
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
#[doc = "Possible values of the field `AA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAR {
    #[doc = "Set the flag"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AAR {
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
            AAR::SET => true,
            AAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AAR {
        match value {
            true => AAR::SET,
            i => AAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == AAR::SET
    }
}
#[doc = "Possible values of the field `SI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIR {
    #[doc = "Set the flag"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SIR {
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
            SIR::SET => true,
            SIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIR {
        match value {
            true => SIR::SET,
            i => SIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == SIR::SET
    }
}
#[doc = "Possible values of the field `STO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOR {
    #[doc = "Set the flag"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl STOR {
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
            STOR::SET => true,
            STOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOR {
        match value {
            true => STOR::SET,
            i => STOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == STOR::SET
    }
}
#[doc = "Possible values of the field `STA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STAR {
    #[doc = "Set the flag"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl STAR {
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
            STAR::SET => true,
            STAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STAR {
        match value {
            true => STAR::SET,
            i => STAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == STAR::SET
    }
}
#[doc = "Possible values of the field `I2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2ENR {
    #[doc = "Enable the I2C interface"]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl I2ENR {
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
            I2ENR::ENABLE => true,
            I2ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2ENR {
        match value {
            true => I2ENR::ENABLE,
            i => I2ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == I2ENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `AA`"]
pub enum AAW {
    #[doc = "Set the flag"]
    SET,
}
impl AAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AAW::SET => true,
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
    #[doc = "Set the flag"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(AAW::SET)
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
    #[doc = "Set the flag"]
    SET,
}
impl SIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIW::SET => true,
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
    #[doc = "Set the flag"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SIW::SET)
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
#[doc = "Values that can be written to the field `STO`"]
pub enum STOW {
    #[doc = "Set the flag"]
    SET,
}
impl STOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOW<'a> {
    w: &'a mut W,
}
impl<'a> _STOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set the flag"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(STOW::SET)
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
#[doc = "Values that can be written to the field `STA`"]
pub enum STAW {
    #[doc = "Set the flag"]
    SET,
}
impl STAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STAW::SET => true,
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
    #[doc = "Set the flag"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(STAW::SET)
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
    #[doc = "Enable the I2C interface"]
    ENABLE,
}
impl I2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2ENW::ENABLE => true,
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
    #[doc = "Enable the I2C interface"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2ENW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Assert Acknowledge flag."]
    #[inline]
    pub fn aa(&self) -> AAR {
        AAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt flag."]
    #[inline]
    pub fn si(&self) -> SIR {
        SIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline]
    pub fn sto(&self) -> STOR {
        STOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - START flag."]
    #[inline]
    pub fn sta(&self) -> STAR {
        STAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - I2C Interface Enable bit."]
    #[inline]
    pub fn i2en(&self) -> I2ENR {
        I2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 2 - Assert Acknowledge flag."]
    #[inline]
    pub fn aa(&mut self) -> _AAW {
        _AAW { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag."]
    #[inline]
    pub fn si(&mut self) -> _SIW {
        _SIW { w: self }
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline]
    pub fn sto(&mut self) -> _STOW {
        _STOW { w: self }
    }
    #[doc = "Bit 5 - START flag."]
    #[inline]
    pub fn sta(&mut self) -> _STAW {
        _STAW { w: self }
    }
    #[doc = "Bit 6 - I2C Interface Enable bit."]
    #[inline]
    pub fn i2en(&mut self) -> _I2ENW {
        _I2ENW { w: self }
    }
}
