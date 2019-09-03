#[doc = "Reader of register DIR"]
pub type R = crate::R<u32, super::DIR>;
#[doc = "Writer for register DIR"]
pub type W = crate::W<u32, super::DIR>;
#[doc = "Register DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR0_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR0_A> for bool {
    #[inline(always)]
    fn from(variant: DIR0_A) -> Self {
        match variant {
            DIR0_A::INPUT => false,
            DIR0_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR0`"]
pub type DIR0_R = crate::R<bool, DIR0_A>;
impl DIR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR0_A {
        match self.bits {
            false => DIR0_A::INPUT,
            true => DIR0_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR0_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR0`"]
pub struct DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR0_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR0_A::OUTPUT)
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
pub enum DIR1_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR1_A> for bool {
    #[inline(always)]
    fn from(variant: DIR1_A) -> Self {
        match variant {
            DIR1_A::INPUT => false,
            DIR1_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR1`"]
pub type DIR1_R = crate::R<bool, DIR1_A>;
impl DIR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR1_A {
        match self.bits {
            false => DIR1_A::INPUT,
            true => DIR1_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR1_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR1_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR1`"]
pub struct DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR1_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR1_A::OUTPUT)
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
pub enum DIR2_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR2_A> for bool {
    #[inline(always)]
    fn from(variant: DIR2_A) -> Self {
        match variant {
            DIR2_A::INPUT => false,
            DIR2_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR2`"]
pub type DIR2_R = crate::R<bool, DIR2_A>;
impl DIR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR2_A {
        match self.bits {
            false => DIR2_A::INPUT,
            true => DIR2_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR2_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR2_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR2`"]
pub struct DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR2_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR2_A::OUTPUT)
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
pub enum DIR3_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR3_A> for bool {
    #[inline(always)]
    fn from(variant: DIR3_A) -> Self {
        match variant {
            DIR3_A::INPUT => false,
            DIR3_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR3`"]
pub type DIR3_R = crate::R<bool, DIR3_A>;
impl DIR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR3_A {
        match self.bits {
            false => DIR3_A::INPUT,
            true => DIR3_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR3_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR3_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR3`"]
pub struct DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR3_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR3_A::OUTPUT)
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
pub enum DIR4_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR4_A> for bool {
    #[inline(always)]
    fn from(variant: DIR4_A) -> Self {
        match variant {
            DIR4_A::INPUT => false,
            DIR4_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR4`"]
pub type DIR4_R = crate::R<bool, DIR4_A>;
impl DIR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR4_A {
        match self.bits {
            false => DIR4_A::INPUT,
            true => DIR4_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR4_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR4_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR4`"]
pub struct DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR4_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR4_A::OUTPUT)
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
pub enum DIR5_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR5_A> for bool {
    #[inline(always)]
    fn from(variant: DIR5_A) -> Self {
        match variant {
            DIR5_A::INPUT => false,
            DIR5_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR5`"]
pub type DIR5_R = crate::R<bool, DIR5_A>;
impl DIR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR5_A {
        match self.bits {
            false => DIR5_A::INPUT,
            true => DIR5_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR5_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR5_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR5`"]
pub struct DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR5_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR5_A::OUTPUT)
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
pub enum DIR6_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR6_A> for bool {
    #[inline(always)]
    fn from(variant: DIR6_A) -> Self {
        match variant {
            DIR6_A::INPUT => false,
            DIR6_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR6`"]
pub type DIR6_R = crate::R<bool, DIR6_A>;
impl DIR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR6_A {
        match self.bits {
            false => DIR6_A::INPUT,
            true => DIR6_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR6_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR6_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR6`"]
pub struct DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR6_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR6_A::OUTPUT)
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
pub enum DIR7_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR7_A> for bool {
    #[inline(always)]
    fn from(variant: DIR7_A) -> Self {
        match variant {
            DIR7_A::INPUT => false,
            DIR7_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR7`"]
pub type DIR7_R = crate::R<bool, DIR7_A>;
impl DIR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR7_A {
        match self.bits {
            false => DIR7_A::INPUT,
            true => DIR7_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR7_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR7_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR7`"]
pub struct DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR7_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR7_A::OUTPUT)
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
pub enum DIR8_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR8_A> for bool {
    #[inline(always)]
    fn from(variant: DIR8_A) -> Self {
        match variant {
            DIR8_A::INPUT => false,
            DIR8_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR8`"]
pub type DIR8_R = crate::R<bool, DIR8_A>;
impl DIR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR8_A {
        match self.bits {
            false => DIR8_A::INPUT,
            true => DIR8_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR8_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR8`"]
pub struct DIR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR8_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR8_A::OUTPUT)
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
pub enum DIR9_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR9_A> for bool {
    #[inline(always)]
    fn from(variant: DIR9_A) -> Self {
        match variant {
            DIR9_A::INPUT => false,
            DIR9_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR9`"]
pub type DIR9_R = crate::R<bool, DIR9_A>;
impl DIR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR9_A {
        match self.bits {
            false => DIR9_A::INPUT,
            true => DIR9_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR9_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR9_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR9`"]
pub struct DIR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR9_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR9_A::OUTPUT)
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
pub enum DIR10_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR10_A> for bool {
    #[inline(always)]
    fn from(variant: DIR10_A) -> Self {
        match variant {
            DIR10_A::INPUT => false,
            DIR10_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR10`"]
pub type DIR10_R = crate::R<bool, DIR10_A>;
impl DIR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR10_A {
        match self.bits {
            false => DIR10_A::INPUT,
            true => DIR10_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR10_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR10_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR10`"]
pub struct DIR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR10_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR10_A::OUTPUT)
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
pub enum DIR11_A {
    #[doc = "0: Pin is configured as an input"]
    INPUT,
    #[doc = "1: Pin is configured as an output"]
    OUTPUT,
}
impl From<DIR11_A> for bool {
    #[inline(always)]
    fn from(variant: DIR11_A) -> Self {
        match variant {
            DIR11_A::INPUT => false,
            DIR11_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `DIR11`"]
pub type DIR11_R = crate::R<bool, DIR11_A>;
impl DIR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR11_A {
        match self.bits {
            false => DIR11_A::INPUT,
            true => DIR11_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR11_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR11_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR11`"]
pub struct DIR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR11_A::INPUT)
    }
    #[doc = "Pin is configured as an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR11_A::OUTPUT)
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
    pub fn dir0(&self) -> DIR0_R {
        DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn dir1(&self) -> DIR1_R {
        DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn dir2(&self) -> DIR2_R {
        DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn dir3(&self) -> DIR3_R {
        DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn dir4(&self) -> DIR4_R {
        DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn dir5(&self) -> DIR5_R {
        DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn dir6(&self) -> DIR6_R {
        DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn dir7(&self) -> DIR7_R {
        DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn dir8(&self) -> DIR8_R {
        DIR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn dir9(&self) -> DIR9_R {
        DIR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn dir10(&self) -> DIR10_R {
        DIR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn dir11(&self) -> DIR11_R {
        DIR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIOn_0."]
    #[inline(always)]
    pub fn dir0(&mut self) -> DIR0_W {
        DIR0_W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn dir1(&mut self) -> DIR1_W {
        DIR1_W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn dir2(&mut self) -> DIR2_W {
        DIR2_W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn dir3(&mut self) -> DIR3_W {
        DIR3_W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn dir4(&mut self) -> DIR4_W {
        DIR4_W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn dir5(&mut self) -> DIR5_W {
        DIR5_W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn dir6(&mut self) -> DIR6_W {
        DIR6_W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn dir7(&mut self) -> DIR7_W {
        DIR7_W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn dir8(&mut self) -> DIR8_W {
        DIR8_W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn dir9(&mut self) -> DIR9_W {
        DIR9_W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn dir10(&mut self) -> DIR10_W {
        DIR10_W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn dir11(&mut self) -> DIR11_W {
        DIR11_W { w: self }
    }
}
