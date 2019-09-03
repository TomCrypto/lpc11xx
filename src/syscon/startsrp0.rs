#[doc = "Reader of register STARTSRP0"]
pub type R = crate::R<u32, super::STARTSRP0>;
#[doc = "Reader of field `SRPIO0_0`"]
pub type SRPIO0_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_1`"]
pub type SRPIO0_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_2`"]
pub type SRPIO0_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_3`"]
pub type SRPIO0_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_4`"]
pub type SRPIO0_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_5`"]
pub type SRPIO0_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_6`"]
pub type SRPIO0_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_7`"]
pub type SRPIO0_7_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_8`"]
pub type SRPIO0_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_9`"]
pub type SRPIO0_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_10`"]
pub type SRPIO0_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_11`"]
pub type SRPIO0_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_0`"]
pub type SRPIO1_0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_0(&self) -> SRPIO0_0_R {
        SRPIO0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_1(&self) -> SRPIO0_1_R {
        SRPIO0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_2(&self) -> SRPIO0_2_R {
        SRPIO0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_3(&self) -> SRPIO0_3_R {
        SRPIO0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_4(&self) -> SRPIO0_4_R {
        SRPIO0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_5(&self) -> SRPIO0_5_R {
        SRPIO0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_6(&self) -> SRPIO0_6_R {
        SRPIO0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_7(&self) -> SRPIO0_7_R {
        SRPIO0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_8(&self) -> SRPIO0_8_R {
        SRPIO0_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_9(&self) -> SRPIO0_9_R {
        SRPIO0_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_10(&self) -> SRPIO0_10_R {
        SRPIO0_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Start signal status for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_11(&self) -> SRPIO0_11_R {
        SRPIO0_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Start signal status for start logic input PIO1_0 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_0(&self) -> SRPIO1_0_R {
        SRPIO1_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
