#[doc = "Writer for register CONCLR"]
pub type W = crate::W<u32, super::CONCLR>;
#[doc = "Register CONCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CONCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Assert Acknowledge flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AA_AW {
    #[doc = "1: Clear the flag"]
    CLEAR,
}
impl From<AA_AW> for bool {
    #[inline(always)]
    fn from(variant: AA_AW) -> Self {
        match variant {
            AA_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `AA`"]
pub struct AA_W<'a> {
    w: &'a mut W,
}
impl<'a> AA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AA_AW::CLEAR)
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
#[doc = "Interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SI_AW {
    #[doc = "1: Clear the flag"]
    CLEAR,
}
impl From<SI_AW> for bool {
    #[inline(always)]
    fn from(variant: SI_AW) -> Self {
        match variant {
            SI_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `SI`"]
pub struct SI_W<'a> {
    w: &'a mut W,
}
impl<'a> SI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SI_AW::CLEAR)
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
#[doc = "START flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STA_AW {
    #[doc = "1: Clear the flag"]
    CLEAR,
}
impl From<STA_AW> for bool {
    #[inline(always)]
    fn from(variant: STA_AW) -> Self {
        match variant {
            STA_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `STA`"]
pub struct STA_W<'a> {
    w: &'a mut W,
}
impl<'a> STA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STA_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "I2C Interface Disable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2EN_AW {
    #[doc = "1: Disable the I2C interface"]
    DISABLE,
}
impl From<I2EN_AW> for bool {
    #[inline(always)]
    fn from(variant: I2EN_AW) -> Self {
        match variant {
            I2EN_AW::DISABLE => true,
        }
    }
}
#[doc = "Write proxy for field `I2EN`"]
pub struct I2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2EN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the I2C interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2EN_AW::DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Assert Acknowledge flag."]
    #[inline(always)]
    pub fn aa(&mut self) -> AA_W {
        AA_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag."]
    #[inline(always)]
    pub fn si(&mut self) -> SI_W {
        SI_W { w: self }
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W {
        STA_W { w: self }
    }
    #[doc = "Bit 6 - I2C Interface Disable bit."]
    #[inline(always)]
    pub fn i2en(&mut self) -> I2EN_W {
        I2EN_W { w: self }
    }
}
