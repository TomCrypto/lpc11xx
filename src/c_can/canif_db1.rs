#[doc = "Reader of register CANIF%s_DB1"]
pub type R = crate::R<u32, super::CANIF_DB1>;
#[doc = "Writer for register CANIF%s_DB1"]
pub type W = crate::W<u32, super::CANIF_DB1>;
#[doc = "Register CANIF%s_DB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIF_DB1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA4`"]
pub type DATA4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA4`"]
pub struct DATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA5`"]
pub type DATA5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA5`"]
pub struct DATA5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 4."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 4."]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 5."]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W { w: self }
    }
}
