#[doc = "Reader of register SCLH"]
pub type R = crate::R<u32, super::SCLH>;
#[doc = "Writer for register SCLH"]
pub type W = crate::W<u32, super::SCLH>;
#[doc = "Register SCLH `reset()`'s with value 0x04"]
impl crate::ResetValue for super::SCLH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `SCLH`"]
pub type SCLH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCLH`"]
pub struct SCLH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W {
        SCLH_W { w: self }
    }
}
