#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Burst mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_A {
    #[doc = "0: Software-controlled mode: Conversions are software-controlled and require 11 clocks"]
    SWMODE,
    #[doc = "1: Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start"]
    HWMODE,
}
impl From<BURST_A> for bool {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        match variant {
            BURST_A::SWMODE => false,
            BURST_A::HWMODE => true,
        }
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<bool, BURST_A>;
impl BURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            false => BURST_A::SWMODE,
            true => BURST_A::HWMODE,
        }
    }
    #[doc = "Checks if the value of the field is `SWMODE`"]
    #[inline(always)]
    pub fn is_swmode(&self) -> bool {
        *self == BURST_A::SWMODE
    }
    #[doc = "Checks if the value of the field is `HWMODE`"]
    #[inline(always)]
    pub fn is_hwmode(&self) -> bool {
        *self == BURST_A::HWMODE
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 11 clocks"]
    #[inline(always)]
    pub fn swmode(self) -> &'a mut W {
        self.variant(BURST_A::SWMODE)
    }
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start"]
    #[inline(always)]
    pub fn hwmode(self) -> &'a mut W {
        self.variant(BURST_A::HWMODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_A {
    #[doc = "0: 11 clocks / 10 bits"]
    TEN_BIT,
    #[doc = "1: 10 clocks / 9 bits"]
    NINE_BIT,
    #[doc = "2: 9 clocks / 8 bits"]
    EIGHT_BIT,
    #[doc = "3: 8 clocks / 7 bits"]
    SEVEN_BIT,
    #[doc = "4: 7 clocks / 6 bits"]
    SIX_BIT,
    #[doc = "5: 6 clocks / 5 bits"]
    FIVE_BIT,
    #[doc = "6: 5 clocks / 4 bits"]
    FOUR_BIT,
    #[doc = "7: 4 clocks / 3 bits"]
    THREE_BIT,
}
impl From<CLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        match variant {
            CLKS_A::TEN_BIT => 0,
            CLKS_A::NINE_BIT => 1,
            CLKS_A::EIGHT_BIT => 2,
            CLKS_A::SEVEN_BIT => 3,
            CLKS_A::SIX_BIT => 4,
            CLKS_A::FIVE_BIT => 5,
            CLKS_A::FOUR_BIT => 6,
            CLKS_A::THREE_BIT => 7,
        }
    }
}
#[doc = "Reader of field `CLKS`"]
pub type CLKS_R = crate::R<u8, CLKS_A>;
impl CLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            0 => CLKS_A::TEN_BIT,
            1 => CLKS_A::NINE_BIT,
            2 => CLKS_A::EIGHT_BIT,
            3 => CLKS_A::SEVEN_BIT,
            4 => CLKS_A::SIX_BIT,
            5 => CLKS_A::FIVE_BIT,
            6 => CLKS_A::FOUR_BIT,
            7 => CLKS_A::THREE_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TEN_BIT`"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == CLKS_A::TEN_BIT
    }
    #[doc = "Checks if the value of the field is `NINE_BIT`"]
    #[inline(always)]
    pub fn is_nine_bit(&self) -> bool {
        *self == CLKS_A::NINE_BIT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BIT`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == CLKS_A::EIGHT_BIT
    }
    #[doc = "Checks if the value of the field is `SEVEN_BIT`"]
    #[inline(always)]
    pub fn is_seven_bit(&self) -> bool {
        *self == CLKS_A::SEVEN_BIT
    }
    #[doc = "Checks if the value of the field is `SIX_BIT`"]
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == CLKS_A::SIX_BIT
    }
    #[doc = "Checks if the value of the field is `FIVE_BIT`"]
    #[inline(always)]
    pub fn is_five_bit(&self) -> bool {
        *self == CLKS_A::FIVE_BIT
    }
    #[doc = "Checks if the value of the field is `FOUR_BIT`"]
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        *self == CLKS_A::FOUR_BIT
    }
    #[doc = "Checks if the value of the field is `THREE_BIT`"]
    #[inline(always)]
    pub fn is_three_bit(&self) -> bool {
        *self == CLKS_A::THREE_BIT
    }
}
#[doc = "Write proxy for field `CLKS`"]
pub struct CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "11 clocks / 10 bits"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(CLKS_A::TEN_BIT)
    }
    #[doc = "10 clocks / 9 bits"]
    #[inline(always)]
    pub fn nine_bit(self) -> &'a mut W {
        self.variant(CLKS_A::NINE_BIT)
    }
    #[doc = "9 clocks / 8 bits"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(CLKS_A::EIGHT_BIT)
    }
    #[doc = "8 clocks / 7 bits"]
    #[inline(always)]
    pub fn seven_bit(self) -> &'a mut W {
        self.variant(CLKS_A::SEVEN_BIT)
    }
    #[doc = "7 clocks / 6 bits"]
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(CLKS_A::SIX_BIT)
    }
    #[doc = "6 clocks / 5 bits"]
    #[inline(always)]
    pub fn five_bit(self) -> &'a mut W {
        self.variant(CLKS_A::FIVE_BIT)
    }
    #[doc = "5 clocks / 4 bits"]
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut W {
        self.variant(CLKS_A::FOUR_BIT)
    }
    #[doc = "4 clocks / 3 bits"]
    #[inline(always)]
    pub fn three_bit(self) -> &'a mut W {
        self.variant(CLKS_A::THREE_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "When the BURST bit is 0, these bits control whether and when an A/D conversion is started.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: No start (this value should be used when clearing PDN to 0)"]
    STOP,
    #[doc = "1: Start conversion now"]
    START,
    #[doc = "2: Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0"]
    EDGE_ON_PIO0_2,
    #[doc = "3: Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0"]
    EDGE_ON_PIO1_5,
    #[doc = "4: Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0"]
    EDGE_ON_CT32B0_MAT0,
    #[doc = "5: Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1"]
    EDGE_ON_CT32B0_MAT1,
    #[doc = "6: Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0"]
    EDGE_ON_CT16B0_MAT0,
    #[doc = "7: Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1"]
    EDGE_ON_CT16B0_MAT1,
}
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        match variant {
            START_A::STOP => 0,
            START_A::START => 1,
            START_A::EDGE_ON_PIO0_2 => 2,
            START_A::EDGE_ON_PIO1_5 => 3,
            START_A::EDGE_ON_CT32B0_MAT0 => 4,
            START_A::EDGE_ON_CT32B0_MAT1 => 5,
            START_A::EDGE_ON_CT16B0_MAT0 => 6,
            START_A::EDGE_ON_CT16B0_MAT1 => 7,
        }
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<u8, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            0 => START_A::STOP,
            1 => START_A::START,
            2 => START_A::EDGE_ON_PIO0_2,
            3 => START_A::EDGE_ON_PIO1_5,
            4 => START_A::EDGE_ON_CT32B0_MAT0,
            5 => START_A::EDGE_ON_CT32B0_MAT1,
            6 => START_A::EDGE_ON_CT16B0_MAT0,
            7 => START_A::EDGE_ON_CT16B0_MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == START_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_PIO0_2`"]
    #[inline(always)]
    pub fn is_edge_on_pio0_2(&self) -> bool {
        *self == START_A::EDGE_ON_PIO0_2
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_PIO1_5`"]
    #[inline(always)]
    pub fn is_edge_on_pio1_5(&self) -> bool {
        *self == START_A::EDGE_ON_PIO1_5
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT32B0_MAT0`"]
    #[inline(always)]
    pub fn is_edge_on_ct32b0_mat0(&self) -> bool {
        *self == START_A::EDGE_ON_CT32B0_MAT0
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT32B0_MAT1`"]
    #[inline(always)]
    pub fn is_edge_on_ct32b0_mat1(&self) -> bool {
        *self == START_A::EDGE_ON_CT32B0_MAT1
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT16B0_MAT0`"]
    #[inline(always)]
    pub fn is_edge_on_ct16b0_mat0(&self) -> bool {
        *self == START_A::EDGE_ON_CT16B0_MAT0
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT16B0_MAT1`"]
    #[inline(always)]
    pub fn is_edge_on_ct16b0_mat1(&self) -> bool {
        *self == START_A::EDGE_ON_CT16B0_MAT1
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(START_A::STOP)
    }
    #[doc = "Start conversion now"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0"]
    #[inline(always)]
    pub fn edge_on_pio0_2(self) -> &'a mut W {
        self.variant(START_A::EDGE_ON_PIO0_2)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0"]
    #[inline(always)]
    pub fn edge_on_pio1_5(self) -> &'a mut W {
        self.variant(START_A::EDGE_ON_PIO1_5)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0"]
    #[inline(always)]
    pub fn edge_on_ct32b0_mat0(self) -> &'a mut W {
        self.variant(START_A::EDGE_ON_CT32B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1"]
    #[inline(always)]
    pub fn edge_on_ct32b0_mat1(self) -> &'a mut W {
        self.variant(START_A::EDGE_ON_CT32B0_MAT1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0"]
    #[inline(always)]
    pub fn edge_on_ct16b0_mat0(self) -> &'a mut W {
        self.variant(START_A::EDGE_ON_CT16B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1"]
    #[inline(always)]
    pub fn edge_on_ct16b0_mat1(self) -> &'a mut W {
        self.variant(START_A::EDGE_ON_CT16B0_MAT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE_A {
    #[doc = "0: Start conversion on a rising edge on the selected CAP/MAT signal"]
    RISING,
    #[doc = "1: Start conversion on a rising edge on the selected CAP/MAT signal"]
    FALLING,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        match variant {
            EDGE_A::RISING => false,
            EDGE_A::FALLING => true,
        }
    }
}
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, EDGE_A>;
impl EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::RISING,
            true => EDGE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EDGE_A::FALLING
    }
}
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGE_A::RISING)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGE_A::FALLING)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst mode."]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 16 - Burst mode."]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline(always)]
    pub fn clks(&mut self) -> CLKS_W {
        CLKS_W { w: self }
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
}
