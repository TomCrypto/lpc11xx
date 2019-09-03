#[doc = "Reader of register CANND1"]
pub type R = crate::R<u32, super::CANND1>;
#[doc = "Reader of field `NEWDAT_16_1`"]
pub type NEWDAT_16_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - New data bits of message objects 16 to 1. 0 = No new data has been written into the data portion of this Message Object by the Message Handler since last time this flag was cleared by the CPU. 1 = The Message Handler or the CPU has written new data into the data portion of this Message Object."]
    #[inline(always)]
    pub fn newdat_16_1(&self) -> NEWDAT_16_1_R {
        NEWDAT_16_1_R::new((self.bits & 0xffff) as u16)
    }
}
