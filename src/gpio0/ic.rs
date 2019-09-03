#[doc = "Writer for register IC"]
pub type W = crate::W<u32, super::IC>;
#[doc = "Register IC `reset()`'s with value 0"]
impl crate::ResetValue for super::IC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR0_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR0_AW) -> Self {
        match variant {
            CLR0_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR0`"]
pub struct CLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR0_AW::CLEAR_INTERRUPT)
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
pub enum CLR1_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR1_AW) -> Self {
        match variant {
            CLR1_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR1`"]
pub struct CLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR1_AW::CLEAR_INTERRUPT)
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
pub enum CLR2_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR2_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR2_AW) -> Self {
        match variant {
            CLR2_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR2`"]
pub struct CLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR2_AW::CLEAR_INTERRUPT)
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
pub enum CLR3_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR3_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR3_AW) -> Self {
        match variant {
            CLR3_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR3`"]
pub struct CLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR3_AW::CLEAR_INTERRUPT)
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
pub enum CLR4_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR4_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR4_AW) -> Self {
        match variant {
            CLR4_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR4`"]
pub struct CLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR4_AW::CLEAR_INTERRUPT)
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
pub enum CLR5_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR5_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR5_AW) -> Self {
        match variant {
            CLR5_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR5`"]
pub struct CLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR5_AW::CLEAR_INTERRUPT)
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
pub enum CLR6_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR6_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR6_AW) -> Self {
        match variant {
            CLR6_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR6`"]
pub struct CLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR6_AW::CLEAR_INTERRUPT)
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
pub enum CLR7_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR7_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR7_AW) -> Self {
        match variant {
            CLR7_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR7`"]
pub struct CLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR7_AW::CLEAR_INTERRUPT)
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
pub enum CLR8_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR8_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR8_AW) -> Self {
        match variant {
            CLR8_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR8`"]
pub struct CLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR8_AW::CLEAR_INTERRUPT)
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
pub enum CLR9_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR9_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR9_AW) -> Self {
        match variant {
            CLR9_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR9`"]
pub struct CLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR9_AW::CLEAR_INTERRUPT)
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
pub enum CLR10_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR10_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR10_AW) -> Self {
        match variant {
            CLR10_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR10`"]
pub struct CLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR10_AW::CLEAR_INTERRUPT)
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
pub enum CLR11_AW {
    #[doc = "1: Clear pin interrupt."]
    CLEAR_INTERRUPT,
}
impl From<CLR11_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR11_AW) -> Self {
        match variant {
            CLR11_AW::CLEAR_INTERRUPT => true,
        }
    }
}
#[doc = "Write proxy for field `CLR11`"]
pub struct CLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear pin interrupt."]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut W {
        self.variant(CLR11_AW::CLEAR_INTERRUPT)
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
impl W {
    #[doc = "Bit 0 - PIOn_0."]
    #[inline(always)]
    pub fn clr0(&mut self) -> CLR0_W {
        CLR0_W { w: self }
    }
    #[doc = "Bit 1 - PIOn_1."]
    #[inline(always)]
    pub fn clr1(&mut self) -> CLR1_W {
        CLR1_W { w: self }
    }
    #[doc = "Bit 2 - PIOn_2."]
    #[inline(always)]
    pub fn clr2(&mut self) -> CLR2_W {
        CLR2_W { w: self }
    }
    #[doc = "Bit 3 - PIOn_3."]
    #[inline(always)]
    pub fn clr3(&mut self) -> CLR3_W {
        CLR3_W { w: self }
    }
    #[doc = "Bit 4 - PIOn_4."]
    #[inline(always)]
    pub fn clr4(&mut self) -> CLR4_W {
        CLR4_W { w: self }
    }
    #[doc = "Bit 5 - PIOn_5."]
    #[inline(always)]
    pub fn clr5(&mut self) -> CLR5_W {
        CLR5_W { w: self }
    }
    #[doc = "Bit 6 - PIOn_6."]
    #[inline(always)]
    pub fn clr6(&mut self) -> CLR6_W {
        CLR6_W { w: self }
    }
    #[doc = "Bit 7 - PIOn_7."]
    #[inline(always)]
    pub fn clr7(&mut self) -> CLR7_W {
        CLR7_W { w: self }
    }
    #[doc = "Bit 8 - PIOn_8."]
    #[inline(always)]
    pub fn clr8(&mut self) -> CLR8_W {
        CLR8_W { w: self }
    }
    #[doc = "Bit 9 - PIOn_9."]
    #[inline(always)]
    pub fn clr9(&mut self) -> CLR9_W {
        CLR9_W { w: self }
    }
    #[doc = "Bit 10 - PIOn_10."]
    #[inline(always)]
    pub fn clr10(&mut self) -> CLR10_W {
        CLR10_W { w: self }
    }
    #[doc = "Bit 11 - PIOn_11."]
    #[inline(always)]
    pub fn clr11(&mut self) -> CLR11_W {
        CLR11_W { w: self }
    }
}
