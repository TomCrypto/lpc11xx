#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SELR {
    bits: u8,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKDIVR {
    bits: u8,
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTR {
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 11 clocks"]
    SWMODE,
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start"]
    HWMODE,
}
impl BURSTR {
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
            BURSTR::SWMODE => false,
            BURSTR::HWMODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTR {
        match value {
            false => BURSTR::SWMODE,
            true => BURSTR::HWMODE,
        }
    }
    #[doc = "Checks if the value of the field is `SWMODE`"]
    #[inline]
    pub fn is_swmode(&self) -> bool {
        *self == BURSTR::SWMODE
    }
    #[doc = "Checks if the value of the field is `HWMODE`"]
    #[inline]
    pub fn is_hwmode(&self) -> bool {
        *self == BURSTR::HWMODE
    }
}
#[doc = "Possible values of the field `CLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSR {
    #[doc = "11 clocks / 10 bits"]
    TEN_BIT,
    #[doc = "10 clocks / 9 bits"]
    NINE_BIT,
    #[doc = "9 clocks / 8 bits"]
    EIGHT_BIT,
    #[doc = "8 clocks / 7 bits"]
    SEVEN_BIT,
    #[doc = "7 clocks / 6 bits"]
    SIX_BIT,
    #[doc = "6 clocks / 5 bits"]
    FIVE_BIT,
    #[doc = "5 clocks / 4 bits"]
    FOUR_BIT,
    #[doc = "4 clocks / 3 bits"]
    THREE_BIT,
}
impl CLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSR::TEN_BIT => 0,
            CLKSR::NINE_BIT => 1,
            CLKSR::EIGHT_BIT => 2,
            CLKSR::SEVEN_BIT => 3,
            CLKSR::SIX_BIT => 4,
            CLKSR::FIVE_BIT => 5,
            CLKSR::FOUR_BIT => 6,
            CLKSR::THREE_BIT => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSR {
        match value {
            0 => CLKSR::TEN_BIT,
            1 => CLKSR::NINE_BIT,
            2 => CLKSR::EIGHT_BIT,
            3 => CLKSR::SEVEN_BIT,
            4 => CLKSR::SIX_BIT,
            5 => CLKSR::FIVE_BIT,
            6 => CLKSR::FOUR_BIT,
            7 => CLKSR::THREE_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TEN_BIT`"]
    #[inline]
    pub fn is_ten_bit(&self) -> bool {
        *self == CLKSR::TEN_BIT
    }
    #[doc = "Checks if the value of the field is `NINE_BIT`"]
    #[inline]
    pub fn is_nine_bit(&self) -> bool {
        *self == CLKSR::NINE_BIT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BIT`"]
    #[inline]
    pub fn is_eight_bit(&self) -> bool {
        *self == CLKSR::EIGHT_BIT
    }
    #[doc = "Checks if the value of the field is `SEVEN_BIT`"]
    #[inline]
    pub fn is_seven_bit(&self) -> bool {
        *self == CLKSR::SEVEN_BIT
    }
    #[doc = "Checks if the value of the field is `SIX_BIT`"]
    #[inline]
    pub fn is_six_bit(&self) -> bool {
        *self == CLKSR::SIX_BIT
    }
    #[doc = "Checks if the value of the field is `FIVE_BIT`"]
    #[inline]
    pub fn is_five_bit(&self) -> bool {
        *self == CLKSR::FIVE_BIT
    }
    #[doc = "Checks if the value of the field is `FOUR_BIT`"]
    #[inline]
    pub fn is_four_bit(&self) -> bool {
        *self == CLKSR::FOUR_BIT
    }
    #[doc = "Checks if the value of the field is `THREE_BIT`"]
    #[inline]
    pub fn is_three_bit(&self) -> bool {
        *self == CLKSR::THREE_BIT
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "No start (this value should be used when clearing PDN to 0)"]
    STOP,
    #[doc = "Start conversion now"]
    START,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0"]
    EDGE_ON_PIO0_2,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0"]
    EDGE_ON_PIO1_5,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0"]
    EDGE_ON_CT32B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1"]
    EDGE_ON_CT32B0_MAT1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0"]
    EDGE_ON_CT16B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1"]
    EDGE_ON_CT16B0_MAT1,
}
impl STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTR::STOP => 0,
            STARTR::START => 1,
            STARTR::EDGE_ON_PIO0_2 => 2,
            STARTR::EDGE_ON_PIO1_5 => 3,
            STARTR::EDGE_ON_CT32B0_MAT0 => 4,
            STARTR::EDGE_ON_CT32B0_MAT1 => 5,
            STARTR::EDGE_ON_CT16B0_MAT0 => 6,
            STARTR::EDGE_ON_CT16B0_MAT1 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTR {
        match value {
            0 => STARTR::STOP,
            1 => STARTR::START,
            2 => STARTR::EDGE_ON_PIO0_2,
            3 => STARTR::EDGE_ON_PIO1_5,
            4 => STARTR::EDGE_ON_CT32B0_MAT0,
            5 => STARTR::EDGE_ON_CT32B0_MAT1,
            6 => STARTR::EDGE_ON_CT16B0_MAT0,
            7 => STARTR::EDGE_ON_CT16B0_MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STARTR::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == STARTR::START
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_PIO0_2`"]
    #[inline]
    pub fn is_edge_on_pio0_2(&self) -> bool {
        *self == STARTR::EDGE_ON_PIO0_2
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_PIO1_5`"]
    #[inline]
    pub fn is_edge_on_pio1_5(&self) -> bool {
        *self == STARTR::EDGE_ON_PIO1_5
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT32B0_MAT0`"]
    #[inline]
    pub fn is_edge_on_ct32b0_mat0(&self) -> bool {
        *self == STARTR::EDGE_ON_CT32B0_MAT0
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT32B0_MAT1`"]
    #[inline]
    pub fn is_edge_on_ct32b0_mat1(&self) -> bool {
        *self == STARTR::EDGE_ON_CT32B0_MAT1
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT16B0_MAT0`"]
    #[inline]
    pub fn is_edge_on_ct16b0_mat0(&self) -> bool {
        *self == STARTR::EDGE_ON_CT16B0_MAT0
    }
    #[doc = "Checks if the value of the field is `EDGE_ON_CT16B0_MAT1`"]
    #[inline]
    pub fn is_edge_on_ct16b0_mat1(&self) -> bool {
        *self == STARTR::EDGE_ON_CT16B0_MAT1
    }
}
#[doc = "Possible values of the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGER {
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    RISING,
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    FALLING,
}
impl EDGER {
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
            EDGER::RISING => false,
            EDGER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGER {
        match value {
            false => EDGER::RISING,
            true => EDGER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == EDGER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == EDGER::FALLING
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURST`"]
pub enum BURSTW {
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 11 clocks"]
    SWMODE,
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start"]
    HWMODE,
}
impl BURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTW::SWMODE => false,
            BURSTW::HWMODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 11 clocks"]
    #[inline]
    pub fn swmode(self) -> &'a mut W {
        self.variant(BURSTW::SWMODE)
    }
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start"]
    #[inline]
    pub fn hwmode(self) -> &'a mut W {
        self.variant(BURSTW::HWMODE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKS`"]
pub enum CLKSW {
    #[doc = "11 clocks / 10 bits"]
    TEN_BIT,
    #[doc = "10 clocks / 9 bits"]
    NINE_BIT,
    #[doc = "9 clocks / 8 bits"]
    EIGHT_BIT,
    #[doc = "8 clocks / 7 bits"]
    SEVEN_BIT,
    #[doc = "7 clocks / 6 bits"]
    SIX_BIT,
    #[doc = "6 clocks / 5 bits"]
    FIVE_BIT,
    #[doc = "5 clocks / 4 bits"]
    FOUR_BIT,
    #[doc = "4 clocks / 3 bits"]
    THREE_BIT,
}
impl CLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSW::TEN_BIT => 0,
            CLKSW::NINE_BIT => 1,
            CLKSW::EIGHT_BIT => 2,
            CLKSW::SEVEN_BIT => 3,
            CLKSW::SIX_BIT => 4,
            CLKSW::FIVE_BIT => 5,
            CLKSW::FOUR_BIT => 6,
            CLKSW::THREE_BIT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "11 clocks / 10 bits"]
    #[inline]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(CLKSW::TEN_BIT)
    }
    #[doc = "10 clocks / 9 bits"]
    #[inline]
    pub fn nine_bit(self) -> &'a mut W {
        self.variant(CLKSW::NINE_BIT)
    }
    #[doc = "9 clocks / 8 bits"]
    #[inline]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(CLKSW::EIGHT_BIT)
    }
    #[doc = "8 clocks / 7 bits"]
    #[inline]
    pub fn seven_bit(self) -> &'a mut W {
        self.variant(CLKSW::SEVEN_BIT)
    }
    #[doc = "7 clocks / 6 bits"]
    #[inline]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(CLKSW::SIX_BIT)
    }
    #[doc = "6 clocks / 5 bits"]
    #[inline]
    pub fn five_bit(self) -> &'a mut W {
        self.variant(CLKSW::FIVE_BIT)
    }
    #[doc = "5 clocks / 4 bits"]
    #[inline]
    pub fn four_bit(self) -> &'a mut W {
        self.variant(CLKSW::FOUR_BIT)
    }
    #[doc = "4 clocks / 3 bits"]
    #[inline]
    pub fn three_bit(self) -> &'a mut W {
        self.variant(CLKSW::THREE_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "No start (this value should be used when clearing PDN to 0)"]
    STOP,
    #[doc = "Start conversion now"]
    START,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0"]
    EDGE_ON_PIO0_2,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0"]
    EDGE_ON_PIO1_5,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0"]
    EDGE_ON_CT32B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1"]
    EDGE_ON_CT32B0_MAT1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0"]
    EDGE_ON_CT16B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1"]
    EDGE_ON_CT16B0_MAT1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTW::STOP => 0,
            STARTW::START => 1,
            STARTW::EDGE_ON_PIO0_2 => 2,
            STARTW::EDGE_ON_PIO1_5 => 3,
            STARTW::EDGE_ON_CT32B0_MAT0 => 4,
            STARTW::EDGE_ON_CT32B0_MAT1 => 5,
            STARTW::EDGE_ON_CT16B0_MAT0 => 6,
            STARTW::EDGE_ON_CT16B0_MAT1 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STARTW::STOP)
    }
    #[doc = "Start conversion now"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW::START)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0"]
    #[inline]
    pub fn edge_on_pio0_2(self) -> &'a mut W {
        self.variant(STARTW::EDGE_ON_PIO0_2)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0"]
    #[inline]
    pub fn edge_on_pio1_5(self) -> &'a mut W {
        self.variant(STARTW::EDGE_ON_PIO1_5)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0"]
    #[inline]
    pub fn edge_on_ct32b0_mat0(self) -> &'a mut W {
        self.variant(STARTW::EDGE_ON_CT32B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1"]
    #[inline]
    pub fn edge_on_ct32b0_mat1(self) -> &'a mut W {
        self.variant(STARTW::EDGE_ON_CT32B0_MAT1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0"]
    #[inline]
    pub fn edge_on_ct16b0_mat0(self) -> &'a mut W {
        self.variant(STARTW::EDGE_ON_CT16B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1"]
    #[inline]
    pub fn edge_on_ct16b0_mat1(self) -> &'a mut W {
        self.variant(STARTW::EDGE_ON_CT16B0_MAT1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE`"]
pub enum EDGEW {
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    RISING,
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    FALLING,
}
impl EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGEW::RISING => false,
            EDGEW::FALLING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGEW::RISING)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGEW::FALLING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline]
    pub fn sel(&self) -> SELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SELR { bits }
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKDIVR { bits }
    }
    #[doc = "Bit 16 - Burst mode."]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        BURSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline]
    pub fn clks(&self) -> CLKSR {
        CLKSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started."]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline]
    pub fn edge(&self) -> EDGER {
        EDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 16 - Burst mode."]
    #[inline]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline]
    pub fn clks(&mut self) -> _CLKSW {
        _CLKSW { w: self }
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started."]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases: Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
}
