#![allow(clippy::all)]
#![doc = "Peripheral access API for LPC111X/LPC11CXX/LPC11XXL/LPC11XXXL microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn PIO0_0();
    fn PIO0_1();
    fn PIO0_2();
    fn PIO0_3();
    fn PIO0_4();
    fn PIO0_5();
    fn PIO0_6();
    fn PIO0_7();
    fn PIO0_8();
    fn PIO0_9();
    fn PIO0_10();
    fn PIO0_11();
    fn PIO1_0();
    fn C_CAN();
    fn SPI1();
    fn I2C();
    fn CT16B0();
    fn CT16B1();
    fn CT32B0();
    fn CT32B1();
    fn SPI0();
    fn UART();
    fn ADC();
    fn WDT();
    fn BOD();
    fn FMC();
    fn GPIO3();
    fn GPIO2();
    fn GPIO1();
    fn GPIO0();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: PIO0_0 },
    Vector { _handler: PIO0_1 },
    Vector { _handler: PIO0_2 },
    Vector { _handler: PIO0_3 },
    Vector { _handler: PIO0_4 },
    Vector { _handler: PIO0_5 },
    Vector { _handler: PIO0_6 },
    Vector { _handler: PIO0_7 },
    Vector { _handler: PIO0_8 },
    Vector { _handler: PIO0_9 },
    Vector { _handler: PIO0_10 },
    Vector { _handler: PIO0_11 },
    Vector { _handler: PIO1_0 },
    Vector { _handler: C_CAN },
    Vector { _handler: SPI1 },
    Vector { _handler: I2C },
    Vector { _handler: CT16B0 },
    Vector { _handler: CT16B1 },
    Vector { _handler: CT32B0 },
    Vector { _handler: CT32B1 },
    Vector { _handler: SPI0 },
    Vector { _handler: UART },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC },
    Vector { _handler: WDT },
    Vector { _handler: BOD },
    Vector { _handler: FMC },
    Vector { _handler: GPIO3 },
    Vector { _handler: GPIO2 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO0 },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - PIO0_0"]
    PIO0_0,
    #[doc = "1 - PIO0_1"]
    PIO0_1,
    #[doc = "2 - PIO0_2"]
    PIO0_2,
    #[doc = "3 - PIO0_3"]
    PIO0_3,
    #[doc = "4 - PIO0_4"]
    PIO0_4,
    #[doc = "5 - PIO0_5"]
    PIO0_5,
    #[doc = "6 - PIO0_6"]
    PIO0_6,
    #[doc = "7 - PIO0_7"]
    PIO0_7,
    #[doc = "8 - PIO0_8"]
    PIO0_8,
    #[doc = "9 - PIO0_9"]
    PIO0_9,
    #[doc = "10 - PIO0_10"]
    PIO0_10,
    #[doc = "11 - PIO0_11"]
    PIO0_11,
    #[doc = "12 - PIO1_0"]
    PIO1_0,
    #[doc = "13 - C_CAN"]
    C_CAN,
    #[doc = "14 - SPI1"]
    SPI1,
    #[doc = "15 - I2C"]
    I2C,
    #[doc = "16 - CT16B0"]
    CT16B0,
    #[doc = "17 - CT16B1"]
    CT16B1,
    #[doc = "18 - CT32B0"]
    CT32B0,
    #[doc = "19 - CT32B1"]
    CT32B1,
    #[doc = "20 - SPI0"]
    SPI0,
    #[doc = "21 - UART"]
    UART,
    #[doc = "24 - ADC"]
    ADC,
    #[doc = "25 - WDT"]
    WDT,
    #[doc = "26 - BOD"]
    BOD,
    #[doc = "27 - FMC"]
    FMC,
    #[doc = "28 - GPIO3"]
    GPIO3,
    #[doc = "29 - GPIO2"]
    GPIO2,
    #[doc = "30 - GPIO1"]
    GPIO1,
    #[doc = "31 - GPIO0"]
    GPIO0,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PIO0_0 => 0,
            Interrupt::PIO0_1 => 1,
            Interrupt::PIO0_2 => 2,
            Interrupt::PIO0_3 => 3,
            Interrupt::PIO0_4 => 4,
            Interrupt::PIO0_5 => 5,
            Interrupt::PIO0_6 => 6,
            Interrupt::PIO0_7 => 7,
            Interrupt::PIO0_8 => 8,
            Interrupt::PIO0_9 => 9,
            Interrupt::PIO0_10 => 10,
            Interrupt::PIO0_11 => 11,
            Interrupt::PIO1_0 => 12,
            Interrupt::C_CAN => 13,
            Interrupt::SPI1 => 14,
            Interrupt::I2C => 15,
            Interrupt::CT16B0 => 16,
            Interrupt::CT16B1 => 17,
            Interrupt::CT32B0 => 18,
            Interrupt::CT32B1 => 19,
            Interrupt::SPI0 => 20,
            Interrupt::UART => 21,
            Interrupt::ADC => 24,
            Interrupt::WDT => 25,
            Interrupt::BOD => 26,
            Interrupt::FMC => 27,
            Interrupt::GPIO3 => 28,
            Interrupt::GPIO2 => 29,
            Interrupt::GPIO1 => 30,
            Interrupt::GPIO0 => 31,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Inter-Integrated Circuit (I\u{b2}C) Controller"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    fn deref(&self) -> &i2c::RegisterBlock {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit (I\u{b2}C) Controller"]
