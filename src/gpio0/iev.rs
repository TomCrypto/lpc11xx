#[doc = "Reader of register IEV"]
pub type R = crate::R<u32, super::IEV>;
#[doc = "Writer for register IEV"]
pub type W = crate::W<u32, super::IEV>;
#[doc = "Register IEV `reset()`'s with value 0"]
impl crate::ResetValue for super::IEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV0_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV0_A> for bool {
    #[inline(always)]
    fn from(variant: IEV0_A) -> Self {
        match variant {
            IEV0_A::FALLING => false,
            IEV0_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV0`"]
pub type IEV0_R = crate::R<bool, IEV0_A>;
impl IEV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV0_A {
        match self.bits {
            false => IEV0_A::FALLING,
            true => IEV0_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV0_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV0_A::RISING
    }
}
#[doc = "Write proxy for field `IEV0`"]
pub struct IEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV0_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV0_A::RISING)
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
pub enum IEV1_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV1_A> for bool {
    #[inline(always)]
    fn from(variant: IEV1_A) -> Self {
        match variant {
            IEV1_A::FALLING => false,
            IEV1_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV1`"]
pub type IEV1_R = crate::R<bool, IEV1_A>;
impl IEV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV1_A {
        match self.bits {
            false => IEV1_A::FALLING,
            true => IEV1_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV1_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV1_A::RISING
    }
}
#[doc = "Write proxy for field `IEV1`"]
pub struct IEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV1_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV1_A::RISING)
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
pub enum IEV2_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV2_A> for bool {
    #[inline(always)]
    fn from(variant: IEV2_A) -> Self {
        match variant {
            IEV2_A::FALLING => false,
            IEV2_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV2`"]
pub type IEV2_R = crate::R<bool, IEV2_A>;
impl IEV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV2_A {
        match self.bits {
            false => IEV2_A::FALLING,
            true => IEV2_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV2_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV2_A::RISING
    }
}
#[doc = "Write proxy for field `IEV2`"]
pub struct IEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV2_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV2_A::RISING)
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
pub enum IEV3_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV3_A> for bool {
    #[inline(always)]
    fn from(variant: IEV3_A) -> Self {
        match variant {
            IEV3_A::FALLING => false,
            IEV3_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV3`"]
pub type IEV3_R = crate::R<bool, IEV3_A>;
impl IEV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV3_A {
        match self.bits {
            false => IEV3_A::FALLING,
            true => IEV3_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV3_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV3_A::RISING
    }
}
#[doc = "Write proxy for field `IEV3`"]
pub struct IEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV3_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV3_A::RISING)
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
pub enum IEV4_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV4_A> for bool {
    #[inline(always)]
    fn from(variant: IEV4_A) -> Self {
        match variant {
            IEV4_A::FALLING => false,
            IEV4_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV4`"]
pub type IEV4_R = crate::R<bool, IEV4_A>;
impl IEV4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV4_A {
        match self.bits {
            false => IEV4_A::FALLING,
            true => IEV4_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV4_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV4_A::RISING
    }
}
#[doc = "Write proxy for field `IEV4`"]
pub struct IEV4_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV4_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV4_A::RISING)
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
pub enum IEV5_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV5_A> for bool {
    #[inline(always)]
    fn from(variant: IEV5_A) -> Self {
        match variant {
            IEV5_A::FALLING => false,
            IEV5_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV5`"]
pub type IEV5_R = crate::R<bool, IEV5_A>;
impl IEV5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV5_A {
        match self.bits {
            false => IEV5_A::FALLING,
            true => IEV5_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV5_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV5_A::RISING
    }
}
#[doc = "Write proxy for field `IEV5`"]
pub struct IEV5_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV5_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV5_A::RISING)
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
pub enum IEV6_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV6_A> for bool {
    #[inline(always)]
    fn from(variant: IEV6_A) -> Self {
        match variant {
            IEV6_A::FALLING => false,
            IEV6_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV6`"]
pub type IEV6_R = crate::R<bool, IEV6_A>;
impl IEV6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV6_A {
        match self.bits {
            false => IEV6_A::FALLING,
            true => IEV6_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV6_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV6_A::RISING
    }
}
#[doc = "Write proxy for field `IEV6`"]
pub struct IEV6_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV6_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV6_A::RISING)
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
pub enum IEV7_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV7_A> for bool {
    #[inline(always)]
    fn from(variant: IEV7_A) -> Self {
        match variant {
            IEV7_A::FALLING => false,
            IEV7_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV7`"]
pub type IEV7_R = crate::R<bool, IEV7_A>;
impl IEV7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV7_A {
        match self.bits {
            false => IEV7_A::FALLING,
            true => IEV7_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV7_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV7_A::RISING
    }
}
#[doc = "Write proxy for field `IEV7`"]
pub struct IEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV7_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV7_A::RISING)
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
pub enum IEV8_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV8_A> for bool {
    #[inline(always)]
    fn from(variant: IEV8_A) -> Self {
        match variant {
            IEV8_A::FALLING => false,
            IEV8_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV8`"]
pub type IEV8_R = crate::R<bool, IEV8_A>;
impl IEV8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV8_A {
        match self.bits {
            false => IEV8_A::FALLING,
            true => IEV8_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV8_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV8_A::RISING
    }
}
#[doc = "Write proxy for field `IEV8`"]
pub struct IEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV8_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV8_A::RISING)
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
pub enum IEV9_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV9_A> for bool {
    #[inline(always)]
    fn from(variant: IEV9_A) -> Self {
        match variant {
            IEV9_A::FALLING => false,
            IEV9_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV9`"]
pub type IEV9_R = crate::R<bool, IEV9_A>;
impl IEV9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV9_A {
        match self.bits {
            false => IEV9_A::FALLING,
            true => IEV9_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV9_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV9_A::RISING
    }
}
#[doc = "Write proxy for field `IEV9`"]
pub struct IEV9_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV9_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV9_A::RISING)
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
pub enum IEV10_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV10_A> for bool {
    #[inline(always)]
    fn from(variant: IEV10_A) -> Self {
        match variant {
            IEV10_A::FALLING => false,
            IEV10_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV10`"]
pub type IEV10_R = crate::R<bool, IEV10_A>;
impl IEV10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV10_A {
        match self.bits {
            false => IEV10_A::FALLING,
            true => IEV10_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV10_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV10_A::RISING
    }
}
#[doc = "Write proxy for field `IEV10`"]
pub struct IEV10_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV10_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV10_A::RISING)
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
pub enum IEV11_A {
    #[doc = "0: Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    FALLING,
    #[doc = "1: Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    RISING,
}
impl From<IEV11_A> for bool {
    #[inline(always)]
    fn from(variant: IEV11_A) -> Self {
        match variant {
            IEV11_A::FALLING => false,
            IEV11_A::RISING => true,
        }
    }
}
#[doc = "Reader of field `IEV11`"]
pub type IEV11_R = crate::R<bool, IEV11_A>;
impl IEV11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV11_A {
        match self.bits {
            false => IEV11_A::FALLING,
            true => IEV11_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == IEV11_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == IEV11_A::RISING
    }
}
#[doc = "Write proxy for field `IEV11`"]
pub struct IEV11_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEV11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin interrupt is triggered on falling edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(IEV11_A::FALLING)
    }
    #[doc = "Pin interrupt is triggered on rising edges (if edge-sensitive)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(IEV11_A::RISING)
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
    pub fn iev0(&self) -> IEV0_R {
        IEV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn iev1(&self) -> IEV1_R {
        IEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn iev2(&self) -> IEV2_R {
        IEV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn iev3(&self) -> IEV3_R {
        IEV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn iev4(&self) -> IEV4_R {
        IEV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn iev5(&self) -> IEV5_R {
        IEV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn iev6(&self) -> IEV6_R {
        IEV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn iev7(&self) -> IEV7_R {
        IEV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn iev8(&self) -> IEV8_R {
        IEV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn iev9(&self) -> IEV9_R {
        IEV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn iev10(&self) -> IEV10_R {
        IEV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn iev11(&self) -> IEV11_R {
        IEV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIOn_0."]
    #[inline(always)]
    pub fn iev0(&mut self) -> IEV0_W {
        IEV0_W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn iev1(&mut self) -> IEV1_W {
        IEV1_W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn iev2(&mut self) -> IEV2_W {
        IEV2_W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn iev3(&mut self) -> IEV3_W {
        IEV3_W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn iev4(&mut self) -> IEV4_W {
        IEV4_W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn iev5(&mut self) -> IEV5_W {
        IEV5_W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn iev6(&mut self) -> IEV6_W {
        IEV6_W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn iev7(&mut self) -> IEV7_W {
        IEV7_W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn iev8(&mut self) -> IEV8_W {
        IEV8_W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn iev9(&mut self) -> IEV9_W {
        IEV9_W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn iev10(&mut self) -> IEV10_W {
        IEV10_W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn iev11(&mut self) -> IEV11_W {
        IEV11_W { w: self }
    }
}
