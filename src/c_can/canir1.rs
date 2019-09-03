#[doc = "Reader of register CANIR1"]
pub type R = crate::R<u32, super::CANIR1>;
#[doc = "Reader of field `INTPND_16_1`"]
pub type INTPND_16_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt pending bits of message objects 16 to 1. 0 = This message object is ignored by the message handler. 1 = This message object is the source of an interrupt."]
    #[inline(always)]
    pub fn intpnd_16_1(&self) -> INTPND_16_1_R {
        INTPND_16_1_R::new((self.bits & 0xffff) as u16)
    }
}
