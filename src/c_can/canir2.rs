#[doc = "Reader of register CANIR2"]
pub type R = crate::R<u32, super::CANIR2>;
#[doc = "Reader of field `INTPND_32_17`"]
pub type INTPND_32_17_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt pending bits of message objects 32 to 17. 0 = This message object is ignored by the message handler. 1 = This message object is the source of an interrupt."]
    #[inline(always)]
    pub fn intpnd_32_17(&self) -> INTPND_32_17_R {
        INTPND_32_17_R::new((self.bits & 0xffff) as u16)
    }
}
