#[doc = "Reader of register CANMSGV1"]
pub type R = crate::R<u32, super::CANMSGV1>;
#[doc = "Reader of field `MSGVAL_16_1`"]
pub type MSGVAL_16_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Message valid bits of message objects 16 to 1. 0 = This message object is ignored by the message handler. 1 = This message object is configured and should be considered by the message handler."]
    #[inline(always)]
    pub fn msgval_16_1(&self) -> MSGVAL_16_1_R {
        MSGVAL_16_1_R::new((self.bits & 0xffff) as u16)
    }
}
