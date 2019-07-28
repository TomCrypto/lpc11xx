#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCON_PIO0_5 {
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
    #[doc = "Pin function PIO0_5 (open-drain pin)"]
    PIO0_5,
    #[doc = "Pin function SDA (open-drain pin)"]
    SDA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::PIO0_5 => 0,
            FUNCR::SDA => 1,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::PIO0_5,
            1 => FUNCR::SDA,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_5`"]
    #[inline]
    pub fn is_pio0_5(&self) -> bool {
        *self == FUNCR::PIO0_5
    }
    #[doc = "Checks if the value of the field is `SDA`"]
    #[inline]
    pub fn is_sda(&self) -> bool {
        *self == FUNCR::SDA
    }
}
#[doc = "Possible values of the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODER {
    #[doc = "Standard mode/ Fast-mode I2C"]
    STANDARD_MODE_FAST,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO_FUNCTION,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl I2CMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CMODER::STANDARD_MODE_FAST => 0,
            I2CMODER::STANDARD_IO_FUNCTION => 1,
            I2CMODER::FAST_MODE_PLUS_I2C => 2,
            I2CMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CMODER {
        match value {
            0 => I2CMODER::STANDARD_MODE_FAST,
            1 => I2CMODER::STANDARD_IO_FUNCTION,
            2 => I2CMODER::FAST_MODE_PLUS_I2C,
            i => I2CMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE_FAST`"]
    #[inline]
    pub fn is_standard_mode_fast(&self) -> bool {
        *self == I2CMODER::STANDARD_MODE_FAST
    }
    #[doc = "Checks if the value of the field is `STANDARD_IO_FUNCTION`"]
    #[inline]
    pub fn is_standard_io_function(&self) -> bool {
        *self == I2CMODER::STANDARD_IO_FUNCTION
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_I2C`"]
    #[inline]
    pub fn is_fast_mode_plus_i2c(&self) -> bool {
        *self == I2CMODER::FAST_MODE_PLUS_I2C
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "Pin function PIO0_5 (open-drain pin)"]
    PIO0_5,
    #[doc = "Pin function SDA (open-drain pin)"]
    SDA,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::PIO0_5 => 0,
            FUNCW::SDA => 1,
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
    #[doc = "Pin function PIO0_5 (open-drain pin)"]
    #[inline]
    pub fn pio0_5(self) -> &'a mut W {
        self.variant(FUNCW::PIO0_5)
    }
    #[doc = "Pin function SDA (open-drain pin)"]
    #[inline]
    pub fn sda(self) -> &'a mut W {
        self.variant(FUNCW::SDA)
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
#[doc = "Values that can be written to the field `I2CMODE`"]
pub enum I2CMODEW {
    #[doc = "Standard mode/ Fast-mode I2C"]
    STANDARD_MODE_FAST,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO_FUNCTION,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
}
impl I2CMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2CMODEW::STANDARD_MODE_FAST => 0,
            I2CMODEW::STANDARD_IO_FUNCTION => 1,
            I2CMODEW::FAST_MODE_PLUS_I2C => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2CMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard mode/ Fast-mode I2C"]
    #[inline]
    pub fn standard_mode_fast(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_MODE_FAST)
    }
    #[doc = "Standard I/O functionality"]
    #[inline]
    pub fn standard_io_function(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_IO_FUNCTION)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline]
    pub fn fast_mode_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODEW::FAST_MODE_PLUS_I2C)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Configure pin function."]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline]
    pub fn i2cmode(&self) -> I2CMODER {
        I2CMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Configure pin function."]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline]
    pub fn i2cmode(&mut self) -> _I2CMODEW {
        _I2CMODEW { w: self }
    }
}
