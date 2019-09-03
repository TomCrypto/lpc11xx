#[doc = "Reader of register CANBRPE"]
pub type R = crate::R<u32, super::CANBRPE>;
#[doc = "Writer for register CANBRPE"]
pub type W = crate::W<u32, super::CANBRPE>;
#[doc = "Register CANBRPE `reset()`'s with value 0"]
impl crate::ResetValue for super::CANBRPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRPE`"]
pub type BRPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRPE`"]
pub struct BRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Baud rate prescaler extension By programming BRPE the Baud Rate Prescaler can be extended to values up to 1023. Hardware interprets the value as the value of BRPE (MSBs) and BRP (LSBs) plus one. Allowed values are 0 to 15."]
    #[inline(always)]
    pub fn brpe(&self) -> BRPE_R {
        BRPE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud rate prescaler extension By programming BRPE the Baud Rate Prescaler can be extended to values up to 1023. Hardware interprets the value as the value of BRPE (MSBs) and BRP (LSBs) plus one. Allowed values are 0 to 15."]
    #[inline(always)]
    pub fn brpe(&mut self) -> BRPE_W {
        BRPE_W { w: self }
    }
}
