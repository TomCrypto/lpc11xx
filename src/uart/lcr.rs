#[doc = "Reader of register LCR"]
pub type R = crate::R<u32, super::LCR>;
#[doc = "Writer for register LCR"]
pub type W = crate::W<u32, super::LCR>;
#[doc = "Register LCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Word Length Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLS_A {
    #[doc = "0: 5-bit character length"]
    FIVE,
    #[doc = "1: 6-bit character length"]
    SIX,
    #[doc = "2: 7-bit character length"]
    SEVEN,
    #[doc = "3: 8-bit character length"]
    EIGHT,
}
impl From<WLS_A> for u8 {
    #[inline(always)]
    fn from(variant: WLS_A) -> Self {
        match variant {
            WLS_A::FIVE => 0,
            WLS_A::SIX => 1,
            WLS_A::SEVEN => 2,
            WLS_A::EIGHT => 3,
        }
    }
}
#[doc = "Reader of field `WLS`"]
pub type WLS_R = crate::R<u8, WLS_A>;
impl WLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLS_A {
        match self.bits {
            0 => WLS_A::FIVE,
            1 => WLS_A::SIX,
            2 => WLS_A::SEVEN,
            3 => WLS_A::EIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == WLS_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == WLS_A::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == WLS_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == WLS_A::EIGHT
    }
}
#[doc = "Write proxy for field `WLS`"]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "5-bit character length"]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(WLS_A::FIVE)
    }
    #[doc = "6-bit character length"]
    #[inline(always)]
    pub fn six(self) -> &'a mut W {
        self.variant(WLS_A::SIX)
    }
    #[doc = "7-bit character length"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(WLS_A::SEVEN)
    }
    #[doc = "8-bit character length"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(WLS_A::EIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Stop Bit Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBS_A {
    #[doc = "0: 1 stop bit"]
    ONE,
    #[doc = "1: 2 stop bits, or 1.5 if using 5-bit words"]
    TWO,
}
impl From<SBS_A> for bool {
    #[inline(always)]
    fn from(variant: SBS_A) -> Self {
        match variant {
            SBS_A::ONE => false,
            SBS_A::TWO => true,
        }
    }
}
#[doc = "Reader of field `SBS`"]
pub type SBS_R = crate::R<bool, SBS_A>;
impl SBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBS_A {
        match self.bits {
            false => SBS_A::ONE,
            true => SBS_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SBS_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SBS_A::TWO
    }
}
#[doc = "Write proxy for field `SBS`"]
pub struct SBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SBS_A::ONE)
    }
    #[doc = "2 stop bits, or 1.5 if using 5-bit words"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(SBS_A::TWO)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Parity Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Disable parity generation and checking"]
    DISABLE,
    #[doc = "1: Enable parity generation and checking"]
    ENABLE,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        match variant {
            PE_A::DISABLE => false,
            PE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLE,
            true => PE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PE_A::ENABLE
    }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable parity generation and checking"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PE_A::DISABLE)
    }
    #[doc = "Enable parity generation and checking"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Parity Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd"]
    ODD,
    #[doc = "1: Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even"]
    EVEN,
    #[doc = "2: Forced 1 stick parity"]
    FORCED_1_STICK,
    #[doc = "3: Forced 0 stick parity"]
    FORCED_0_STICK,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        match variant {
            PS_A::ODD => 0,
            PS_A::EVEN => 1,
            PS_A::FORCED_1_STICK => 2,
            PS_A::FORCED_0_STICK => 3,
        }
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::ODD,
            1 => PS_A::EVEN,
            2 => PS_A::FORCED_1_STICK,
            3 => PS_A::FORCED_0_STICK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS_A::EVEN
    }
    #[doc = "Checks if the value of the field is `FORCED_1_STICK`"]
    #[inline(always)]
    pub fn is_forced_1_stick(&self) -> bool {
        *self == PS_A::FORCED_1_STICK
    }
    #[doc = "Checks if the value of the field is `FORCED_0_STICK`"]
    #[inline(always)]
    pub fn is_forced_0_stick(&self) -> bool {
        *self == PS_A::FORCED_0_STICK
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PS_A::ODD)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PS_A::EVEN)
    }
    #[doc = "Forced 1 stick parity"]
    #[inline(always)]
    pub fn forced_1_stick(self) -> &'a mut W {
        self.variant(PS_A::FORCED_1_STICK)
    }
    #[doc = "Forced 0 stick parity"]
    #[inline(always)]
    pub fn forced_0_stick(self) -> &'a mut W {
        self.variant(PS_A::FORCED_0_STICK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Break Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BC_A {
    #[doc = "0: Disable break transmission"]
    DISABLE,
    #[doc = "1: Enable break transmission. Output pin UART TXD is forced to logic 0 when LCR\\[6\\] is active high"]
    ENABLE,
}
impl From<BC_A> for bool {
    #[inline(always)]
    fn from(variant: BC_A) -> Self {
        match variant {
            BC_A::DISABLE => false,
            BC_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `BC`"]
pub type BC_R = crate::R<bool, BC_A>;
impl BC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC_A {
        match self.bits {
            false => BC_A::DISABLE,
            true => BC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BC_A::ENABLE
    }
}
#[doc = "Write proxy for field `BC`"]
pub struct BC_W<'a> {
    w: &'a mut W,
}
impl<'a> BC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable break transmission"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BC_A::DISABLE)
    }
    #[doc = "Enable break transmission. Output pin UART TXD is forced to logic 0 when LCR\\[6\\] is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BC_A::ENABLE)
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
#[doc = "Divisor Latch Access Bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLAB_A {
    #[doc = "0: Disable access to Divisor Latches"]
    DISABLE,
    #[doc = "1: Enable access to Divisor Latches"]
    ENABLE,
}
impl From<DLAB_A> for bool {
    #[inline(always)]
    fn from(variant: DLAB_A) -> Self {
        match variant {
            DLAB_A::DISABLE => false,
            DLAB_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `DLAB`"]
pub type DLAB_R = crate::R<bool, DLAB_A>;
impl DLAB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLAB_A {
        match self.bits {
            false => DLAB_A::DISABLE,
            true => DLAB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DLAB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DLAB_A::ENABLE
    }
}
#[doc = "Write proxy for field `DLAB`"]
pub struct DLAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLAB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable access to Divisor Latches"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DLAB_A::DISABLE)
    }
    #[doc = "Enable access to Divisor Latches"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DLAB_A::ENABLE)
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
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit."]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&mut self) -> SBS_W {
        SBS_W { w: self }
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W {
        BC_W { w: self }
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit."]
    #[inline(always)]
    pub fn dlab(&mut self) -> DLAB_W {
        DLAB_W { w: self }
    }
}
