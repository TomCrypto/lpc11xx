#[doc = "Reader of register IBE"]
pub type R = crate::R<u32, super::IBE>;
#[doc = "Writer for register IBE"]
pub type W = crate::W<u32, super::IBE>;
#[doc = "Register IBE `reset()`'s with value 0"]
impl crate::ResetValue for super::IBE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE0_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE0_A> for bool {
    #[inline(always)]
    fn from(variant: IBE0_A) -> Self {
        match variant {
            IBE0_A::IEV => false,
            IBE0_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE0`"]
pub type IBE0_R = crate::R<bool, IBE0_A>;
impl IBE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE0_A {
        match self.bits {
            false => IBE0_A::IEV,
            true => IBE0_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE0_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE0_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE0`"]
pub struct IBE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE0_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE0_A::BOTH)
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
#[doc = "PIOn_1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE1_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE1_A> for bool {
    #[inline(always)]
    fn from(variant: IBE1_A) -> Self {
        match variant {
            IBE1_A::IEV => false,
            IBE1_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE1`"]
pub type IBE1_R = crate::R<bool, IBE1_A>;
impl IBE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE1_A {
        match self.bits {
            false => IBE1_A::IEV,
            true => IBE1_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE1_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE1_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE1`"]
pub struct IBE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE1_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE1_A::BOTH)
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
#[doc = "PIOn_2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE2_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE2_A> for bool {
    #[inline(always)]
    fn from(variant: IBE2_A) -> Self {
        match variant {
            IBE2_A::IEV => false,
            IBE2_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE2`"]
pub type IBE2_R = crate::R<bool, IBE2_A>;
impl IBE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE2_A {
        match self.bits {
            false => IBE2_A::IEV,
            true => IBE2_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE2_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE2_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE2`"]
pub struct IBE2_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE2_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE2_A::BOTH)
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
#[doc = "PIOn_3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE3_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE3_A> for bool {
    #[inline(always)]
    fn from(variant: IBE3_A) -> Self {
        match variant {
            IBE3_A::IEV => false,
            IBE3_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE3`"]
pub type IBE3_R = crate::R<bool, IBE3_A>;
impl IBE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE3_A {
        match self.bits {
            false => IBE3_A::IEV,
            true => IBE3_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE3_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE3_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE3`"]
pub struct IBE3_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE3_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE3_A::BOTH)
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
#[doc = "PIOn_4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE4_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE4_A> for bool {
    #[inline(always)]
    fn from(variant: IBE4_A) -> Self {
        match variant {
            IBE4_A::IEV => false,
            IBE4_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE4`"]
pub type IBE4_R = crate::R<bool, IBE4_A>;
impl IBE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE4_A {
        match self.bits {
            false => IBE4_A::IEV,
            true => IBE4_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE4_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE4_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE4`"]
pub struct IBE4_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE4_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE4_A::BOTH)
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
#[doc = "PIOn_5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE5_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE5_A> for bool {
    #[inline(always)]
    fn from(variant: IBE5_A) -> Self {
        match variant {
            IBE5_A::IEV => false,
            IBE5_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE5`"]
pub type IBE5_R = crate::R<bool, IBE5_A>;
impl IBE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE5_A {
        match self.bits {
            false => IBE5_A::IEV,
            true => IBE5_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE5_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE5_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE5`"]
pub struct IBE5_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE5_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE5_A::BOTH)
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
#[doc = "PIOn_6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE6_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE6_A> for bool {
    #[inline(always)]
    fn from(variant: IBE6_A) -> Self {
        match variant {
            IBE6_A::IEV => false,
            IBE6_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE6`"]
pub type IBE6_R = crate::R<bool, IBE6_A>;
impl IBE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE6_A {
        match self.bits {
            false => IBE6_A::IEV,
            true => IBE6_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE6_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE6_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE6`"]
pub struct IBE6_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE6_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE6_A::BOTH)
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
#[doc = "PIOn_7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE7_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE7_A> for bool {
    #[inline(always)]
    fn from(variant: IBE7_A) -> Self {
        match variant {
            IBE7_A::IEV => false,
            IBE7_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE7`"]
pub type IBE7_R = crate::R<bool, IBE7_A>;
impl IBE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE7_A {
        match self.bits {
            false => IBE7_A::IEV,
            true => IBE7_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE7_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE7_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE7`"]
pub struct IBE7_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE7_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE7_A::BOTH)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "PIOn_8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE8_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE8_A> for bool {
    #[inline(always)]
    fn from(variant: IBE8_A) -> Self {
        match variant {
            IBE8_A::IEV => false,
            IBE8_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE8`"]
pub type IBE8_R = crate::R<bool, IBE8_A>;
impl IBE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE8_A {
        match self.bits {
            false => IBE8_A::IEV,
            true => IBE8_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE8_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE8_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE8`"]
pub struct IBE8_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE8_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE8_A::BOTH)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "PIOn_9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE9_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE9_A> for bool {
    #[inline(always)]
    fn from(variant: IBE9_A) -> Self {
        match variant {
            IBE9_A::IEV => false,
            IBE9_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE9`"]
pub type IBE9_R = crate::R<bool, IBE9_A>;
impl IBE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE9_A {
        match self.bits {
            false => IBE9_A::IEV,
            true => IBE9_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE9_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE9_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE9`"]
pub struct IBE9_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE9_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE9_A::BOTH)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "PIOn_10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE10_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE10_A> for bool {
    #[inline(always)]
    fn from(variant: IBE10_A) -> Self {
        match variant {
            IBE10_A::IEV => false,
            IBE10_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE10`"]
pub type IBE10_R = crate::R<bool, IBE10_A>;
impl IBE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE10_A {
        match self.bits {
            false => IBE10_A::IEV,
            true => IBE10_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE10_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE10_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE10`"]
pub struct IBE10_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE10_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE10_A::BOTH)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "PIOn_11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBE11_A {
    #[doc = "0: Pin interrupt edge is configured through the IEV register"]
    IEV,
    #[doc = "1: Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    BOTH,
}
impl From<IBE11_A> for bool {
    #[inline(always)]
    fn from(variant: IBE11_A) -> Self {
        match variant {
            IBE11_A::IEV => false,
            IBE11_A::BOTH => true,
        }
    }
}
#[doc = "Reader of field `IBE11`"]
pub type IBE11_R = crate::R<bool, IBE11_A>;
impl IBE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBE11_A {
        match self.bits {
            false => IBE11_A::IEV,
            true => IBE11_A::BOTH,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBE11_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == IBE11_A::BOTH
    }
}
#[doc = "Write proxy for field `IBE11`"]
pub struct IBE11_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBE11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt edge is configured through the IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBE11_A::IEV)
    }
    #[doc = "Pin interrupt is triggered on both rising and falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(IBE11_A::BOTH)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PIOn_0."]
    #[inline(always)]
    pub fn ibe0(&self) -> IBE0_R {
        IBE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn ibe1(&self) -> IBE1_R {
        IBE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn ibe2(&self) -> IBE2_R {
        IBE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn ibe3(&self) -> IBE3_R {
        IBE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn ibe4(&self) -> IBE4_R {
        IBE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn ibe5(&self) -> IBE5_R {
        IBE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn ibe6(&self) -> IBE6_R {
        IBE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn ibe7(&self) -> IBE7_R {
        IBE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn ibe8(&self) -> IBE8_R {
        IBE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn ibe9(&self) -> IBE9_R {
        IBE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn ibe10(&self) -> IBE10_R {
        IBE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn ibe11(&self) -> IBE11_R {
        IBE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIOn_0."]
    #[inline(always)]
    pub fn ibe0(&mut self) -> IBE0_W {
        IBE0_W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn ibe1(&mut self) -> IBE1_W {
        IBE1_W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn ibe2(&mut self) -> IBE2_W {
        IBE2_W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn ibe3(&mut self) -> IBE3_W {
        IBE3_W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn ibe4(&mut self) -> IBE4_W {
        IBE4_W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn ibe5(&mut self) -> IBE5_W {
        IBE5_W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn ibe6(&mut self) -> IBE6_W {
        IBE6_W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn ibe7(&mut self) -> IBE7_W {
        IBE7_W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn ibe8(&mut self) -> IBE8_W {
        IBE8_W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn ibe9(&mut self) -> IBE9_W {
        IBE9_W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn ibe10(&mut self) -> IBE10_W {
        IBE10_W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn ibe11(&mut self) -> IBE11_W {
        IBE11_W { w: self }
    }
}
