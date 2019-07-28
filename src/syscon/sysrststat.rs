#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYSRSTSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `POR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR {
    #[doc = "No POR detected"]
    NO_POR_DETECTED_,
    #[doc = "POR detected. Writing a one clears this reset"]
    POR_DETECTED_WRITIN,
}
impl PORR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PORR::NO_POR_DETECTED_ => false,
            PORR::POR_DETECTED_WRITIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PORR {
        match value {
            false => PORR::NO_POR_DETECTED_,
            true => PORR::POR_DETECTED_WRITIN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POR_DETECTED_`"]
    #[inline]
    pub fn is_no_por_detected_(&self) -> bool {
        *self == PORR::NO_POR_DETECTED_
    }
    #[doc = "Checks if the value of the field is `POR_DETECTED_WRITIN`"]
    #[inline]
    pub fn is_por_detected_writin(&self) -> bool {
        *self == PORR::POR_DETECTED_WRITIN
    }
}
#[doc = "Possible values of the field `EXTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRSTR {
    #[doc = "No RESET event detected"]
    NO_RESET_EVENT_DETEC,
    #[doc = "RESET detected. Writing a one clears this reset"]
    RESET_DETECTED_WRIT,
}
impl EXTRSTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EXTRSTR::NO_RESET_EVENT_DETEC => false,
            EXTRSTR::RESET_DETECTED_WRIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTRSTR {
        match value {
            false => EXTRSTR::NO_RESET_EVENT_DETEC,
            true => EXTRSTR::RESET_DETECTED_WRIT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET_EVENT_DETEC`"]
    #[inline]
    pub fn is_no_reset_event_detec(&self) -> bool {
        *self == EXTRSTR::NO_RESET_EVENT_DETEC
    }
    #[doc = "Checks if the value of the field is `RESET_DETECTED_WRIT`"]
    #[inline]
    pub fn is_reset_detected_writ(&self) -> bool {
        *self == EXTRSTR::RESET_DETECTED_WRIT
    }
}
#[doc = "Possible values of the field `WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTR {
    #[doc = "No WDT reset detected"]
    NO_WDT_RESET_DETECTE,
    #[doc = "WDT reset detected. Writing a one clears this reset"]
    WDT_RESET_DETECTED_,
}
impl WDTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WDTR::NO_WDT_RESET_DETECTE => false,
            WDTR::WDT_RESET_DETECTED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTR {
        match value {
            false => WDTR::NO_WDT_RESET_DETECTE,
            true => WDTR::WDT_RESET_DETECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WDT_RESET_DETECTE`"]
    #[inline]
    pub fn is_no_wdt_reset_detecte(&self) -> bool {
        *self == WDTR::NO_WDT_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `WDT_RESET_DETECTED_`"]
    #[inline]
    pub fn is_wdt_reset_detected_(&self) -> bool {
        *self == WDTR::WDT_RESET_DETECTED_
    }
}
#[doc = "Possible values of the field `BOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODR {
    #[doc = "No BOD reset detected"]
    NO_BOD_RESET_DETECTE,
    #[doc = "BOD reset detected. Writing a one clears this reset"]
    BOD_RESET_DETECTED_,
}
impl BODR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BODR::NO_BOD_RESET_DETECTE => false,
            BODR::BOD_RESET_DETECTED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODR {
        match value {
            false => BODR::NO_BOD_RESET_DETECTE,
            true => BODR::BOD_RESET_DETECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BOD_RESET_DETECTE`"]
    #[inline]
    pub fn is_no_bod_reset_detecte(&self) -> bool {
        *self == BODR::NO_BOD_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `BOD_RESET_DETECTED_`"]
    #[inline]
    pub fn is_bod_reset_detected_(&self) -> bool {
        *self == BODR::BOD_RESET_DETECTED_
    }
}
#[doc = "Possible values of the field `SYSRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTR {
    #[doc = "No System reset detected"]
    NO_SYSTEM_RESET_DETE,
    #[doc = "System reset detected. Writing a one clears this reset"]
    SYSTEM_RESET_DETECTE,
}
impl SYSRSTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYSRSTR::NO_SYSTEM_RESET_DETE => false,
            SYSRSTR::SYSTEM_RESET_DETECTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSRSTR {
        match value {
            false => SYSRSTR::NO_SYSTEM_RESET_DETE,
            true => SYSRSTR::SYSTEM_RESET_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYSTEM_RESET_DETE`"]
    #[inline]
    pub fn is_no_system_reset_dete(&self) -> bool {
        *self == SYSRSTR::NO_SYSTEM_RESET_DETE
    }
    #[doc = "Checks if the value of the field is `SYSTEM_RESET_DETECTE`"]
    #[inline]
    pub fn is_system_reset_detecte(&self) -> bool {
        *self == SYSRSTR::SYSTEM_RESET_DETECTE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - POR reset status."]
    #[inline]
    pub fn por(&self) -> PORR {
        PORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Status of the external RESET pin."]
    #[inline]
    pub fn extrst(&self) -> EXTRSTR {
        EXTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Status of the Watchdog reset."]
    #[inline]
    pub fn wdt(&self) -> WDTR {
        WDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset."]
    #[inline]
    pub fn bod(&self) -> BODR {
        BODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Status of the software system reset."]
    #[inline]
    pub fn sysrst(&self) -> SYSRSTR {
        SYSRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
