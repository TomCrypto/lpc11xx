#[doc = "Reader of register CANEC"]
pub type R = crate::R<u32, super::CANEC>;
#[doc = "Reader of field `TEC_7_0`"]
pub type TEC_7_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `REC_6_0`"]
pub type REC_6_0_R = crate::R<u8, u8>;
#[doc = "Receive error passive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RP_A {
    #[doc = "0: The receive counter is below the error passive level"]
    BELOWERRORPASSIVE,
    #[doc = "1: The receive counter has reached the error passive level as defined in the CAN2.0 specification"]
    ERRORPASSIVE,
}
impl From<RP_A> for bool {
    #[inline(always)]
    fn from(variant: RP_A) -> Self {
        match variant {
            RP_A::BELOWERRORPASSIVE => false,
            RP_A::ERRORPASSIVE => true,
        }
    }
}
#[doc = "Reader of field `RP`"]
pub type RP_R = crate::R<bool, RP_A>;
impl RP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_A {
        match self.bits {
            false => RP_A::BELOWERRORPASSIVE,
            true => RP_A::ERRORPASSIVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOWERRORPASSIVE`"]
    #[inline(always)]
    pub fn is_belowerrorpassive(&self) -> bool {
        *self == RP_A::BELOWERRORPASSIVE
    }
    #[doc = "Checks if the value of the field is `ERRORPASSIVE`"]
    #[inline(always)]
    pub fn is_errorpassive(&self) -> bool {
        *self == RP_A::ERRORPASSIVE
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit error counter Current value of the transmit error counter (maximum value 255)."]
    #[inline(always)]
    pub fn tec_7_0(&self) -> TEC_7_0_R {
        TEC_7_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive error counter Current value of the receive error counter (maximum value 127)."]
    #[inline(always)]
    pub fn rec_6_0(&self) -> REC_6_0_R {
        REC_6_0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive error passive."]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
