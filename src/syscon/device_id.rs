#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVICE_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DEVICEIDR {
    bits: u32,
}
impl DEVICEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Part ID numbers for LPC111x/LPC11Cxx parts 0x041E 502B; 0x2516 D02B = LPC1111FHN33/101 0x2516 D02B = LPC1111FHN33/102 0x0416 502B; 0x2516 902B = LPC1111FHN33/201 0x2516 902B = LPC1111FHN33/202 0x042D 502B; 0x2524 D02B = LPC1112FHN33/101 0x2524 D02B = LPC1112FHN33/102 0x0425 502B; 0x2524 902B = LPC1112FHN33/201 0x2524 902B = LPC1112FHN33/202 0x2524 902B = LPC1112FHI33/202 0x0434 502B; 0x2532 902B = LPC1113FHN33/201 0x2532 902B = LPC1113FHN33/202 0x0434 102B; 0x2532 102B = LPC1113FHN33/301 0x2532 102B = LPC1113FHN33/302 0x0434 102B; 0x2532 102B = LPC1113FBD48/301 0x2532 102B = LPC1113FBD48/302 0x0444 502B; 0x2540 902B = LPC1114FHN33/201 0x2540 902B = LPC1114FHN33/202 0x0444 102B; 0x2540 102B = LPC1114FHN33/301 0x2540 102B = LPC1114FHN33/302 0x2540 102B = LPC1114FHI33/302 0x0444 102B; 0x2540 102B = LPC1114FBD48/301 0x2540 102B = LPC1114FBD48/302 0x2540 102B = LPC11D14FBD100/302 0x1421 102B = LPC11C12/FBD48/301 0x1440 102B = LPC11C14/FBD48/301 0x1431 102B = LPC11C22/FBD48/301 0X1430 102B = LPC11C24/FBD48/301."]
    #[inline]
    pub fn deviceid(&self) -> DEVICEIDR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DEVICEIDR { bits }
    }
}
