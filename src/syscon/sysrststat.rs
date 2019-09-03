#[doc = "Reader of register SYSRSTSTAT"]
pub type R = crate::R<u32, super::SYSRSTSTAT>;
#[doc = "POR reset status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: No POR detected"]
    NO_POR_DETECTED_,
    #[doc = "1: POR detected. Writing a one clears this reset"]
    POR_DETECTED_WRITIN,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        match variant {
            POR_A::NO_POR_DETECTED_ => false,
            POR_A::POR_DETECTED_WRITIN => true,
        }
    }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, POR_A>;
impl POR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::NO_POR_DETECTED_,
            true => POR_A::POR_DETECTED_WRITIN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POR_DETECTED_`"]
    #[inline(always)]
    pub fn is_no_por_detected_(&self) -> bool {
        *self == POR_A::NO_POR_DETECTED_
    }
    #[doc = "Checks if the value of the field is `POR_DETECTED_WRITIN`"]
    #[inline(always)]
    pub fn is_por_detected_writin(&self) -> bool {
        *self == POR_A::POR_DETECTED_WRITIN
    }
}
#[doc = "Status of the external RESET pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRST_A {
    #[doc = "0: No RESET event detected"]
    NO_RESET_EVENT_DETEC,
    #[doc = "1: RESET detected. Writing a one clears this reset"]
    RESET_DETECTED_WRIT,
}
impl From<EXTRST_A> for bool {
    #[inline(always)]
    fn from(variant: EXTRST_A) -> Self {
        match variant {
            EXTRST_A::NO_RESET_EVENT_DETEC => false,
            EXTRST_A::RESET_DETECTED_WRIT => true,
        }
    }
}
#[doc = "Reader of field `EXTRST`"]
pub type EXTRST_R = crate::R<bool, EXTRST_A>;
impl EXTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTRST_A {
        match self.bits {
            false => EXTRST_A::NO_RESET_EVENT_DETEC,
            true => EXTRST_A::RESET_DETECTED_WRIT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET_EVENT_DETEC`"]
    #[inline(always)]
    pub fn is_no_reset_event_detec(&self) -> bool {
        *self == EXTRST_A::NO_RESET_EVENT_DETEC
    }
    #[doc = "Checks if the value of the field is `RESET_DETECTED_WRIT`"]
    #[inline(always)]
    pub fn is_reset_detected_writ(&self) -> bool {
        *self == EXTRST_A::RESET_DETECTED_WRIT
    }
}
#[doc = "Status of the Watchdog reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: No WDT reset detected"]
    NO_WDT_RESET_DETECTE,
    #[doc = "1: WDT reset detected. Writing a one clears this reset"]
    WDT_RESET_DETECTED_,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        match variant {
            WDT_A::NO_WDT_RESET_DETECTE => false,
            WDT_A::WDT_RESET_DETECTED_ => true,
        }
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, WDT_A>;
impl WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::NO_WDT_RESET_DETECTE,
            true => WDT_A::WDT_RESET_DETECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WDT_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_no_wdt_reset_detecte(&self) -> bool {
        *self == WDT_A::NO_WDT_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `WDT_RESET_DETECTED_`"]
    #[inline(always)]
    pub fn is_wdt_reset_detected_(&self) -> bool {
        *self == WDT_A::WDT_RESET_DETECTED_
    }
}
#[doc = "Status of the Brown-out detect reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_A {
    #[doc = "0: No BOD reset detected"]
    NO_BOD_RESET_DETECTE,
    #[doc = "1: BOD reset detected. Writing a one clears this reset"]
    BOD_RESET_DETECTED_,
}
impl From<BOD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_A) -> Self {
        match variant {
            BOD_A::NO_BOD_RESET_DETECTE => false,
            BOD_A::BOD_RESET_DETECTED_ => true,
        }
    }
}
#[doc = "Reader of field `BOD`"]
pub type BOD_R = crate::R<bool, BOD_A>;
impl BOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_A {
        match self.bits {
            false => BOD_A::NO_BOD_RESET_DETECTE,
            true => BOD_A::BOD_RESET_DETECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BOD_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_no_bod_reset_detecte(&self) -> bool {
        *self == BOD_A::NO_BOD_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `BOD_RESET_DETECTED_`"]
    #[inline(always)]
    pub fn is_bod_reset_detected_(&self) -> bool {
        *self == BOD_A::BOD_RESET_DETECTED_
    }
}
#[doc = "Status of the software system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRST_A {
    #[doc = "0: No System reset detected"]
    NO_SYSTEM_RESET_DETE,
    #[doc = "1: System reset detected. Writing a one clears this reset"]
    SYSTEM_RESET_DETECTE,
}
impl From<SYSRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRST_A) -> Self {
        match variant {
            SYSRST_A::NO_SYSTEM_RESET_DETE => false,
            SYSRST_A::SYSTEM_RESET_DETECTE => true,
        }
    }
}
#[doc = "Reader of field `SYSRST`"]
pub type SYSRST_R = crate::R<bool, SYSRST_A>;
impl SYSRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRST_A {
        match self.bits {
            false => SYSRST_A::NO_SYSTEM_RESET_DETE,
            true => SYSRST_A::SYSTEM_RESET_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYSTEM_RESET_DETE`"]
    #[inline(always)]
    pub fn is_no_system_reset_dete(&self) -> bool {
        *self == SYSRST_A::NO_SYSTEM_RESET_DETE
    }
    #[doc = "Checks if the value of the field is `SYSTEM_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_system_reset_detecte(&self) -> bool {
        *self == SYSRST_A::SYSTEM_RESET_DETECTE
    }
}
impl R {
    #[doc = "Bit 0 - POR reset status."]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the external RESET pin."]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset."]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the software system reset."]
    #[inline(always)]
    pub fn sysrst(&self) -> SYSRST_R {
        SYSRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
