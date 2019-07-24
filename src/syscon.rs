#[doc = r" Register block"]
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
#[doc = "System memory remap"]
pub struct SYSMEMREMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System memory remap"]
pub mod sysmemremap;
#[doc = "Peripheral reset control"]
pub struct PRESETCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control"]
pub mod presetctrl;
#[doc = "System PLL control"]
pub struct SYSPLLCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "System PLL status"]
pub struct SYSPLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL status"]
pub mod syspllstat;
#[doc = "System oscillator control"]
pub struct SYSOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "Watchdog oscillator control"]
pub struct WDTOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "IRC control"]
pub struct IRCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRC control"]
pub mod ircctrl;
#[doc = "System reset status register"]
pub struct SYSRSTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "System PLL clock source select"]
pub struct SYSPLLCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL clock source select"]
pub mod syspllclksel;
#[doc = "System PLL clock source update enable"]
pub struct SYSPLLCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL clock source update enable"]
pub mod syspllclkuen;
#[doc = "Main clock source select"]
pub struct MAINCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source select"]
pub mod mainclksel;
#[doc = "Main clock source update enable"]
pub struct MAINCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source update enable"]
pub mod mainclkuen;
#[doc = "System AHB clock divider"]
pub struct SYSAHBCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System AHB clock divider"]
pub mod sysahbclkdiv;
#[doc = "System AHB clock control"]
pub struct SYSAHBCLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System AHB clock control"]
pub mod sysahbclkctrl;
#[doc = "SPI0 clock divider"]
pub struct SSP0CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI0 clock divider"]
pub mod ssp0clkdiv;
#[doc = "UART clock divder"]
pub struct UARTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART clock divder"]
pub mod uartclkdiv;
#[doc = "SPI1 clock divder"]
pub struct SSP1CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1 clock divder"]
pub mod ssp1clkdiv;
#[doc = "WDT clock source select"]
pub struct WDTCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT clock source select"]
pub mod wdtclksel;
#[doc = "WDT clock source update enable"]
pub struct WDTCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT clock source update enable"]
pub mod wdtclkuen;
#[doc = "WDT clock divider"]
pub struct WDTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "CLKOUT clock source select"]
pub struct CLKOUTCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock source select"]
pub mod clkoutclksel;
#[doc = "CLKOUT clock source update enable"]
pub struct CLKOUTUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUT clock divider"]
pub struct CLKOUTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock divider"]
pub mod clkoutclkdiv;
#[doc = "POR captured PIO status 0"]
pub struct PIOPORCAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "POR captured PIO status 1"]
pub struct PIOPORCAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POR captured PIO status 1"]
pub mod pioporcap1;
#[doc = "BOD control"]
pub struct BODCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD control"]
pub mod bodctrl;
#[doc = "System tick counter calibration"]
pub struct SYSTCKCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "NMI source selection"]
pub struct NMISRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NMI source selection"]
pub mod nmisrc;
#[doc = "Start logic edge control register 0"]
pub struct STARTAPRP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic edge control register 0"]
pub mod startaprp0;
#[doc = "Start logic signal enable register 0"]
pub struct STARTERP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic signal enable register 0"]
pub mod starterp0;
#[doc = "Start logic reset register 0"]
pub struct STARTRSRP0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic reset register 0"]
pub mod startrsrp0clr;
#[doc = "Start logic status register 0"]
pub struct STARTSRP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic status register 0"]
pub mod startsrp0;
#[doc = "Power-down states in Deep-sleep mode"]
pub struct PDSLEEPCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-down states in Deep-sleep mode"]
pub mod pdsleepcfg;
#[doc = "Power-down states after wake-up from Deep-sleep mode"]
pub struct PDAWAKECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-down states after wake-up from Deep-sleep mode"]
pub mod pdawakecfg;
#[doc = "Power-down configuration register"]
pub struct PDRUNCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-down configuration register"]
pub mod pdruncfg;
#[doc = "Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L"]
pub struct DEVICE_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L"]
pub mod device_id;
