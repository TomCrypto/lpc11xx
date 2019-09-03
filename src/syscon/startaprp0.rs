#[doc = "Reader of register STARTAPRP0"]
pub type R = crate::R<u32, super::STARTAPRP0>;
#[doc = "Writer for register STARTAPRP0"]
pub type W = crate::W<u32, super::STARTAPRP0>;
#[doc = "Register STARTAPRP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTAPRP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APRPIO0_0`"]
pub type APRPIO0_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_0`"]
pub struct APRPIO0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_0_W<'a> {
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
#[doc = "Reader of field `APRPIO0_1`"]
pub type APRPIO0_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_1`"]
pub struct APRPIO0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_1_W<'a> {
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
#[doc = "Reader of field `APRPIO0_2`"]
pub type APRPIO0_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_2`"]
pub struct APRPIO0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_2_W<'a> {
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
#[doc = "Reader of field `APRPIO0_3`"]
pub type APRPIO0_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_3`"]
pub struct APRPIO0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_3_W<'a> {
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
#[doc = "Reader of field `APRPIO0_4`"]
pub type APRPIO0_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_4`"]
pub struct APRPIO0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_4_W<'a> {
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
#[doc = "Reader of field `APRPIO0_5`"]
pub type APRPIO0_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_5`"]
pub struct APRPIO0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_5_W<'a> {
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
#[doc = "Reader of field `APRPIO0_6`"]
pub type APRPIO0_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_6`"]
pub struct APRPIO0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_6_W<'a> {
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
#[doc = "Reader of field `APRPIO0_7`"]
pub type APRPIO0_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_7`"]
pub struct APRPIO0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_7_W<'a> {
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
#[doc = "Reader of field `APRPIO0_8`"]
pub type APRPIO0_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_8`"]
pub struct APRPIO0_8_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_8_W<'a> {
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
#[doc = "Reader of field `APRPIO0_9`"]
pub type APRPIO0_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_9`"]
pub struct APRPIO0_9_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_9_W<'a> {
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
#[doc = "Reader of field `APRPIO0_10`"]
pub type APRPIO0_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_10`"]
pub struct APRPIO0_10_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_10_W<'a> {
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
#[doc = "Reader of field `APRPIO0_11`"]
pub type APRPIO0_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_11`"]
pub struct APRPIO0_11_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_11_W<'a> {
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
#[doc = "Reader of field `APRPIO1_0`"]
pub type APRPIO1_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_0`"]
pub struct APRPIO1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_0(&self) -> APRPIO0_0_R {
        APRPIO0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_1(&self) -> APRPIO0_1_R {
        APRPIO0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_2(&self) -> APRPIO0_2_R {
        APRPIO0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_3(&self) -> APRPIO0_3_R {
        APRPIO0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_4(&self) -> APRPIO0_4_R {
        APRPIO0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_5(&self) -> APRPIO0_5_R {
        APRPIO0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_6(&self) -> APRPIO0_6_R {
        APRPIO0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_7(&self) -> APRPIO0_7_R {
        APRPIO0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_8(&self) -> APRPIO0_8_R {
        APRPIO0_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_9(&self) -> APRPIO0_9_R {
        APRPIO0_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_10(&self) -> APRPIO0_10_R {
        APRPIO0_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_11(&self) -> APRPIO0_11_R {
        APRPIO0_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_0(&self) -> APRPIO1_0_R {
        APRPIO1_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_0(&mut self) -> APRPIO0_0_W {
        APRPIO0_0_W { w: self }
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_1(&mut self) -> APRPIO0_1_W {
        APRPIO0_1_W { w: self }
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_2(&mut self) -> APRPIO0_2_W {
        APRPIO0_2_W { w: self }
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_3(&mut self) -> APRPIO0_3_W {
        APRPIO0_3_W { w: self }
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_4(&mut self) -> APRPIO0_4_W {
        APRPIO0_4_W { w: self }
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_5(&mut self) -> APRPIO0_5_W {
        APRPIO0_5_W { w: self }
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_6(&mut self) -> APRPIO0_6_W {
        APRPIO0_6_W { w: self }
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_7(&mut self) -> APRPIO0_7_W {
        APRPIO0_7_W { w: self }
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_8(&mut self) -> APRPIO0_8_W {
        APRPIO0_8_W { w: self }
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_9(&mut self) -> APRPIO0_9_W {
        APRPIO0_9_W { w: self }
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_10(&mut self) -> APRPIO0_10_W {
        APRPIO0_10_W { w: self }
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_11(&mut self) -> APRPIO0_11_W {
        APRPIO0_11_W { w: self }
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_0 0 = Falling edge 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_0(&mut self) -> APRPIO1_0_W {
        APRPIO1_0_W { w: self }
    }
}
