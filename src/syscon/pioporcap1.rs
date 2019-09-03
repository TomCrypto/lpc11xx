#[doc = "Reader of register PIOPORCAP1"]
pub type R = crate::R<u32, super::PIOPORCAP1>;
#[doc = "Reader of field `CAPPIO2_8`"]
pub type CAPPIO2_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_9`"]
pub type CAPPIO2_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_10`"]
pub type CAPPIO2_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_11`"]
pub type CAPPIO2_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO3_0`"]
pub type CAPPIO3_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO3_1`"]
pub type CAPPIO3_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO3_2`"]
pub type CAPPIO3_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO3_3`"]
pub type CAPPIO3_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO3_4`"]
pub type CAPPIO3_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO3_5`"]
pub type CAPPIO3_5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw reset status input PIO2_8."]
    #[inline(always)]
    pub fn cappio2_8(&self) -> CAPPIO2_8_R {
        CAPPIO2_8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw reset status input PIO2_9."]
    #[inline(always)]
    pub fn cappio2_9(&self) -> CAPPIO2_9_R {
        CAPPIO2_9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw reset status input PIO2_10."]
    #[inline(always)]
    pub fn cappio2_10(&self) -> CAPPIO2_10_R {
        CAPPIO2_10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raw reset status input PIO2_11."]
    #[inline(always)]
    pub fn cappio2_11(&self) -> CAPPIO2_11_R {
        CAPPIO2_11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Raw reset status input PIO3_0."]
    #[inline(always)]
    pub fn cappio3_0(&self) -> CAPPIO3_0_R {
        CAPPIO3_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Raw reset status input PIO3_1."]
    #[inline(always)]
    pub fn cappio3_1(&self) -> CAPPIO3_1_R {
        CAPPIO3_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Raw reset status input PIO3_2."]
    #[inline(always)]
    pub fn cappio3_2(&self) -> CAPPIO3_2_R {
        CAPPIO3_2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Raw reset status input PIO3_3."]
    #[inline(always)]
    pub fn cappio3_3(&self) -> CAPPIO3_3_R {
        CAPPIO3_3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Raw reset status input PIO3_4."]
    #[inline(always)]
    pub fn cappio3_4(&self) -> CAPPIO3_4_R {
        CAPPIO3_4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Raw reset status input PIO3_5."]
    #[inline(always)]
    pub fn cappio3_5(&self) -> CAPPIO3_5_R {
        CAPPIO3_5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
