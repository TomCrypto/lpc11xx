#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTRC`"]
pub type DTRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTRC`"]
pub struct DTRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RTSC`"]
pub type RTSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTSC`"]
pub struct RTSC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LMS`"]
pub type LMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LMS`"]
pub struct LMS_W<'a> {
    w: &'a mut W,
}
impl<'a> LMS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "RTS flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEN_A {
    #[doc = "0: Disable auto-rts flow control"]
    DISABLE_AUTO_RTS_FLO,
    #[doc = "1: Enable auto-rts flow control"]
    ENABLE_AUTO_RTS_FLOW,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        match variant {
            RTSEN_A::DISABLE_AUTO_RTS_FLO => false,
            RTSEN_A::ENABLE_AUTO_RTS_FLOW => true,
        }
    }
}
#[doc = "Reader of field `RTSEN`"]
pub type RTSEN_R = crate::R<bool, RTSEN_A>;
impl RTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEN_A {
        match self.bits {
            false => RTSEN_A::DISABLE_AUTO_RTS_FLO,
            true => RTSEN_A::ENABLE_AUTO_RTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_RTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == RTSEN_A::DISABLE_AUTO_RTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_RTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == RTSEN_A::ENABLE_AUTO_RTS_FLOW
    }
}
#[doc = "Write proxy for field `RTSEN`"]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto-rts flow control"]
    #[inline(always)]
    pub fn disable_auto_rts_flo(self) -> &'a mut W {
        self.variant(RTSEN_A::DISABLE_AUTO_RTS_FLO)
    }
    #[doc = "Enable auto-rts flow control"]
    #[inline(always)]
    pub fn enable_auto_rts_flow(self) -> &'a mut W {
        self.variant(RTSEN_A::ENABLE_AUTO_RTS_FLOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "CTS flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "0: Disable auto-cts flow control"]
    DISABLE_AUTO_CTS_FLO,
    #[doc = "1: Enable auto-cts flow control"]
    ENABLE_AUTO_CTS_FLOW,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        match variant {
            CTSEN_A::DISABLE_AUTO_CTS_FLO => false,
            CTSEN_A::ENABLE_AUTO_CTS_FLOW => true,
        }
    }
}
#[doc = "Reader of field `CTSEN`"]
pub type CTSEN_R = crate::R<bool, CTSEN_A>;
impl CTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE_AUTO_CTS_FLO,
            true => CTSEN_A::ENABLE_AUTO_CTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_CTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == CTSEN_A::DISABLE_AUTO_CTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_CTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == CTSEN_A::ENABLE_AUTO_CTS_FLOW
    }
}
#[doc = "Write proxy for field `CTSEN`"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto-cts flow control"]
    #[inline(always)]
    pub fn disable_auto_cts_flo(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLE_AUTO_CTS_FLO)
    }
    #[doc = "Enable auto-cts flow control"]
    #[inline(always)]
    pub fn enable_auto_cts_flow(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLE_AUTO_CTS_FLOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrc(&self) -> DTRC_R {
        DTRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsc(&self) -> RTSC_R {
        RTSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTS flow control."]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTS flow control."]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrc(&mut self) -> DTRC_W {
        DTRC_W { w: self }
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsc(&mut self) -> RTSC_W {
        RTSC_W { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The four modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the four modem outputs are connected to the four modem inputs. As a result of these connections, the upper four bits of the MSR will be driven by the lower four bits of the MCR rather than the four modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&mut self) -> LMS_W {
        LMS_W { w: self }
    }
    #[doc = "Bit 6 - RTS flow control."]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bit 7 - CTS flow control."]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
}
