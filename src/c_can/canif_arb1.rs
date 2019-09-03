#[doc = "Reader of register CANIF%s_ARB1"]
pub type R = crate::R<u32, super::CANIF_ARB1>;
#[doc = "Writer for register CANIF%s_ARB1"]
pub type W = crate::W<u32, super::CANIF_ARB1>;
#[doc = "Register CANIF%s_ARB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIF_ARB1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID_15_0`"]
pub type ID_15_0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ID_15_0`"]
pub struct ID_15_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_15_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)."]
    #[inline(always)]
    pub fn id_15_0(&self) -> ID_15_0_R {
        ID_15_0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message identifier 29-bit identifier (extended frame) 11-bit identifier (standard frame)."]
    #[inline(always)]
    pub fn id_15_0(&mut self) -> ID_15_0_W {
        ID_15_0_W { w: self }
    }
}
