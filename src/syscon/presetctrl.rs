#[doc = "Reader of register PRESETCTRL"]
pub type R = crate::R<u32, super::PRESETCTRL>;
#[doc = "Writer for register PRESETCTRL"]
pub type W = crate::W<u32, super::PRESETCTRL>;
#[doc = "Register PRESETCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_N_A {
    #[doc = "0: Resets the SPI0 peripheral"]
    SPIO0RESET,
    #[doc = "1: SPI0 reset de-asserted"]
    SPIO0NORESET,
}
impl From<SSP0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP0_RST_N_A) -> Self {
        match variant {
            SSP0_RST_N_A::SPIO0RESET => false,
            SSP0_RST_N_A::SPIO0NORESET => true,
        }
    }
}
#[doc = "Reader of field `SSP0_RST_N`"]
pub type SSP0_RST_N_R = crate::R<bool, SSP0_RST_N_A>;
impl SSP0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP0_RST_N_A {
        match self.bits {
            false => SSP0_RST_N_A::SPIO0RESET,
            true => SSP0_RST_N_A::SPIO0NORESET,
        }
    }
    #[doc = "Checks if the value of the field is `SPIO0RESET`"]
    #[inline(always)]
    pub fn is_spio0reset(&self) -> bool {
        *self == SSP0_RST_N_A::SPIO0RESET
    }
    #[doc = "Checks if the value of the field is `SPIO0NORESET`"]
    #[inline(always)]
    pub fn is_spio0noreset(&self) -> bool {
        *self == SSP0_RST_N_A::SPIO0NORESET
    }
}
#[doc = "Write proxy for field `SSP0_RST_N`"]
pub struct SSP0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the SPI0 peripheral"]
    #[inline(always)]
    pub fn spio0reset(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::SPIO0RESET)
    }
    #[doc = "SPI0 reset de-asserted"]
    #[inline(always)]
    pub fn spio0noreset(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::SPIO0NORESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "I2C reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_N_A {
    #[doc = "0: Resets the I2C peripheral"]
    I2CRESET,
    #[doc = "1: I2C reset de-asserted"]
    I2CNORESET,
}
impl From<I2C_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_RST_N_A) -> Self {
        match variant {
            I2C_RST_N_A::I2CRESET => false,
            I2C_RST_N_A::I2CNORESET => true,
        }
    }
}
#[doc = "Reader of field `I2C_RST_N`"]
pub type I2C_RST_N_R = crate::R<bool, I2C_RST_N_A>;
impl I2C_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_RST_N_A {
        match self.bits {
            false => I2C_RST_N_A::I2CRESET,
            true => I2C_RST_N_A::I2CNORESET,
        }
    }
    #[doc = "Checks if the value of the field is `I2CRESET`"]
    #[inline(always)]
    pub fn is_i2creset(&self) -> bool {
        *self == I2C_RST_N_A::I2CRESET
    }
    #[doc = "Checks if the value of the field is `I2CNORESET`"]
    #[inline(always)]
    pub fn is_i2cnoreset(&self) -> bool {
        *self == I2C_RST_N_A::I2CNORESET
    }
}
#[doc = "Write proxy for field `I2C_RST_N`"]
pub struct I2C_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the I2C peripheral"]
    #[inline(always)]
    pub fn i2creset(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::I2CRESET)
    }
    #[doc = "I2C reset de-asserted"]
    #[inline(always)]
    pub fn i2cnoreset(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::I2CNORESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "SPI1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_N_A {
    #[doc = "0: Resets the SPI1 peripheral"]
    SPI1RESET,
    #[doc = "1: SPI1 reset de-asserted"]
    SPI2NORESET,
}
impl From<SSP1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_RST_N_A) -> Self {
        match variant {
            SSP1_RST_N_A::SPI1RESET => false,
            SSP1_RST_N_A::SPI2NORESET => true,
        }
    }
}
#[doc = "Reader of field `SSP1_RST_N`"]
pub type SSP1_RST_N_R = crate::R<bool, SSP1_RST_N_A>;
impl SSP1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_RST_N_A {
        match self.bits {
            false => SSP1_RST_N_A::SPI1RESET,
            true => SSP1_RST_N_A::SPI2NORESET,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1RESET`"]
    #[inline(always)]
    pub fn is_spi1reset(&self) -> bool {
        *self == SSP1_RST_N_A::SPI1RESET
    }
    #[doc = "Checks if the value of the field is `SPI2NORESET`"]
    #[inline(always)]
    pub fn is_spi2noreset(&self) -> bool {
        *self == SSP1_RST_N_A::SPI2NORESET
    }
}
#[doc = "Write proxy for field `SSP1_RST_N`"]
pub struct SSP1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the SPI1 peripheral"]
    #[inline(always)]
    pub fn spi1reset(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::SPI1RESET)
    }
    #[doc = "SPI1 reset de-asserted"]
    #[inline(always)]
    pub fn spi2noreset(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::SPI2NORESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "C_CAN reset control. See Section 3.1 for part specific details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_RST_N_A {
    #[doc = "0: Resets the C_CAN peripheral"]
    CANRESET,
    #[doc = "1: C_CAN reset de-asserted"]
    CANNORESET,
}
impl From<CAN_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: CAN_RST_N_A) -> Self {
        match variant {
            CAN_RST_N_A::CANRESET => false,
            CAN_RST_N_A::CANNORESET => true,
        }
    }
}
#[doc = "Reader of field `CAN_RST_N`"]
pub type CAN_RST_N_R = crate::R<bool, CAN_RST_N_A>;
impl CAN_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN_RST_N_A {
        match self.bits {
            false => CAN_RST_N_A::CANRESET,
            true => CAN_RST_N_A::CANNORESET,
        }
    }
    #[doc = "Checks if the value of the field is `CANRESET`"]
    #[inline(always)]
    pub fn is_canreset(&self) -> bool {
        *self == CAN_RST_N_A::CANRESET
    }
    #[doc = "Checks if the value of the field is `CANNORESET`"]
    #[inline(always)]
    pub fn is_cannoreset(&self) -> bool {
        *self == CAN_RST_N_A::CANNORESET
    }
}
#[doc = "Write proxy for field `CAN_RST_N`"]
pub struct CAN_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the C_CAN peripheral"]
    #[inline(always)]
    pub fn canreset(self) -> &'a mut W {
        self.variant(CAN_RST_N_A::CANRESET)
    }
    #[doc = "C_CAN reset de-asserted"]
    #[inline(always)]
    pub fn cannoreset(self) -> &'a mut W {
        self.variant(CAN_RST_N_A::CANNORESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline(always)]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_N_R {
        SSP0_RST_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C reset control."]
    #[inline(always)]
    pub fn i2c_rst_n(&self) -> I2C_RST_N_R {
        I2C_RST_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPI1 reset control."]
    #[inline(always)]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_N_R {
        SSP1_RST_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - C_CAN reset control. See Section 3.1 for part specific details."]
    #[inline(always)]
    pub fn can_rst_n(&self) -> CAN_RST_N_R {
        CAN_RST_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline(always)]
    pub fn ssp0_rst_n(&mut self) -> SSP0_RST_N_W {
        SSP0_RST_N_W { w: self }
    }
    #[doc = "Bit 1 - I2C reset control."]
    #[inline(always)]
    pub fn i2c_rst_n(&mut self) -> I2C_RST_N_W {
        I2C_RST_N_W { w: self }
    }
    #[doc = "Bit 2 - SPI1 reset control."]
    #[inline(always)]
    pub fn ssp1_rst_n(&mut self) -> SSP1_RST_N_W {
        SSP1_RST_N_W { w: self }
    }
    #[doc = "Bit 3 - C_CAN reset control. See Section 3.1 for part specific details."]
    #[inline(always)]
    pub fn can_rst_n(&mut self) -> CAN_RST_N_W {
        CAN_RST_N_W { w: self }
    }
}
