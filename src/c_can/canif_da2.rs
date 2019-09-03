#[doc = "Reader of register CANIF%s_DA2"]
pub type R = crate::R<u32, super::CANIF_DA2>;
#[doc = "Writer for register CANIF%s_DA2"]
pub type W = crate::W<u32, super::CANIF_DA2>;
#[doc = "Register CANIF%s_DA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIF_DA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA2`"]
pub type DATA2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA2`"]
pub struct DATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA3`"]
pub type DATA3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA3`"]
pub struct DATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 2."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 3."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 2."]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W {
        DATA2_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 3."]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W { w: self }
    }
}
