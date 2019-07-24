#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTAPRP0 {
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
pub struct APRPIO0_0R {
    bits: bool,
}
impl APRPIO0_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_1R {
    bits: bool,
}
impl APRPIO0_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_2R {
    bits: bool,
}
impl APRPIO0_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_3R {
    bits: bool,
}
impl APRPIO0_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_4R {
    bits: bool,
}
impl APRPIO0_4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_5R {
    bits: bool,
}
impl APRPIO0_5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_6R {
    bits: bool,
}
impl APRPIO0_6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_7R {
    bits: bool,
}
impl APRPIO0_7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_8R {
    bits: bool,
}
impl APRPIO0_8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_9R {
    bits: bool,
}
impl APRPIO0_9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_10R {
    bits: bool,
}
impl APRPIO0_10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO0_11R {
    bits: bool,
}
impl APRPIO0_11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct APRPIO1_0R {
    bits: bool,
}
impl APRPIO1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _APRPIO0_0W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_0W<'a> {
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
#[doc = r" Proxy"]
pub struct _APRPIO0_1W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_1W<'a> {
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
#[doc = r" Proxy"]
pub struct _APRPIO0_2W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_2W<'a> {
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
#[doc = r" Proxy"]
pub struct _APRPIO0_3W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_3W<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_4W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_4W<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_5W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_5W<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_6W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_6W<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_7W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_7W<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_8W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_8W<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_9W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_9W<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_10W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_10W<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO0_11W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO0_11W<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRPIO1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _APRPIO1_0W<'a> {
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_0(&self) -> APRPIO0_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_0R { bits }
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_1(&self) -> APRPIO0_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_1R { bits }
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_2(&self) -> APRPIO0_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_2R { bits }
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_3(&self) -> APRPIO0_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_3R { bits }
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_4(&self) -> APRPIO0_4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_4R { bits }
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_5(&self) -> APRPIO0_5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_5R { bits }
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_6(&self) -> APRPIO0_6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_6R { bits }
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_7(&self) -> APRPIO0_7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_7R { bits }
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_8(&self) -> APRPIO0_8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_8R { bits }
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_9(&self) -> APRPIO0_9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_9R { bits }
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_10(&self) -> APRPIO0_10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_10R { bits }
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_11(&self) -> APRPIO0_11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO0_11R { bits }
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio1_0(&self) -> APRPIO1_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APRPIO1_0R { bits }
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
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_0(&mut self) -> _APRPIO0_0W {
        _APRPIO0_0W { w: self }
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_1(&mut self) -> _APRPIO0_1W {
        _APRPIO0_1W { w: self }
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_2(&mut self) -> _APRPIO0_2W {
        _APRPIO0_2W { w: self }
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_3(&mut self) -> _APRPIO0_3W {
        _APRPIO0_3W { w: self }
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_4(&mut self) -> _APRPIO0_4W {
        _APRPIO0_4W { w: self }
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_5(&mut self) -> _APRPIO0_5W {
        _APRPIO0_5W { w: self }
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_6(&mut self) -> _APRPIO0_6W {
        _APRPIO0_6W { w: self }
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_7(&mut self) -> _APRPIO0_7W {
        _APRPIO0_7W { w: self }
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_8(&mut self) -> _APRPIO0_8W {
        _APRPIO0_8W { w: self }
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_9(&mut self) -> _APRPIO0_9W {
        _APRPIO0_9W { w: self }
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_10(&mut self) -> _APRPIO0_10W {
        _APRPIO0_10W { w: self }
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n: PIO0_11 to PIO0_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio0_11(&mut self) -> _APRPIO0_11W {
        _APRPIO0_11W { w: self }
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_0 0 = Falling edge 1 = Rising edge"]
    #[inline]
    pub fn aprpio1_0(&mut self) -> _APRPIO1_0W {
        _APRPIO1_0W { w: self }
    }
}
