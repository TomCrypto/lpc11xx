#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL {
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
#[doc = "Possible values of the field `SSP0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_NR {
    #[doc = "Resets the SPI0 peripheral"]
    SPIO0RESET,
    #[doc = "SPI0 reset de-asserted"]
    SPIO0NORESET,
}
impl SSP0_RST_NR {
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
            SSP0_RST_NR::SPIO0RESET => false,
            SSP0_RST_NR::SPIO0NORESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP0_RST_NR {
        match value {
            false => SSP0_RST_NR::SPIO0RESET,
            true => SSP0_RST_NR::SPIO0NORESET,
        }
    }
    #[doc = "Checks if the value of the field is `SPIO0RESET`"]
    #[inline]
    pub fn is_spio0reset(&self) -> bool {
        *self == SSP0_RST_NR::SPIO0RESET
    }
    #[doc = "Checks if the value of the field is `SPIO0NORESET`"]
    #[inline]
    pub fn is_spio0noreset(&self) -> bool {
        *self == SSP0_RST_NR::SPIO0NORESET
    }
}
#[doc = "Possible values of the field `I2C_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_NR {
    #[doc = "Resets the I2C peripheral"]
    I2CRESET,
    #[doc = "I2C reset de-asserted"]
    I2CNORESET,
}
impl I2C_RST_NR {
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
            I2C_RST_NR::I2CRESET => false,
            I2C_RST_NR::I2CNORESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C_RST_NR {
        match value {
            false => I2C_RST_NR::I2CRESET,
            true => I2C_RST_NR::I2CNORESET,
        }
    }
    #[doc = "Checks if the value of the field is `I2CRESET`"]
    #[inline]
    pub fn is_i2creset(&self) -> bool {
        *self == I2C_RST_NR::I2CRESET
    }
    #[doc = "Checks if the value of the field is `I2CNORESET`"]
    #[inline]
    pub fn is_i2cnoreset(&self) -> bool {
        *self == I2C_RST_NR::I2CNORESET
    }
}
#[doc = "Possible values of the field `SSP1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_NR {
    #[doc = "Resets the SPI1 peripheral"]
    SPI1RESET,
    #[doc = "SPI1 reset de-asserted"]
    SPI2NORESET,
}
impl SSP1_RST_NR {
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
            SSP1_RST_NR::SPI1RESET => false,
            SSP1_RST_NR::SPI2NORESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP1_RST_NR {
        match value {
            false => SSP1_RST_NR::SPI1RESET,
            true => SSP1_RST_NR::SPI2NORESET,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1RESET`"]
    #[inline]
    pub fn is_spi1reset(&self) -> bool {
        *self == SSP1_RST_NR::SPI1RESET
    }
    #[doc = "Checks if the value of the field is `SPI2NORESET`"]
    #[inline]
    pub fn is_spi2noreset(&self) -> bool {
        *self == SSP1_RST_NR::SPI2NORESET
    }
}
#[doc = "Possible values of the field `CAN_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_RST_NR {
    #[doc = "Resets the C_CAN peripheral"]
    CANRESET,
    #[doc = "C_CAN reset de-asserted"]
    CANNORESET,
}
impl CAN_RST_NR {
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
            CAN_RST_NR::CANRESET => false,
            CAN_RST_NR::CANNORESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAN_RST_NR {
        match value {
            false => CAN_RST_NR::CANRESET,
            true => CAN_RST_NR::CANNORESET,
        }
    }
    #[doc = "Checks if the value of the field is `CANRESET`"]
    #[inline]
    pub fn is_canreset(&self) -> bool {
        *self == CAN_RST_NR::CANRESET
    }
    #[doc = "Checks if the value of the field is `CANNORESET`"]
    #[inline]
    pub fn is_cannoreset(&self) -> bool {
        *self == CAN_RST_NR::CANNORESET
    }
}
#[doc = "Values that can be written to the field `SSP0_RST_N`"]
pub enum SSP0_RST_NW {
    #[doc = "Resets the SPI0 peripheral"]
    SPIO0RESET,
    #[doc = "SPI0 reset de-asserted"]
    SPIO0NORESET,
}
impl SSP0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP0_RST_NW::SPIO0RESET => false,
            SSP0_RST_NW::SPIO0NORESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the SPI0 peripheral"]
    #[inline]
    pub fn spio0reset(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::SPIO0RESET)
    }
    #[doc = "SPI0 reset de-asserted"]
    #[inline]
    pub fn spio0noreset(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::SPIO0NORESET)
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
#[doc = "Values that can be written to the field `I2C_RST_N`"]
pub enum I2C_RST_NW {
    #[doc = "Resets the I2C peripheral"]
    I2CRESET,
    #[doc = "I2C reset de-asserted"]
    I2CNORESET,
}
impl I2C_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C_RST_NW::I2CRESET => false,
            I2C_RST_NW::I2CNORESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the I2C peripheral"]
    #[inline]
    pub fn i2creset(self) -> &'a mut W {
        self.variant(I2C_RST_NW::I2CRESET)
    }
    #[doc = "I2C reset de-asserted"]
    #[inline]
    pub fn i2cnoreset(self) -> &'a mut W {
        self.variant(I2C_RST_NW::I2CNORESET)
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
#[doc = "Values that can be written to the field `SSP1_RST_N`"]
pub enum SSP1_RST_NW {
    #[doc = "Resets the SPI1 peripheral"]
    SPI1RESET,
    #[doc = "SPI1 reset de-asserted"]
    SPI2NORESET,
}
impl SSP1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP1_RST_NW::SPI1RESET => false,
            SSP1_RST_NW::SPI2NORESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the SPI1 peripheral"]
    #[inline]
    pub fn spi1reset(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::SPI1RESET)
    }
    #[doc = "SPI1 reset de-asserted"]
    #[inline]
    pub fn spi2noreset(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::SPI2NORESET)
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
#[doc = "Values that can be written to the field `CAN_RST_N`"]
pub enum CAN_RST_NW {
    #[doc = "Resets the C_CAN peripheral"]
    CANRESET,
    #[doc = "C_CAN reset de-asserted"]
    CANNORESET,
}
impl CAN_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAN_RST_NW::CANRESET => false,
            CAN_RST_NW::CANNORESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAN_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the C_CAN peripheral"]
    #[inline]
    pub fn canreset(self) -> &'a mut W {
        self.variant(CAN_RST_NW::CANRESET)
    }
    #[doc = "C_CAN reset de-asserted"]
    #[inline]
    pub fn cannoreset(self) -> &'a mut W {
        self.variant(CAN_RST_NW::CANNORESET)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_NR {
        SSP0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - I2C reset control."]
    #[inline]
    pub fn i2c_rst_n(&self) -> I2C_RST_NR {
        I2C_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SPI1 reset control."]
    #[inline]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_NR {
        SSP1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - C_CAN reset control. See Section 3.1 for part specific details."]
    #[inline]
    pub fn can_rst_n(&self) -> CAN_RST_NR {
        CAN_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline]
    pub fn ssp0_rst_n(&mut self) -> _SSP0_RST_NW {
        _SSP0_RST_NW { w: self }
    }
    #[doc = "Bit 1 - I2C reset control."]
    #[inline]
    pub fn i2c_rst_n(&mut self) -> _I2C_RST_NW {
        _I2C_RST_NW { w: self }
    }
    #[doc = "Bit 2 - SPI1 reset control."]
    #[inline]
    pub fn ssp1_rst_n(&mut self) -> _SSP1_RST_NW {
        _SSP1_RST_NW { w: self }
    }
    #[doc = "Bit 3 - C_CAN reset control. See Section 3.1 for part specific details."]
    #[inline]
    pub fn can_rst_n(&mut self) -> _CAN_RST_NW {
        _CAN_RST_NW { w: self }
    }
}
