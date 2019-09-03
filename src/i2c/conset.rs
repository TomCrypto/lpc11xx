#[doc = "Reader of register CONSET"]
pub type R = crate::R<u32, super::CONSET>;
#[doc = "Writer for register CONSET"]
pub type W = crate::W<u32, super::CONSET>;
#[doc = "Register CONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Assert Acknowledge flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AA_A {
    #[doc = "1: Set the flag"]
    SET,
}
impl From<AA_A> for bool {
    #[inline(always)]
    fn from(variant: AA_A) -> Self {
        match variant {
            AA_A::SET => true,
        }
    }
}
#[doc = "Reader of field `AA`"]
pub type AA_R = crate::R<bool, AA_A>;
impl AA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, AA_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(AA_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AA_A::SET
    }
}
#[doc = "Write proxy for field `AA`"]
pub struct AA_W<'a> {
    w: &'a mut W,
}
impl<'a> AA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(AA_A::SET)
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
pub enum SI_A {
    #[doc = "1: Set the flag"]
    SET,
}
impl From<SI_A> for bool {
    #[inline(always)]
    fn from(variant: SI_A) -> Self {
        match variant {
            SI_A::SET => true,
        }
    }
}
#[doc = "Reader of field `SI`"]
pub type SI_R = crate::R<bool, SI_A>;
impl SI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SI_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SI_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SI_A::SET
    }
}
#[doc = "Write proxy for field `SI`"]
pub struct SI_W<'a> {
    w: &'a mut W,
}
impl<'a> SI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SI_A::SET)
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
#[doc = "STOP flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STO_A {
    #[doc = "1: Set the flag"]
    SET,
}
impl From<STO_A> for bool {
    #[inline(always)]
    fn from(variant: STO_A) -> Self {
        match variant {
            STO_A::SET => true,
        }
    }
}
#[doc = "Reader of field `STO`"]
pub type STO_R = crate::R<bool, STO_A>;
impl STO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, STO_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(STO_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == STO_A::SET
    }
}
#[doc = "Write proxy for field `STO`"]
pub struct STO_W<'a> {
    w: &'a mut W,
}
impl<'a> STO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STO_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "START flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STA_A {
    #[doc = "1: Set the flag"]
    SET,
}
impl From<STA_A> for bool {
    #[inline(always)]
    fn from(variant: STA_A) -> Self {
        match variant {
            STA_A::SET => true,
        }
    }
}
#[doc = "Reader of field `STA`"]
pub type STA_R = crate::R<bool, STA_A>;
impl STA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, STA_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(STA_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == STA_A::SET
    }
}
#[doc = "Write proxy for field `STA`"]
pub struct STA_W<'a> {
    w: &'a mut W,
}
impl<'a> STA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the flag"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STA_A::SET)
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
#[doc = "I2C Interface Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2EN_A {
    #[doc = "1: Enable the I2C interface"]
    ENABLE,
}
impl From<I2EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2EN_A) -> Self {
        match variant {
            I2EN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `I2EN`"]
pub type I2EN_R = crate::R<bool, I2EN_A>;
impl I2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, I2EN_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(I2EN_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `I2EN`"]
pub struct I2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the I2C interface"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2EN_A::ENABLE)
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
impl R {
    #[doc = "Bit 2 - Assert Acknowledge flag."]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag."]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Interface Enable bit."]
    #[inline(always)]
    pub fn i2en(&self) -> I2EN_R {
        I2EN_R::new(((self.bits >> 6) & 0x01) != 0)
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
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W {
        STO_W { w: self }
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W {
        STA_W { w: self }
    }
    #[doc = "Bit 6 - I2C Interface Enable bit."]
    #[inline(always)]
    pub fn i2en(&mut self) -> I2EN_W {
        I2EN_W { w: self }
    }
}
