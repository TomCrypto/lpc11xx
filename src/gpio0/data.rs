#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA0_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA0_A> for bool {
    #[inline(always)]
    fn from(variant: DATA0_A) -> Self {
        match variant {
            DATA0_A::LOW => false,
            DATA0_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<bool, DATA0_A>;
impl DATA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA0_A {
        match self.bits {
            false => DATA0_A::LOW,
            true => DATA0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA0_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA0`"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA0_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA0_A::HIGH)
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
pub enum DATA1_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA1_A> for bool {
    #[inline(always)]
    fn from(variant: DATA1_A) -> Self {
        match variant {
            DATA1_A::LOW => false,
            DATA1_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<bool, DATA1_A>;
impl DATA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA1_A {
        match self.bits {
            false => DATA1_A::LOW,
            true => DATA1_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA1_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA1_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA1`"]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA1_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA1_A::HIGH)
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
pub enum DATA2_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA2_A> for bool {
    #[inline(always)]
    fn from(variant: DATA2_A) -> Self {
        match variant {
            DATA2_A::LOW => false,
            DATA2_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA2`"]
pub type DATA2_R = crate::R<bool, DATA2_A>;
impl DATA2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA2_A {
        match self.bits {
            false => DATA2_A::LOW,
            true => DATA2_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA2_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA2_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA2`"]
pub struct DATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA2_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA2_A::HIGH)
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
pub enum DATA3_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA3_A> for bool {
    #[inline(always)]
    fn from(variant: DATA3_A) -> Self {
        match variant {
            DATA3_A::LOW => false,
            DATA3_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA3`"]
pub type DATA3_R = crate::R<bool, DATA3_A>;
impl DATA3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA3_A {
        match self.bits {
            false => DATA3_A::LOW,
            true => DATA3_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA3_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA3_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA3`"]
pub struct DATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA3_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA3_A::HIGH)
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
pub enum DATA4_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA4_A> for bool {
    #[inline(always)]
    fn from(variant: DATA4_A) -> Self {
        match variant {
            DATA4_A::LOW => false,
            DATA4_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA4`"]
pub type DATA4_R = crate::R<bool, DATA4_A>;
impl DATA4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA4_A {
        match self.bits {
            false => DATA4_A::LOW,
            true => DATA4_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA4_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA4_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA4`"]
pub struct DATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA4_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA4_A::HIGH)
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
pub enum DATA5_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA5_A> for bool {
    #[inline(always)]
    fn from(variant: DATA5_A) -> Self {
        match variant {
            DATA5_A::LOW => false,
            DATA5_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA5`"]
pub type DATA5_R = crate::R<bool, DATA5_A>;
impl DATA5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA5_A {
        match self.bits {
            false => DATA5_A::LOW,
            true => DATA5_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA5_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA5_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA5`"]
pub struct DATA5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA5_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA5_A::HIGH)
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
pub enum DATA6_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA6_A> for bool {
    #[inline(always)]
    fn from(variant: DATA6_A) -> Self {
        match variant {
            DATA6_A::LOW => false,
            DATA6_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA6`"]
pub type DATA6_R = crate::R<bool, DATA6_A>;
impl DATA6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA6_A {
        match self.bits {
            false => DATA6_A::LOW,
            true => DATA6_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA6_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA6_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA6`"]
pub struct DATA6_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA6_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA6_A::HIGH)
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
pub enum DATA7_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA7_A> for bool {
    #[inline(always)]
    fn from(variant: DATA7_A) -> Self {
        match variant {
            DATA7_A::LOW => false,
            DATA7_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA7`"]
pub type DATA7_R = crate::R<bool, DATA7_A>;
impl DATA7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA7_A {
        match self.bits {
            false => DATA7_A::LOW,
            true => DATA7_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA7_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA7_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA7`"]
pub struct DATA7_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA7_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA7_A::HIGH)
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
pub enum DATA8_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA8_A> for bool {
    #[inline(always)]
    fn from(variant: DATA8_A) -> Self {
        match variant {
            DATA8_A::LOW => false,
            DATA8_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA8`"]
pub type DATA8_R = crate::R<bool, DATA8_A>;
impl DATA8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA8_A {
        match self.bits {
            false => DATA8_A::LOW,
            true => DATA8_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA8_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA8_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA8`"]
pub struct DATA8_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA8_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA8_A::HIGH)
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
pub enum DATA9_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA9_A> for bool {
    #[inline(always)]
    fn from(variant: DATA9_A) -> Self {
        match variant {
            DATA9_A::LOW => false,
            DATA9_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA9`"]
pub type DATA9_R = crate::R<bool, DATA9_A>;
impl DATA9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA9_A {
        match self.bits {
            false => DATA9_A::LOW,
            true => DATA9_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA9_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA9_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA9`"]
pub struct DATA9_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA9_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA9_A::HIGH)
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
pub enum DATA10_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA10_A> for bool {
    #[inline(always)]
    fn from(variant: DATA10_A) -> Self {
        match variant {
            DATA10_A::LOW => false,
            DATA10_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA10`"]
pub type DATA10_R = crate::R<bool, DATA10_A>;
impl DATA10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA10_A {
        match self.bits {
            false => DATA10_A::LOW,
            true => DATA10_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA10_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA10_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA10`"]
pub struct DATA10_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA10_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA10_A::HIGH)
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
pub enum DATA11_A {
    #[doc = "0: Pin is driven low"]
    LOW,
    #[doc = "1: Pin is driven high"]
    HIGH,
}
impl From<DATA11_A> for bool {
    #[inline(always)]
    fn from(variant: DATA11_A) -> Self {
        match variant {
            DATA11_A::LOW => false,
            DATA11_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DATA11`"]
pub type DATA11_R = crate::R<bool, DATA11_A>;
impl DATA11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA11_A {
        match self.bits {
            false => DATA11_A::LOW,
            true => DATA11_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DATA11_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DATA11_A::HIGH
    }
}
#[doc = "Write proxy for field `DATA11`"]
pub struct DATA11_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DATA11_A::LOW)
    }
    #[doc = "Pin is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DATA11_A::HIGH)
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
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn data10(&self) -> DATA10_R {
        DATA10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn data11(&self) -> DATA11_R {
        DATA11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIOn_0."]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W {
        DATA2_W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W {
        DATA8_W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn data9(&mut self) -> DATA9_W {
        DATA9_W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn data10(&mut self) -> DATA10_W {
        DATA10_W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn data11(&mut self) -> DATA11_W {
        DATA11_W { w: self }
    }
}
