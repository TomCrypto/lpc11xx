#[doc = "Reader of register IS"]
pub type R = crate::R<u32, super::IS>;
#[doc = "Writer for register IS"]
pub type W = crate::W<u32, super::IS>;
#[doc = "Register IS `reset()`'s with value 0"]
impl crate::ResetValue for super::IS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENSE0_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE0_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE0_A) -> Self {
        match variant {
            ISENSE0_A::EDGE => false,
            ISENSE0_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE0`"]
pub type ISENSE0_R = crate::R<bool, ISENSE0_A>;
impl ISENSE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE0_A {
        match self.bits {
            false => ISENSE0_A::EDGE,
            true => ISENSE0_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE0_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE0_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE0`"]
pub struct ISENSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE0_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE0_A::LEVEL)
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
pub enum ISENSE1_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE1_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE1_A) -> Self {
        match variant {
            ISENSE1_A::EDGE => false,
            ISENSE1_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE1`"]
pub type ISENSE1_R = crate::R<bool, ISENSE1_A>;
impl ISENSE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE1_A {
        match self.bits {
            false => ISENSE1_A::EDGE,
            true => ISENSE1_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE1_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE1_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE1`"]
pub struct ISENSE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE1_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE1_A::LEVEL)
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
pub enum ISENSE2_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE2_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE2_A) -> Self {
        match variant {
            ISENSE2_A::EDGE => false,
            ISENSE2_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE2`"]
pub type ISENSE2_R = crate::R<bool, ISENSE2_A>;
impl ISENSE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE2_A {
        match self.bits {
            false => ISENSE2_A::EDGE,
            true => ISENSE2_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE2_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE2_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE2`"]
pub struct ISENSE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE2_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE2_A::LEVEL)
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
pub enum ISENSE3_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE3_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE3_A) -> Self {
        match variant {
            ISENSE3_A::EDGE => false,
            ISENSE3_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE3`"]
pub type ISENSE3_R = crate::R<bool, ISENSE3_A>;
impl ISENSE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE3_A {
        match self.bits {
            false => ISENSE3_A::EDGE,
            true => ISENSE3_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE3_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE3_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE3`"]
pub struct ISENSE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE3_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE3_A::LEVEL)
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
pub enum ISENSE4_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE4_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE4_A) -> Self {
        match variant {
            ISENSE4_A::EDGE => false,
            ISENSE4_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE4`"]
pub type ISENSE4_R = crate::R<bool, ISENSE4_A>;
impl ISENSE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE4_A {
        match self.bits {
            false => ISENSE4_A::EDGE,
            true => ISENSE4_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE4_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE4_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE4`"]
pub struct ISENSE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE4_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE4_A::LEVEL)
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
pub enum ISENSE5_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE5_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE5_A) -> Self {
        match variant {
            ISENSE5_A::EDGE => false,
            ISENSE5_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE5`"]
pub type ISENSE5_R = crate::R<bool, ISENSE5_A>;
impl ISENSE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE5_A {
        match self.bits {
            false => ISENSE5_A::EDGE,
            true => ISENSE5_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE5_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE5_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE5`"]
pub struct ISENSE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE5_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE5_A::LEVEL)
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
pub enum ISENSE6_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE6_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE6_A) -> Self {
        match variant {
            ISENSE6_A::EDGE => false,
            ISENSE6_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE6`"]
pub type ISENSE6_R = crate::R<bool, ISENSE6_A>;
impl ISENSE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE6_A {
        match self.bits {
            false => ISENSE6_A::EDGE,
            true => ISENSE6_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE6_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE6_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE6`"]
pub struct ISENSE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE6_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE6_A::LEVEL)
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
pub enum ISENSE7_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE7_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE7_A) -> Self {
        match variant {
            ISENSE7_A::EDGE => false,
            ISENSE7_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE7`"]
pub type ISENSE7_R = crate::R<bool, ISENSE7_A>;
impl ISENSE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE7_A {
        match self.bits {
            false => ISENSE7_A::EDGE,
            true => ISENSE7_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE7_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE7_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE7`"]
pub struct ISENSE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE7_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE7_A::LEVEL)
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
pub enum ISENSE8_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE8_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE8_A) -> Self {
        match variant {
            ISENSE8_A::EDGE => false,
            ISENSE8_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE8`"]
pub type ISENSE8_R = crate::R<bool, ISENSE8_A>;
impl ISENSE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE8_A {
        match self.bits {
            false => ISENSE8_A::EDGE,
            true => ISENSE8_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE8_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE8_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE8`"]
pub struct ISENSE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE8_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE8_A::LEVEL)
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
pub enum ISENSE9_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE9_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE9_A) -> Self {
        match variant {
            ISENSE9_A::EDGE => false,
            ISENSE9_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE9`"]
pub type ISENSE9_R = crate::R<bool, ISENSE9_A>;
impl ISENSE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE9_A {
        match self.bits {
            false => ISENSE9_A::EDGE,
            true => ISENSE9_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE9_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE9_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE9`"]
pub struct ISENSE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE9_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE9_A::LEVEL)
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
pub enum ISENSE10_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE10_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE10_A) -> Self {
        match variant {
            ISENSE10_A::EDGE => false,
            ISENSE10_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE10`"]
pub type ISENSE10_R = crate::R<bool, ISENSE10_A>;
impl ISENSE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE10_A {
        match self.bits {
            false => ISENSE10_A::EDGE,
            true => ISENSE10_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE10_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE10_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE10`"]
pub struct ISENSE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE10_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE10_A::LEVEL)
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
pub enum ISENSE11_A {
    #[doc = "0: Pin interrupt is edge-sensitive"]
    EDGE,
    #[doc = "1: Pin interrupt is level-sensitive"]
    LEVEL,
}
impl From<ISENSE11_A> for bool {
    #[inline(always)]
    fn from(variant: ISENSE11_A) -> Self {
        match variant {
            ISENSE11_A::EDGE => false,
            ISENSE11_A::LEVEL => true,
        }
    }
}
#[doc = "Reader of field `ISENSE11`"]
pub type ISENSE11_R = crate::R<bool, ISENSE11_A>;
impl ISENSE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISENSE11_A {
        match self.bits {
            false => ISENSE11_A::EDGE,
            true => ISENSE11_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISENSE11_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISENSE11_A::LEVEL
    }
}
#[doc = "Write proxy for field `ISENSE11`"]
pub struct ISENSE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISENSE11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is edge-sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ISENSE11_A::EDGE)
    }
    #[doc = "Pin interrupt is level-sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENSE11_A::LEVEL)
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
    pub fn isense0(&self) -> ISENSE0_R {
        ISENSE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn isense1(&self) -> ISENSE1_R {
        ISENSE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn isense2(&self) -> ISENSE2_R {
        ISENSE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn isense3(&self) -> ISENSE3_R {
        ISENSE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn isense4(&self) -> ISENSE4_R {
        ISENSE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn isense5(&self) -> ISENSE5_R {
        ISENSE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn isense6(&self) -> ISENSE6_R {
        ISENSE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn isense7(&self) -> ISENSE7_R {
        ISENSE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn isense8(&self) -> ISENSE8_R {
        ISENSE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn isense9(&self) -> ISENSE9_R {
        ISENSE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn isense10(&self) -> ISENSE10_R {
        ISENSE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn isense11(&self) -> ISENSE11_R {
        ISENSE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIOn_0."]
    #[inline(always)]
    pub fn isense0(&mut self) -> ISENSE0_W {
        ISENSE0_W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn isense1(&mut self) -> ISENSE1_W {
        ISENSE1_W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn isense2(&mut self) -> ISENSE2_W {
        ISENSE2_W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn isense3(&mut self) -> ISENSE3_W {
        ISENSE3_W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn isense4(&mut self) -> ISENSE4_W {
        ISENSE4_W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn isense5(&mut self) -> ISENSE5_W {
        ISENSE5_W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn isense6(&mut self) -> ISENSE6_W {
        ISENSE6_W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn isense7(&mut self) -> ISENSE7_W {
        ISENSE7_W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn isense8(&mut self) -> ISENSE8_W {
        ISENSE8_W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn isense9(&mut self) -> ISENSE9_W {
        ISENSE9_W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn isense10(&mut self) -> ISENSE10_W {
        ISENSE10_W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn isense11(&mut self) -> ISENSE11_W {
        ISENSE11_W { w: self }
    }
}
