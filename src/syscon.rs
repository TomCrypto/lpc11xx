#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System memory remap"]
    pub sysmemremap: SYSMEMREMAP,
    #[doc = "0x04 - Peripheral reset control"]
    pub presetctrl: PRESETCTRL,
    #[doc = "0x08 - System PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x0c - System PLL status"]
    pub syspllstat: SYSPLLSTAT,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - System oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    #[doc = "0x28 - IRC control"]
    pub ircctrl: IRCCTRL,
    _reserved7: [u8; 4usize],
    #[doc = "0x30 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved8: [u8; 12usize],
    #[doc = "0x40 - System PLL clock source select"]
    pub syspllclksel: SYSPLLCLKSEL,
    #[doc = "0x44 - System PLL clock source update enable"]
    pub syspllclkuen: SYSPLLCLKUEN,
    _reserved10: [u8; 40usize],
    #[doc = "0x70 - Main clock source select"]
    pub mainclksel: MAINCLKSEL,
    #[doc = "0x74 - Main clock source update enable"]
    pub mainclkuen: MAINCLKUEN,
    #[doc = "0x78 - System AHB clock divider"]
    pub sysahbclkdiv: SYSAHBCLKDIV,
    _reserved13: [u8; 4usize],
    #[doc = "0x80 - System AHB clock control"]
    pub sysahbclkctrl: SYSAHBCLKCTRL,
    _reserved14: [u8; 16usize],
    #[doc = "0x94 - SPI0 clock divider"]
    pub ssp0clkdiv: SSP0CLKDIV,
    #[doc = "0x98 - UART clock divder"]
    pub uartclkdiv: UARTCLKDIV,
    #[doc = "0x9c - SPI1 clock divder"]
    pub ssp1clkdiv: SSP1CLKDIV,
    _reserved17: [u8; 48usize],
    #[doc = "0xd0 - WDT clock source select"]
    pub wdtclksel: WDTCLKSEL,
    #[doc = "0xd4 - WDT clock source update enable"]
    pub wdtclkuen: WDTCLKUEN,
    #[doc = "0xd8 - WDT clock divider"]
    pub wdtclkdiv: WDTCLKDIV,
    _reserved20: [u8; 4usize],
    #[doc = "0xe0 - CLKOUT clock source select"]
    pub clkoutclksel: CLKOUTCLKSEL,
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    pub clkoutuen: CLKOUTUEN,
    #[doc = "0xe8 - CLKOUT clock divider"]
    pub clkoutclkdiv: CLKOUTCLKDIV,
    _reserved23: [u8; 20usize],
    #[doc = "0x100 - POR captured PIO status 0"]
    pub pioporcap0: PIOPORCAP0,
    #[doc = "0x104 - POR captured PIO status 1"]
    pub pioporcap1: PIOPORCAP1,
    _reserved25: [u8; 72usize],
    #[doc = "0x150 - BOD control"]
    pub bodctrl: BODCTRL,
    #[doc = "0x154 - System tick counter calibration"]
    pub systckcal: SYSTCKCAL,
    _reserved27: [u8; 28usize],
    #[doc = "0x174 - NMI source selection"]
    pub nmisrc: NMISRC,
    _reserved28: [u8; 136usize],
    #[doc = "0x200 - Start logic edge control register 0"]
    pub startaprp0: STARTAPRP0,
    #[doc = "0x204 - Start logic signal enable register 0"]
    pub starterp0: STARTERP0,
    #[doc = "0x208 - Start logic reset register 0"]
    pub startrsrp0clr: STARTRSRP0CLR,
    #[doc = "0x20c - Start logic status register 0"]
    pub startsrp0: STARTSRP0,
    _reserved32: [u8; 32usize],
    #[doc = "0x230 - Power-down states in Deep-sleep mode"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Power-down states after wake-up from Deep-sleep mode"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power-down configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved35: [u8; 440usize],
    #[doc = "0x3f4 - Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L"]
    pub device_id: DEVICE_ID,
}
#[doc = "System memory remap\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysmemremap](sysmemremap) module"]
pub type SYSMEMREMAP = crate::Reg<u32, _SYSMEMREMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSMEMREMAP;
#[doc = "`read()` method returns [sysmemremap::R](sysmemremap::R) reader structure"]
impl crate::Readable for SYSMEMREMAP {}
#[doc = "`write(|w| ..)` method takes [sysmemremap::W](sysmemremap::W) writer structure"]
impl crate::Writable for SYSMEMREMAP {}
#[doc = "System memory remap"]
pub mod sysmemremap;
#[doc = "Peripheral reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [presetctrl](presetctrl) module"]
pub type PRESETCTRL = crate::Reg<u32, _PRESETCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL;
#[doc = "`read()` method returns [presetctrl::R](presetctrl::R) reader structure"]
impl crate::Readable for PRESETCTRL {}
#[doc = "`write(|w| ..)` method takes [presetctrl::W](presetctrl::W) writer structure"]
impl crate::Writable for PRESETCTRL {}
#[doc = "Peripheral reset control"]
pub mod presetctrl;
#[doc = "System PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syspllctrl](syspllctrl) module"]
pub type SYSPLLCTRL = crate::Reg<u32, _SYSPLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCTRL;
#[doc = "`read()` method returns [syspllctrl::R](syspllctrl::R) reader structure"]
impl crate::Readable for SYSPLLCTRL {}
#[doc = "`write(|w| ..)` method takes [syspllctrl::W](syspllctrl::W) writer structure"]
impl crate::Writable for SYSPLLCTRL {}
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "System PLL status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syspllstat](syspllstat) module"]
pub type SYSPLLSTAT = crate::Reg<u32, _SYSPLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLSTAT;
#[doc = "`read()` method returns [syspllstat::R](syspllstat::R) reader structure"]
impl crate::Readable for SYSPLLSTAT {}
#[doc = "System PLL status"]
pub mod syspllstat;
#[doc = "System oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysoscctrl](sysoscctrl) module"]
pub type SYSOSCCTRL = crate::Reg<u32, _SYSOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSOSCCTRL;
#[doc = "`read()` method returns [sysoscctrl::R](sysoscctrl::R) reader structure"]
impl crate::Readable for SYSOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [sysoscctrl::W](sysoscctrl::W) writer structure"]
impl crate::Writable for SYSOSCCTRL {}
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "Watchdog oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtoscctrl](wdtoscctrl) module"]
pub type WDTOSCCTRL = crate::Reg<u32, _WDTOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTOSCCTRL;
#[doc = "`read()` method returns [wdtoscctrl::R](wdtoscctrl::R) reader structure"]
impl crate::Readable for WDTOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [wdtoscctrl::W](wdtoscctrl::W) writer structure"]
impl crate::Writable for WDTOSCCTRL {}
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "IRC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ircctrl](ircctrl) module"]
pub type IRCCTRL = crate::Reg<u32, _IRCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRCCTRL;
#[doc = "`read()` method returns [ircctrl::R](ircctrl::R) reader structure"]
impl crate::Readable for IRCCTRL {}
#[doc = "`write(|w| ..)` method takes [ircctrl::W](ircctrl::W) writer structure"]
impl crate::Writable for IRCCTRL {}
#[doc = "IRC control"]
pub mod ircctrl;
#[doc = "System reset status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrststat](sysrststat) module"]
pub type SYSRSTSTAT = crate::Reg<u32, _SYSRSTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSRSTSTAT;
#[doc = "`read()` method returns [sysrststat::R](sysrststat::R) reader structure"]
impl crate::Readable for SYSRSTSTAT {}
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "System PLL clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syspllclksel](syspllclksel) module"]
pub type SYSPLLCLKSEL = crate::Reg<u32, _SYSPLLCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCLKSEL;
#[doc = "`read()` method returns [syspllclksel::R](syspllclksel::R) reader structure"]
impl crate::Readable for SYSPLLCLKSEL {}
#[doc = "`write(|w| ..)` method takes [syspllclksel::W](syspllclksel::W) writer structure"]
impl crate::Writable for SYSPLLCLKSEL {}
#[doc = "System PLL clock source select"]
pub mod syspllclksel;
#[doc = "System PLL clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syspllclkuen](syspllclkuen) module"]
pub type SYSPLLCLKUEN = crate::Reg<u32, _SYSPLLCLKUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCLKUEN;
#[doc = "`read()` method returns [syspllclkuen::R](syspllclkuen::R) reader structure"]
impl crate::Readable for SYSPLLCLKUEN {}
#[doc = "`write(|w| ..)` method takes [syspllclkuen::W](syspllclkuen::W) writer structure"]
impl crate::Writable for SYSPLLCLKUEN {}
#[doc = "System PLL clock source update enable"]
pub mod syspllclkuen;
#[doc = "Main clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mainclksel](mainclksel) module"]
pub type MAINCLKSEL = crate::Reg<u32, _MAINCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKSEL;
#[doc = "`read()` method returns [mainclksel::R](mainclksel::R) reader structure"]
impl crate::Readable for MAINCLKSEL {}
#[doc = "`write(|w| ..)` method takes [mainclksel::W](mainclksel::W) writer structure"]
impl crate::Writable for MAINCLKSEL {}
#[doc = "Main clock source select"]
pub mod mainclksel;
#[doc = "Main clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mainclkuen](mainclkuen) module"]
pub type MAINCLKUEN = crate::Reg<u32, _MAINCLKUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKUEN;
#[doc = "`read()` method returns [mainclkuen::R](mainclkuen::R) reader structure"]
impl crate::Readable for MAINCLKUEN {}
#[doc = "`write(|w| ..)` method takes [mainclkuen::W](mainclkuen::W) writer structure"]
impl crate::Writable for MAINCLKUEN {}
#[doc = "Main clock source update enable"]
pub mod mainclkuen;
#[doc = "System AHB clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysahbclkdiv](sysahbclkdiv) module"]
pub type SYSAHBCLKDIV = crate::Reg<u32, _SYSAHBCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSAHBCLKDIV;
#[doc = "`read()` method returns [sysahbclkdiv::R](sysahbclkdiv::R) reader structure"]
impl crate::Readable for SYSAHBCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sysahbclkdiv::W](sysahbclkdiv::W) writer structure"]
impl crate::Writable for SYSAHBCLKDIV {}
#[doc = "System AHB clock divider"]
pub mod sysahbclkdiv;
#[doc = "System AHB clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysahbclkctrl](sysahbclkctrl) module"]
pub type SYSAHBCLKCTRL = crate::Reg<u32, _SYSAHBCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSAHBCLKCTRL;
#[doc = "`read()` method returns [sysahbclkctrl::R](sysahbclkctrl::R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl::W](sysahbclkctrl::W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL {}
#[doc = "System AHB clock control"]
pub mod sysahbclkctrl;
#[doc = "SPI0 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssp0clkdiv](ssp0clkdiv) module"]
pub type SSP0CLKDIV = crate::Reg<u32, _SSP0CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSP0CLKDIV;
#[doc = "`read()` method returns [ssp0clkdiv::R](ssp0clkdiv::R) reader structure"]
impl crate::Readable for SSP0CLKDIV {}
#[doc = "`write(|w| ..)` method takes [ssp0clkdiv::W](ssp0clkdiv::W) writer structure"]
impl crate::Writable for SSP0CLKDIV {}
#[doc = "SPI0 clock divider"]
pub mod ssp0clkdiv;
#[doc = "UART clock divder\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartclkdiv](uartclkdiv) module"]
pub type UARTCLKDIV = crate::Reg<u32, _UARTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCLKDIV;
#[doc = "`read()` method returns [uartclkdiv::R](uartclkdiv::R) reader structure"]
impl crate::Readable for UARTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [uartclkdiv::W](uartclkdiv::W) writer structure"]
impl crate::Writable for UARTCLKDIV {}
#[doc = "UART clock divder"]
pub mod uartclkdiv;
#[doc = "SPI1 clock divder\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssp1clkdiv](ssp1clkdiv) module"]
pub type SSP1CLKDIV = crate::Reg<u32, _SSP1CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSP1CLKDIV;
#[doc = "`read()` method returns [ssp1clkdiv::R](ssp1clkdiv::R) reader structure"]
impl crate::Readable for SSP1CLKDIV {}
#[doc = "`write(|w| ..)` method takes [ssp1clkdiv::W](ssp1clkdiv::W) writer structure"]
impl crate::Writable for SSP1CLKDIV {}
#[doc = "SPI1 clock divder"]
pub mod ssp1clkdiv;
#[doc = "WDT clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtclksel](wdtclksel) module"]
pub type WDTCLKSEL = crate::Reg<u32, _WDTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLKSEL;
#[doc = "`read()` method returns [wdtclksel::R](wdtclksel::R) reader structure"]
impl crate::Readable for WDTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [wdtclksel::W](wdtclksel::W) writer structure"]
impl crate::Writable for WDTCLKSEL {}
#[doc = "WDT clock source select"]
pub mod wdtclksel;
#[doc = "WDT clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtclkuen](wdtclkuen) module"]
pub type WDTCLKUEN = crate::Reg<u32, _WDTCLKUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLKUEN;
#[doc = "`read()` method returns [wdtclkuen::R](wdtclkuen::R) reader structure"]
impl crate::Readable for WDTCLKUEN {}
#[doc = "`write(|w| ..)` method takes [wdtclkuen::W](wdtclkuen::W) writer structure"]
impl crate::Writable for WDTCLKUEN {}
#[doc = "WDT clock source update enable"]
pub mod wdtclkuen;
#[doc = "WDT clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtclkdiv](wdtclkdiv) module"]
pub type WDTCLKDIV = crate::Reg<u32, _WDTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLKDIV;
#[doc = "`read()` method returns [wdtclkdiv::R](wdtclkdiv::R) reader structure"]
impl crate::Readable for WDTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [wdtclkdiv::W](wdtclkdiv::W) writer structure"]
impl crate::Writable for WDTCLKDIV {}
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "CLKOUT clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkoutclksel](clkoutclksel) module"]
pub type CLKOUTCLKSEL = crate::Reg<u32, _CLKOUTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTCLKSEL;
#[doc = "`read()` method returns [clkoutclksel::R](clkoutclksel::R) reader structure"]
impl crate::Readable for CLKOUTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [clkoutclksel::W](clkoutclksel::W) writer structure"]
impl crate::Writable for CLKOUTCLKSEL {}
#[doc = "CLKOUT clock source select"]
pub mod clkoutclksel;
#[doc = "CLKOUT clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkoutuen](clkoutuen) module"]
pub type CLKOUTUEN = crate::Reg<u32, _CLKOUTUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTUEN;
#[doc = "`read()` method returns [clkoutuen::R](clkoutuen::R) reader structure"]
impl crate::Readable for CLKOUTUEN {}
#[doc = "`write(|w| ..)` method takes [clkoutuen::W](clkoutuen::W) writer structure"]
impl crate::Writable for CLKOUTUEN {}
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUT clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkoutclkdiv](clkoutclkdiv) module"]
pub type CLKOUTCLKDIV = crate::Reg<u32, _CLKOUTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTCLKDIV;
#[doc = "`read()` method returns [clkoutclkdiv::R](clkoutclkdiv::R) reader structure"]
impl crate::Readable for CLKOUTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkoutclkdiv::W](clkoutclkdiv::W) writer structure"]
impl crate::Writable for CLKOUTCLKDIV {}
#[doc = "CLKOUT clock divider"]
pub mod clkoutclkdiv;
#[doc = "POR captured PIO status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pioporcap0](pioporcap0) module"]
pub type PIOPORCAP0 = crate::Reg<u32, _PIOPORCAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOPORCAP0;
#[doc = "`read()` method returns [pioporcap0::R](pioporcap0::R) reader structure"]
impl crate::Readable for PIOPORCAP0 {}
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "POR captured PIO status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pioporcap1](pioporcap1) module"]
pub type PIOPORCAP1 = crate::Reg<u32, _PIOPORCAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOPORCAP1;
#[doc = "`read()` method returns [pioporcap1::R](pioporcap1::R) reader structure"]
impl crate::Readable for PIOPORCAP1 {}
#[doc = "POR captured PIO status 1"]
pub mod pioporcap1;
#[doc = "BOD control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bodctrl](bodctrl) module"]
pub type BODCTRL = crate::Reg<u32, _BODCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODCTRL;
#[doc = "`read()` method returns [bodctrl::R](bodctrl::R) reader structure"]
impl crate::Readable for BODCTRL {}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](bodctrl::W) writer structure"]
impl crate::Writable for BODCTRL {}
#[doc = "BOD control"]
pub mod bodctrl;
#[doc = "System tick counter calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systckcal](systckcal) module"]
pub type SYSTCKCAL = crate::Reg<u32, _SYSTCKCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTCKCAL;
#[doc = "`read()` method returns [systckcal::R](systckcal::R) reader structure"]
impl crate::Readable for SYSTCKCAL {}
#[doc = "`write(|w| ..)` method takes [systckcal::W](systckcal::W) writer structure"]
impl crate::Writable for SYSTCKCAL {}
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "NMI source selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nmisrc](nmisrc) module"]
pub type NMISRC = crate::Reg<u32, _NMISRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMISRC;
#[doc = "`read()` method returns [nmisrc::R](nmisrc::R) reader structure"]
impl crate::Readable for NMISRC {}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](nmisrc::W) writer structure"]
impl crate::Writable for NMISRC {}
#[doc = "NMI source selection"]
pub mod nmisrc;
#[doc = "Start logic edge control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [startaprp0](startaprp0) module"]
pub type STARTAPRP0 = crate::Reg<u32, _STARTAPRP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTAPRP0;
#[doc = "`read()` method returns [startaprp0::R](startaprp0::R) reader structure"]
impl crate::Readable for STARTAPRP0 {}
#[doc = "`write(|w| ..)` method takes [startaprp0::W](startaprp0::W) writer structure"]
impl crate::Writable for STARTAPRP0 {}
#[doc = "Start logic edge control register 0"]
pub mod startaprp0;
#[doc = "Start logic signal enable register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [starterp0](starterp0) module"]
pub type STARTERP0 = crate::Reg<u32, _STARTERP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERP0;
#[doc = "`read()` method returns [starterp0::R](starterp0::R) reader structure"]
impl crate::Readable for STARTERP0 {}
#[doc = "`write(|w| ..)` method takes [starterp0::W](starterp0::W) writer structure"]
impl crate::Writable for STARTERP0 {}
#[doc = "Start logic signal enable register 0"]
pub mod starterp0;
#[doc = "Start logic reset register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [startrsrp0clr](startrsrp0clr) module"]
pub type STARTRSRP0CLR = crate::Reg<u32, _STARTRSRP0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTRSRP0CLR;
#[doc = "`write(|w| ..)` method takes [startrsrp0clr::W](startrsrp0clr::W) writer structure"]
impl crate::Writable for STARTRSRP0CLR {}
#[doc = "Start logic reset register 0"]
pub mod startrsrp0clr;
#[doc = "Start logic status register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [startsrp0](startsrp0) module"]
pub type STARTSRP0 = crate::Reg<u32, _STARTSRP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTSRP0;
#[doc = "`read()` method returns [startsrp0::R](startsrp0::R) reader structure"]
impl crate::Readable for STARTSRP0 {}
#[doc = "Start logic status register 0"]
pub mod startsrp0;
#[doc = "Power-down states in Deep-sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdsleepcfg](pdsleepcfg) module"]
pub type PDSLEEPCFG = crate::Reg<u32, _PDSLEEPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSLEEPCFG;
#[doc = "`read()` method returns [pdsleepcfg::R](pdsleepcfg::R) reader structure"]
impl crate::Readable for PDSLEEPCFG {}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg::W](pdsleepcfg::W) writer structure"]
impl crate::Writable for PDSLEEPCFG {}
#[doc = "Power-down states in Deep-sleep mode"]
pub mod pdsleepcfg;
#[doc = "Power-down states after wake-up from Deep-sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdawakecfg](pdawakecfg) module"]
pub type PDAWAKECFG = crate::Reg<u32, _PDAWAKECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAWAKECFG;
#[doc = "`read()` method returns [pdawakecfg::R](pdawakecfg::R) reader structure"]
impl crate::Readable for PDAWAKECFG {}
#[doc = "`write(|w| ..)` method takes [pdawakecfg::W](pdawakecfg::W) writer structure"]
impl crate::Writable for PDAWAKECFG {}
#[doc = "Power-down states after wake-up from Deep-sleep mode"]
pub mod pdawakecfg;
#[doc = "Power-down configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdruncfg](pdruncfg) module"]
pub type PDRUNCFG = crate::Reg<u32, _PDRUNCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFG;
#[doc = "`read()` method returns [pdruncfg::R](pdruncfg::R) reader structure"]
impl crate::Readable for PDRUNCFG {}
#[doc = "`write(|w| ..)` method takes [pdruncfg::W](pdruncfg::W) writer structure"]
impl crate::Writable for PDRUNCFG {}
#[doc = "Power-down configuration register"]
pub mod pdruncfg;
#[doc = "Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [device_id](device_id) module"]
pub type DEVICE_ID = crate::Reg<u32, _DEVICE_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_ID;
#[doc = "`read()` method returns [device_id::R](device_id::R) reader structure"]
impl crate::Readable for DEVICE_ID {}
#[doc = "Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L"]
pub mod device_id;
