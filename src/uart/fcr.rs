#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCR {
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
}
#[doc = "Values that can be written to the field `FIFOEN`"]
pub enum FIFOENW {
    #[doc = "UART FIFOs are disabled. Must not be used in the application"]
    DISABLE,
    #[doc = "Active high enable for both UART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the UART FIFOs"]
    ENABLE,
}
impl FIFOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFOENW::DISABLE => false,
            FIFOENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFOENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UART FIFOs are disabled. Must not be used in the application"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIFOENW::DISABLE)
    }
    #[doc = "Active high enable for both UART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the UART FIFOs"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIFOENW::ENABLE)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXFIFORES`"]
pub enum RXFIFORESW {
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in UART Rx FIFO, reset the pointer logic. This bit is self-clearing"]
    CLEAR,
}
impl RXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFORESW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFORESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in UART Rx FIFO, reset the pointer logic. This bit is self-clearing"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFORESW::CLEAR)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXFIFORES`"]
pub enum TXFIFORESW {
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in UART TX FIFO, reset the pointer logic. This bit is self-clearing"]
    CLEAR,
}
impl TXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFIFORESW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFIFORESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in UART TX FIFO, reset the pointer logic. This bit is self-clearing"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFORESW::CLEAR)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXTL`"]
pub enum RXTLW {
    #[doc = "Trigger level 0 (1 character)"]
    ONE_WORD,
    #[doc = "Trigger level 1 (4 characters)"]
    FOUR_WORDS,
    #[doc = "Trigger level 2 (8 characters)"]
    EIGHT_WORDS,
    #[doc = "Trigger level 3 (14 characters)"]
    FOURTEEN_WORDS,
}
impl RXTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTLW::ONE_WORD => 0,
            RXTLW::FOUR_WORDS => 1,
            RXTLW::EIGHT_WORDS => 2,
            RXTLW::FOURTEEN_WORDS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger level 0 (1 character)"]
    #[inline]
    pub fn one_word(self) -> &'a mut W {
        self.variant(RXTLW::ONE_WORD)
    }
    #[doc = "Trigger level 1 (4 characters)"]
    #[inline]
    pub fn four_words(self) -> &'a mut W {
        self.variant(RXTLW::FOUR_WORDS)
    }
    #[doc = "Trigger level 2 (8 characters)"]
    #[inline]
    pub fn eight_words(self) -> &'a mut W {
        self.variant(RXTLW::EIGHT_WORDS)
    }
    #[doc = "Trigger level 3 (14 characters)"]
    #[inline]
    pub fn fourteen_words(self) -> &'a mut W {
        self.variant(RXTLW::FOURTEEN_WORDS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - FIFO Enable."]
    #[inline]
    pub fn fifoen(&mut self) -> _FIFOENW {
        _FIFOENW { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline]
    pub fn rxfifores(&mut self) -> _RXFIFORESW {
        _RXFIFORESW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline]
    pub fn txfifores(&mut self) -> _TXFIFORESW {
        _TXFIFORESW { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART FIFO characters must be written before an interrupt is activated."]
    #[inline]
    pub fn rxtl(&mut self) -> _RXTLW {
        _RXTLW { w: self }
    }
}
