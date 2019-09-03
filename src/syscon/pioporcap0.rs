#[doc = "Reader of register PIOPORCAP0"]
pub type R = crate::R<u32, super::PIOPORCAP0>;
#[doc = "Reader of field `CAPPIO0_n`"]
pub type CAPPIO0_N_R = crate::R<u16, u16>;
#[doc = "Reader of field `CAPPIO1_n`"]
pub type CAPPIO1_N_R = crate::R<u16, u16>;
#[doc = "Reader of field `CAPPIO2_n`"]
pub type CAPPIO2_N_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Raw reset status input PIO0_n: PIO0_11 to PIO0_0."]
    #[inline(always)]
    pub fn cappio0_n(&self) -> CAPPIO0_N_R {
        CAPPIO0_N_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Raw reset status input PIO1_n: PIO1_11 to PIO1_0."]
    #[inline(always)]
    pub fn cappio1_n(&self) -> CAPPIO1_N_R {
        CAPPIO1_N_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - Raw reset status input PIO2_n: PIO2_7 to PIO2_0."]
    #[inline(always)]
    pub fn cappio2_n(&self) -> CAPPIO2_N_R {
        CAPPIO2_N_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