pub mod i2c;
#[doc = "Windowed Watchdog Timer"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdt::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &wwdt::RegisterBlock {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Windowed Watchdog Timer"]
pub mod wwdt;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    fn deref(&self) -> &uart::RegisterBlock {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart;
#[doc = "16-bit Counter/Timer"]
pub struct CT16B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT16B0 {}
impl CT16B0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct16b0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for CT16B0 {
    type Target = ct16b0::RegisterBlock;
    fn deref(&self) -> &ct16b0::RegisterBlock {
        unsafe { &*CT16B0::ptr() }
    }
}
#[doc = "16-bit Counter/Timer"]
pub mod ct16b0;
#[doc = "CT16B1"]
pub struct CT16B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT16B1 {}
impl CT16B1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct16b0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for CT16B1 {
    type Target = ct16b0::RegisterBlock;
    fn deref(&self) -> &ct16b0::RegisterBlock {
        unsafe { &*CT16B1::ptr() }
    }
}
#[doc = "32-bit Counter/Timer"]
pub struct CT32B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT32B0 {}
impl CT32B0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct32b0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for CT32B0 {
    type Target = ct32b0::RegisterBlock;
    fn deref(&self) -> &ct32b0::RegisterBlock {
        unsafe { &*CT32B0::ptr() }
    }
}
#[doc = "32-bit Counter/Timer"]
pub mod ct32b0;
#[doc = "CT32B1"]
pub struct CT32B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT32B1 {}
impl CT32B1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct32b0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for CT32B1 {
    type Target = ct32b0::RegisterBlock;
    fn deref(&self) -> &ct32b0::RegisterBlock {
        unsafe { &*CT32B1::ptr() }
    }
}
#[doc = "10-bit Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "10-bit Analog-to-Digital Converter"]
pub mod adc;
#[doc = "Power Management Unit"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    fn deref(&self) -> &pmu::RegisterBlock {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "Power Management Unit"]
pub mod pmu;
#[doc = "Flash Programming Firmware"]
pub struct FLASHCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHCTRL {}
impl FLASHCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flashctrl::RegisterBlock {
        1073987584 as *const _
    }
}
impl Deref for FLASHCTRL {
    type Target = flashctrl::RegisterBlock;
    fn deref(&self) -> &flashctrl::RegisterBlock {
        unsafe { &*FLASHCTRL::ptr() }
    }
}
#[doc = "Flash Programming Firmware"]
pub mod flashctrl;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "I/O Configuration Block"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iocon::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    fn deref(&self) -> &iocon::RegisterBlock {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "I/O Configuration Block"]
pub mod iocon;
#[doc = "System Configuration Block"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscon::RegisterBlock {
        1074036736 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &syscon::RegisterBlock {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "System Configuration Block"]
pub mod syscon;
#[doc = "Controller Area Network Controller"]
pub struct C_CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for C_CAN {}
impl C_CAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const c_can::RegisterBlock {
        1074069504 as *const _
    }
}
impl Deref for C_CAN {
    type Target = c_can::RegisterBlock;
    fn deref(&self) -> &c_can::RegisterBlock {
        unsafe { &*C_CAN::ptr() }
    }
}
#[doc = "Controller Area Network Controller"]
pub mod c_can;
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074102272 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub struct GPIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0 {}
impl GPIO0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio0::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    fn deref(&self) -> &gpio0::RegisterBlock {
        unsafe { &*GPIO0::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio0;
#[doc = "GPIO1"]
pub struct GPIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO1 {}
impl GPIO1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio0::RegisterBlock {
        1342242816 as *const _
    }
}
impl Deref for GPIO1 {
    type Target = gpio0::RegisterBlock;
    fn deref(&self) -> &gpio0::RegisterBlock {
        unsafe { &*GPIO1::ptr() }
    }
}
#[doc = "GPIO2"]
pub struct GPIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO2 {}
impl GPIO2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio0::RegisterBlock {
        1342308352 as *const _
    }
}
impl Deref for GPIO2 {
    type Target = gpio0::RegisterBlock;
    fn deref(&self) -> &gpio0::RegisterBlock {
        unsafe { &*GPIO2::ptr() }
    }
}
#[doc = "GPIO3"]
pub struct GPIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO3 {}
impl GPIO3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio0::RegisterBlock {
        1342373888 as *const _
    }
}
impl Deref for GPIO3 {
    type Target = gpio0::RegisterBlock;
    fn deref(&self) -> &gpio0::RegisterBlock {
        unsafe { &*GPIO3::ptr() }
    }
}
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "CT16B0"]
    pub CT16B0: CT16B0,
    #[doc = "CT16B1"]
    pub CT16B1: CT16B1,
    #[doc = "CT32B0"]
    pub CT32B0: CT32B0,
    #[doc = "CT32B1"]
    pub CT32B1: CT32B1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "FLASHCTRL"]
    pub FLASHCTRL: FLASHCTRL,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "C_CAN"]
    pub C_CAN: C_CAN,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "GPIO1"]
    pub GPIO1: GPIO1,
    #[doc = "GPIO2"]
    pub GPIO2: GPIO2,
    #[doc = "GPIO3"]
    pub GPIO3: GPIO3,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            I2C: I2C {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            CT16B0: CT16B0 {
                _marker: PhantomData,
            },
            CT16B1: CT16B1 {
                _marker: PhantomData,
            },
            CT32B0: CT32B0 {
                _marker: PhantomData,
            },
            CT32B1: CT32B1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            FLASHCTRL: FLASHCTRL {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            C_CAN: C_CAN {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            GPIO0: GPIO0 {
                _marker: PhantomData,
            },
            GPIO1: GPIO1 {
                _marker: PhantomData,
            },
            GPIO2: GPIO2 {
                _marker: PhantomData,
            },
            GPIO3: GPIO3 {
                _marker: PhantomData,
            },
        }
    }
}
