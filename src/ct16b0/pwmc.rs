#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWMC {
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
#[doc = "Possible values of the field `PWMEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0R {
    #[doc = "CT16Bn_MAT0 is controlled by EM0"]
    CT16BN_MAT0_IS_CONTR,
    #[doc = "PWM mode is enabled for CT16Bn_MAT0"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN0R {
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
            PWMEN0R::CT16BN_MAT0_IS_CONTR => false,
            PWMEN0R::PWM_MODE_IS_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN0R {
        match value {
            false => PWMEN0R::CT16BN_MAT0_IS_CONTR,
            true => PWMEN0R::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CT16BN_MAT0_IS_CONTR`"]
    #[inline]
    pub fn is_ct16bn_mat0_is_contr(&self) -> bool {
        *self == PWMEN0R::CT16BN_MAT0_IS_CONTR
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN0R::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Possible values of the field `PWMEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1R {
    #[doc = "CT16Bn_MAT1 is controlled by EM1"]
    CT16BN_MAT1_IS_CONTR,
    #[doc = "PWM mode is enabled for CT16Bn_MAT1"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN1R {
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
            PWMEN1R::CT16BN_MAT1_IS_CONTR => false,
            PWMEN1R::PWM_MODE_IS_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN1R {
        match value {
            false => PWMEN1R::CT16BN_MAT1_IS_CONTR,
            true => PWMEN1R::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CT16BN_MAT1_IS_CONTR`"]
    #[inline]
    pub fn is_ct16bn_mat1_is_contr(&self) -> bool {
        *self == PWMEN1R::CT16BN_MAT1_IS_CONTR
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN1R::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Possible values of the field `PWMEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2R {
    #[doc = "Match channel 2 or pin CT16B0_MAT2 is controlled by EM2. Match channel 2 is not pinned out on timer 1"]
    MATCH_CHANNEL_2_OR_P,
    #[doc = "PWM mode is enabled for match channel 2 or pin CT16B0_MAT2"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN2R {
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
            PWMEN2R::MATCH_CHANNEL_2_OR_P => false,
            PWMEN2R::PWM_MODE_IS_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN2R {
        match value {
            false => PWMEN2R::MATCH_CHANNEL_2_OR_P,
            true => PWMEN2R::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH_CHANNEL_2_OR_P`"]
    #[inline]
    pub fn is_match_channel_2_or_p(&self) -> bool {
        *self == PWMEN2R::MATCH_CHANNEL_2_OR_P
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN2R::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Possible values of the field `PWMEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3R {
    #[doc = "Match channel 3 match channel 3 is controlled by EM3"]
    MATCH_CHANNEL_3_MATC,
    #[doc = "PWM mode is enabled for match channel 3match channel 3"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN3R {
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
            PWMEN3R::MATCH_CHANNEL_3_MATC => false,
            PWMEN3R::PWM_MODE_IS_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN3R {
        match value {
            false => PWMEN3R::MATCH_CHANNEL_3_MATC,
            true => PWMEN3R::PWM_MODE_IS_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH_CHANNEL_3_MATC`"]
    #[inline]
    pub fn is_match_channel_3_matc(&self) -> bool {
        *self == PWMEN3R::MATCH_CHANNEL_3_MATC
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN3R::PWM_MODE_IS_ENABLED_
    }
}
#[doc = "Values that can be written to the field `PWMEN0`"]
pub enum PWMEN0W {
    #[doc = "CT16Bn_MAT0 is controlled by EM0"]
    CT16BN_MAT0_IS_CONTR,
    #[doc = "PWM mode is enabled for CT16Bn_MAT0"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN0W::CT16BN_MAT0_IS_CONTR => false,
            PWMEN0W::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CT16Bn_MAT0 is controlled by EM0"]
    #[inline]
    pub fn ct16bn_mat0_is_contr(self) -> &'a mut W {
        self.variant(PWMEN0W::CT16BN_MAT0_IS_CONTR)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT0"]
    #[inline]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN0W::PWM_MODE_IS_ENABLED_)
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
#[doc = "Values that can be written to the field `PWMEN1`"]
pub enum PWMEN1W {
    #[doc = "CT16Bn_MAT1 is controlled by EM1"]
    CT16BN_MAT1_IS_CONTR,
    #[doc = "PWM mode is enabled for CT16Bn_MAT1"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN1W::CT16BN_MAT1_IS_CONTR => false,
            PWMEN1W::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CT16Bn_MAT1 is controlled by EM1"]
    #[inline]
    pub fn ct16bn_mat1_is_contr(self) -> &'a mut W {
        self.variant(PWMEN1W::CT16BN_MAT1_IS_CONTR)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT1"]
    #[inline]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN1W::PWM_MODE_IS_ENABLED_)
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
#[doc = "Values that can be written to the field `PWMEN2`"]
pub enum PWMEN2W {
    #[doc = "Match channel 2 or pin CT16B0_MAT2 is controlled by EM2. Match channel 2 is not pinned out on timer 1"]
    MATCH_CHANNEL_2_OR_P,
    #[doc = "PWM mode is enabled for match channel 2 or pin CT16B0_MAT2"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN2W::MATCH_CHANNEL_2_OR_P => false,
            PWMEN2W::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Match channel 2 or pin CT16B0_MAT2 is controlled by EM2. Match channel 2 is not pinned out on timer 1"]
    #[inline]
    pub fn match_channel_2_or_p(self) -> &'a mut W {
        self.variant(PWMEN2W::MATCH_CHANNEL_2_OR_P)
    }
    #[doc = "PWM mode is enabled for match channel 2 or pin CT16B0_MAT2"]
    #[inline]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN2W::PWM_MODE_IS_ENABLED_)
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
#[doc = "Values that can be written to the field `PWMEN3`"]
pub enum PWMEN3W {
    #[doc = "Match channel 3 match channel 3 is controlled by EM3"]
    MATCH_CHANNEL_3_MATC,
    #[doc = "PWM mode is enabled for match channel 3match channel 3"]
    PWM_MODE_IS_ENABLED_,
}
impl PWMEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN3W::MATCH_CHANNEL_3_MATC => false,
            PWMEN3W::PWM_MODE_IS_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Match channel 3 match channel 3 is controlled by EM3"]
    #[inline]
    pub fn match_channel_3_matc(self) -> &'a mut W {
        self.variant(PWMEN3W::MATCH_CHANNEL_3_MATC)
    }
    #[doc = "PWM mode is enabled for match channel 3match channel 3"]
    #[inline]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN3W::PWM_MODE_IS_ENABLED_)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - PWM channel0 enable"]
    #[inline]
    pub fn pwmen0(&self) -> PWMEN0R {
        PWMEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PWM channel1 enable"]
    #[inline]
    pub fn pwmen1(&self) -> PWMEN1R {
        PWMEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PWM channel2 enable"]
    #[inline]
    pub fn pwmen2(&self) -> PWMEN2R {
        PWMEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PWM channel3 enable Note: It is recommended to use match channel 3 to set the PWM cycle because it is not pinned out"]
    #[inline]
    pub fn pwmen3(&self) -> PWMEN3R {
        PWMEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - PWM channel0 enable"]
    #[inline]
    pub fn pwmen0(&mut self) -> _PWMEN0W {
        _PWMEN0W { w: self }
    }
    #[doc = "Bit 1 - PWM channel1 enable"]
    #[inline]
    pub fn pwmen1(&mut self) -> _PWMEN1W {
        _PWMEN1W { w: self }
    }
    #[doc = "Bit 2 - PWM channel2 enable"]
    #[inline]
    pub fn pwmen2(&mut self) -> _PWMEN2W {
        _PWMEN2W { w: self }
    }
    #[doc = "Bit 3 - PWM channel3 enable Note: It is recommended to use match channel 3 to set the PWM cycle because it is not pinned out"]
    #[inline]
    pub fn pwmen3(&mut self) -> _PWMEN3W {
        _PWMEN3W { w: self }
    }
}
