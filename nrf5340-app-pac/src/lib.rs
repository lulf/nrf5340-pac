#![doc = "Peripheral access API for NRF5340_APPLICATION microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn FPU();
    fn CACHE();
    fn SPU();
    fn CLOCK_POWER();
    fn SPIM0_SPIS0_TWIM0_TWIS0_UARTE0();
    fn SPIM1_SPIS1_TWIM1_TWIS1_UARTE1();
    fn SPIM2();
    fn GPIOTE0();
    fn SAADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn RTC1();
    fn WDT0();
    fn WDT1();
    fn COMP_LPCOMP();
    fn EGU0();
    fn EGU1();
    fn EGU2();
    fn EGU3();
    fn EGU4();
    fn EGU5();
    fn PWM0();
    fn PWM1();
    fn PWM2();
    fn PDM0();
    fn I2S0();
    fn IPC();
    fn QSPI();
    fn NFCT();
    fn GPIOTE1();
    fn QDEC();
    fn USBD();
    fn USBREGULATOR();
    fn KMU();
    fn CRYPTOCELL();
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
pub static __INTERRUPTS: [Vector; 69] = [
    Vector { _handler: FPU },
    Vector { _handler: CACHE },
    Vector { _reserved: 0 },
    Vector { _handler: SPU },
    Vector { _reserved: 0 },
    Vector {
        _handler: CLOCK_POWER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: SPIM0_SPIS0_TWIM0_TWIS0_UARTE0,
    },
    Vector {
        _handler: SPIM1_SPIS1_TWIM1_TWIS1_UARTE1,
    },
    Vector { _handler: SPIM2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE0 },
    Vector { _handler: SAADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC0 },
    Vector { _handler: RTC1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDT0 },
    Vector { _handler: WDT1 },
    Vector {
        _handler: COMP_LPCOMP,
    },
    Vector { _handler: EGU0 },
    Vector { _handler: EGU1 },
    Vector { _handler: EGU2 },
    Vector { _handler: EGU3 },
    Vector { _handler: EGU4 },
    Vector { _handler: EGU5 },
    Vector { _handler: PWM0 },
    Vector { _handler: PWM1 },
    Vector { _handler: PWM2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PDM0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2S0 },
    Vector { _reserved: 0 },
    Vector { _handler: IPC },
    Vector { _handler: QSPI },
    Vector { _reserved: 0 },
    Vector { _handler: NFCT },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: QDEC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USBD },
    Vector {
        _handler: USBREGULATOR,
    },
    Vector { _reserved: 0 },
    Vector { _handler: KMU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: CRYPTOCELL,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - FPU"]
    FPU,
    #[doc = "1 - CACHE"]
    CACHE,
    #[doc = "3 - SPU"]
    SPU,
    #[doc = "5 - CLOCK_POWER"]
    CLOCK_POWER,
    #[doc = "8 - SPIM0_SPIS0_TWIM0_TWIS0_UARTE0"]
    SPIM0_SPIS0_TWIM0_TWIS0_UARTE0,
    #[doc = "9 - SPIM1_SPIS1_TWIM1_TWIS1_UARTE1"]
    SPIM1_SPIS1_TWIM1_TWIS1_UARTE1,
    #[doc = "10 - SPIM2"]
    SPIM2,
    #[doc = "13 - GPIOTE0"]
    GPIOTE0,
    #[doc = "14 - SAADC"]
    SAADC,
    #[doc = "15 - TIMER0"]
    TIMER0,
    #[doc = "16 - TIMER1"]
    TIMER1,
    #[doc = "17 - TIMER2"]
    TIMER2,
    #[doc = "20 - RTC0"]
    RTC0,
    #[doc = "21 - RTC1"]
    RTC1,
    #[doc = "24 - WDT0"]
    WDT0,
    #[doc = "25 - WDT1"]
    WDT1,
    #[doc = "26 - COMP_LPCOMP"]
    COMP_LPCOMP,
    #[doc = "27 - EGU0"]
    EGU0,
    #[doc = "28 - EGU1"]
    EGU1,
    #[doc = "29 - EGU2"]
    EGU2,
    #[doc = "30 - EGU3"]
    EGU3,
    #[doc = "31 - EGU4"]
    EGU4,
    #[doc = "32 - EGU5"]
    EGU5,
    #[doc = "33 - PWM0"]
    PWM0,
    #[doc = "34 - PWM1"]
    PWM1,
    #[doc = "35 - PWM2"]
    PWM2,
    #[doc = "38 - PDM0"]
    PDM0,
    #[doc = "40 - I2S0"]
    I2S0,
    #[doc = "42 - IPC"]
    IPC,
    #[doc = "43 - QSPI"]
    QSPI,
    #[doc = "45 - NFCT"]
    NFCT,
    #[doc = "47 - GPIOTE1"]
    GPIOTE1,
    #[doc = "51 - QDEC"]
    QDEC,
    #[doc = "54 - USBD"]
    USBD,
    #[doc = "55 - USBREGULATOR"]
    USBREGULATOR,
    #[doc = "57 - KMU"]
    KMU,
    #[doc = "68 - CRYPTOCELL"]
    CRYPTOCELL,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::FPU => 0,
            Interrupt::CACHE => 1,
            Interrupt::SPU => 3,
            Interrupt::CLOCK_POWER => 5,
            Interrupt::SPIM0_SPIS0_TWIM0_TWIS0_UARTE0 => 8,
            Interrupt::SPIM1_SPIS1_TWIM1_TWIS1_UARTE1 => 9,
            Interrupt::SPIM2 => 10,
            Interrupt::GPIOTE0 => 13,
            Interrupt::SAADC => 14,
            Interrupt::TIMER0 => 15,
            Interrupt::TIMER1 => 16,
            Interrupt::TIMER2 => 17,
            Interrupt::RTC0 => 20,
            Interrupt::RTC1 => 21,
            Interrupt::WDT0 => 24,
            Interrupt::WDT1 => 25,
            Interrupt::COMP_LPCOMP => 26,
            Interrupt::EGU0 => 27,
            Interrupt::EGU1 => 28,
            Interrupt::EGU2 => 29,
            Interrupt::EGU3 => 30,
            Interrupt::EGU4 => 31,
            Interrupt::EGU5 => 32,
            Interrupt::PWM0 => 33,
            Interrupt::PWM1 => 34,
            Interrupt::PWM2 => 35,
            Interrupt::PDM0 => 38,
            Interrupt::I2S0 => 40,
            Interrupt::IPC => 42,
            Interrupt::QSPI => 43,
            Interrupt::NFCT => 45,
            Interrupt::GPIOTE1 => 47,
            Interrupt::QDEC => 51,
            Interrupt::USBD => 54,
            Interrupt::USBREGULATOR => 55,
            Interrupt::KMU => 57,
            Interrupt::CRYPTOCELL => 68,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "CACHEDATA"]
pub struct CACHEDATA_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CACHEDATA_S {}
impl CACHEDATA_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cachedata_s::RegisterBlock {
        0x00f0_0000 as *const _
    }
}
impl Deref for CACHEDATA_S {
    type Target = cachedata_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CACHEDATA_S::ptr() }
    }
}
#[doc = "CACHEDATA"]
pub mod cachedata_s;
#[doc = "CACHEINFO"]
pub struct CACHEINFO_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CACHEINFO_S {}
impl CACHEINFO_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cacheinfo_s::RegisterBlock {
        0x00f0_8000 as *const _
    }
}
impl Deref for CACHEINFO_S {
    type Target = cacheinfo_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CACHEINFO_S::ptr() }
    }
}
#[doc = "CACHEINFO"]
pub mod cacheinfo_s;
#[doc = "Factory Information Configuration Registers"]
pub struct FICR_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR_S {}
impl FICR_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ficr_s::RegisterBlock {
        0x00ff_0000 as *const _
    }
}
impl Deref for FICR_S {
    type Target = ficr_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FICR_S::ptr() }
    }
}
#[doc = "Factory Information Configuration Registers"]
pub mod ficr_s;
#[doc = "User Information Configuration Registers User information configuration registers"]
pub struct UICR_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR_S {}
impl UICR_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uicr_s::RegisterBlock {
        0x00ff_8000 as *const _
    }
}
impl Deref for UICR_S {
    type Target = uicr_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UICR_S::ptr() }
    }
}
#[doc = "User Information Configuration Registers User information configuration registers"]
pub mod uicr_s;
#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
pub struct CTI_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTI_S {}
impl CTI_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cti_s::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for CTI_S {
    type Target = cti_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTI_S::ptr() }
    }
}
#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
pub mod cti_s;
#[doc = "Trace and debug control"]
pub struct TAD_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAD_S {}
impl TAD_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tad_s::RegisterBlock {
        0xe008_0000 as *const _
    }
}
impl Deref for TAD_S {
    type Target = tad_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TAD_S::ptr() }
    }
}
#[doc = "Trace and debug control"]
pub mod tad_s;
#[doc = "Domain configuration management 0"]
pub struct DCNF_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCNF_NS {}
impl DCNF_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcnf_ns::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for DCNF_NS {
    type Target = dcnf_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCNF_NS::ptr() }
    }
}
#[doc = "Domain configuration management 0"]
pub mod dcnf_ns;
#[doc = "FPU control peripheral 0"]
pub struct FPU_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_NS {}
impl FPU_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fpu_ns::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for FPU_NS {
    type Target = fpu_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU_NS::ptr() }
    }
}
#[doc = "FPU control peripheral 0"]
pub mod fpu_ns;
#[doc = "Domain configuration management 1"]
pub struct DCNF_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCNF_S {}
impl DCNF_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcnf_ns::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for DCNF_S {
    type Target = dcnf_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCNF_S::ptr() }
    }
}
#[doc = "FPU control peripheral 1"]
pub struct FPU_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_S {}
impl FPU_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fpu_ns::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for FPU_S {
    type Target = fpu_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU_S::ptr() }
    }
}
#[doc = "Cache"]
pub struct CACHE_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CACHE_S {}
impl CACHE_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cache_s::RegisterBlock {
        0x5000_1000 as *const _
    }
}
impl Deref for CACHE_S {
    type Target = cache_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CACHE_S::ptr() }
    }
}
#[doc = "Cache"]
pub mod cache_s;
#[doc = "System protection unit"]
pub struct SPU_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPU_S {}
impl SPU_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spu_s::RegisterBlock {
        0x5000_3000 as *const _
    }
}
impl Deref for SPU_S {
    type Target = spu_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPU_S::ptr() }
    }
}
#[doc = "System protection unit"]
pub mod spu_s;
#[doc = "Oscillator control 0"]
pub struct OSCILLATORS_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSCILLATORS_NS {}
impl OSCILLATORS_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oscillators_ns::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for OSCILLATORS_NS {
    type Target = oscillators_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSCILLATORS_NS::ptr() }
    }
}
#[doc = "Oscillator control 0"]
pub mod oscillators_ns;
#[doc = "Voltage regulators 0"]
pub struct REGULATORS_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REGULATORS_NS {}
impl REGULATORS_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const regulators_ns::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for REGULATORS_NS {
    type Target = regulators_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*REGULATORS_NS::ptr() }
    }
}
#[doc = "Voltage regulators 0"]
pub mod regulators_ns;
#[doc = "Oscillator control 1"]
pub struct OSCILLATORS_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSCILLATORS_S {}
impl OSCILLATORS_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oscillators_ns::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for OSCILLATORS_S {
    type Target = oscillators_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSCILLATORS_S::ptr() }
    }
}
#[doc = "Voltage regulators 1"]
pub struct REGULATORS_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REGULATORS_S {}
impl REGULATORS_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const regulators_ns::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for REGULATORS_S {
    type Target = regulators_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*REGULATORS_S::ptr() }
    }
}
#[doc = "Clock management 0"]
pub struct CLOCK_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK_NS {}
impl CLOCK_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clock_ns::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for CLOCK_NS {
    type Target = clock_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLOCK_NS::ptr() }
    }
}
#[doc = "Clock management 0"]
pub mod clock_ns;
#[doc = "Power control 0"]
pub struct POWER_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER_NS {}
impl POWER_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const power_ns::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for POWER_NS {
    type Target = power_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*POWER_NS::ptr() }
    }
}
#[doc = "Power control 0"]
pub mod power_ns;
#[doc = "Reset control 0"]
pub struct RESET_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RESET_NS {}
impl RESET_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const reset_ns::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for RESET_NS {
    type Target = reset_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RESET_NS::ptr() }
    }
}
#[doc = "Reset control 0"]
pub mod reset_ns;
#[doc = "Clock management 1"]
pub struct CLOCK_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK_S {}
impl CLOCK_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clock_ns::RegisterBlock {
        0x5000_5000 as *const _
    }
}
impl Deref for CLOCK_S {
    type Target = clock_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLOCK_S::ptr() }
    }
}
#[doc = "Power control 1"]
pub struct POWER_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER_S {}
impl POWER_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const power_ns::RegisterBlock {
        0x5000_5000 as *const _
    }
}
impl Deref for POWER_S {
    type Target = power_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*POWER_S::ptr() }
    }
}
#[doc = "Reset control 1"]
pub struct RESET_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RESET_S {}
impl RESET_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const reset_ns::RegisterBlock {
        0x5000_5000 as *const _
    }
}
impl Deref for RESET_S {
    type Target = reset_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RESET_S::ptr() }
    }
}
#[doc = "Control access port 0"]
pub struct CTRLAP_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTRLAP_NS {}
impl CTRLAP_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctrlap_ns::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for CTRLAP_NS {
    type Target = ctrlap_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTRLAP_NS::ptr() }
    }
}
#[doc = "Control access port 0"]
pub mod ctrlap_ns;
#[doc = "Control access port 1"]
pub struct CTRLAP_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTRLAP_S {}
impl CTRLAP_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctrlap_ns::RegisterBlock {
        0x5000_6000 as *const _
    }
}
impl Deref for CTRLAP_S {
    type Target = ctrlap_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTRLAP_S::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub struct SPIM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0_NS {}
impl SPIM0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0_ns::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SPIM0_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM0_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub mod spim0_ns;
#[doc = "SPI Slave 0"]
pub struct SPIS0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0_NS {}
impl SPIS0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis0_ns::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SPIS0_NS {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIS0_NS::ptr() }
    }
}
#[doc = "SPI Slave 0"]
pub mod spis0_ns;
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub struct TWIM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0_NS {}
impl TWIM0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0_ns::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for TWIM0_NS {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM0_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub mod twim0_ns;
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub struct TWIS0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0_NS {}
impl TWIS0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0_ns::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for TWIS0_NS {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS0_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub mod twis0_ns;
#[doc = "UART with EasyDMA 0"]
pub struct UARTE0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0_NS {}
impl UARTE0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarte0_ns::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for UARTE0_NS {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTE0_NS::ptr() }
    }
}
#[doc = "UART with EasyDMA 0"]
pub mod uarte0_ns;
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub struct SPIM0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0_S {}
impl SPIM0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0_ns::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for SPIM0_S {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM0_S::ptr() }
    }
}
#[doc = "SPI Slave 1"]
pub struct SPIS0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0_S {}
impl SPIS0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis0_ns::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for SPIS0_S {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIS0_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub struct TWIM0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0_S {}
impl TWIM0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0_ns::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for TWIM0_S {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM0_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub struct TWIS0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0_S {}
impl TWIS0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0_ns::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for TWIS0_S {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS0_S::ptr() }
    }
}
#[doc = "UART with EasyDMA 1"]
pub struct UARTE0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0_S {}
impl UARTE0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarte0_ns::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for UARTE0_S {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTE0_S::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 2"]
pub struct SPIM1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1_NS {}
impl SPIM1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0_ns::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for SPIM1_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM1_NS::ptr() }
    }
}
#[doc = "SPI Slave 2"]
pub struct SPIS1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1_NS {}
impl SPIS1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis0_ns::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for SPIS1_NS {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIS1_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 2"]
pub struct TWIM1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1_NS {}
impl TWIM1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0_ns::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for TWIM1_NS {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM1_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 2"]
pub struct TWIS1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1_NS {}
impl TWIS1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0_ns::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for TWIS1_NS {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS1_NS::ptr() }
    }
}
#[doc = "UART with EasyDMA 2"]
pub struct UARTE1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE1_NS {}
impl UARTE1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarte0_ns::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for UARTE1_NS {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTE1_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 3"]
pub struct SPIM1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1_S {}
impl SPIM1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0_ns::RegisterBlock {
        0x5000_9000 as *const _
    }
}
impl Deref for SPIM1_S {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM1_S::ptr() }
    }
}
#[doc = "SPI Slave 3"]
pub struct SPIS1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1_S {}
impl SPIS1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis0_ns::RegisterBlock {
        0x5000_9000 as *const _
    }
}
impl Deref for SPIS1_S {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIS1_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 3"]
pub struct TWIM1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1_S {}
impl TWIM1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0_ns::RegisterBlock {
        0x5000_9000 as *const _
    }
}
impl Deref for TWIM1_S {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM1_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 3"]
pub struct TWIS1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1_S {}
impl TWIS1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0_ns::RegisterBlock {
        0x5000_9000 as *const _
    }
}
impl Deref for TWIS1_S {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS1_S::ptr() }
    }
}
#[doc = "UART with EasyDMA 3"]
pub struct UARTE1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE1_S {}
impl UARTE1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarte0_ns::RegisterBlock {
        0x5000_9000 as *const _
    }
}
impl Deref for UARTE1_S {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTE1_S::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 4"]
pub struct SPIM2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM2_NS {}
impl SPIM2_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0_ns::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for SPIM2_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM2_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 5"]
pub struct SPIM2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM2_S {}
impl SPIM2_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0_ns::RegisterBlock {
        0x5000_a000 as *const _
    }
}
impl Deref for SPIM2_S {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM2_S::ptr() }
    }
}
#[doc = "GPIO Tasks and Events 0"]
pub struct GPIOTE0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE0_S {}
impl GPIOTE0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiote0_s::RegisterBlock {
        0x5000_d000 as *const _
    }
}
impl Deref for GPIOTE0_S {
    type Target = gpiote0_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOTE0_S::ptr() }
    }
}
#[doc = "GPIO Tasks and Events 0"]
pub mod gpiote0_s;
#[doc = "Analog to Digital Converter 0"]
pub struct SAADC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAADC_NS {}
impl SAADC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const saadc_ns::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for SAADC_NS {
    type Target = saadc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAADC_NS::ptr() }
    }
}
#[doc = "Analog to Digital Converter 0"]
pub mod saadc_ns;
#[doc = "Analog to Digital Converter 1"]
pub struct SAADC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAADC_S {}
impl SAADC_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const saadc_ns::RegisterBlock {
        0x5000_e000 as *const _
    }
}
impl Deref for SAADC_S {
    type Target = saadc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAADC_S::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub struct TIMER0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0_NS {}
impl TIMER0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x4000_f000 as *const _
    }
}
impl Deref for TIMER0_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0_NS::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod timer0_ns;
#[doc = "Timer/Counter 1"]
pub struct TIMER0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0_S {}
impl TIMER0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x5000_f000 as *const _
    }
}
impl Deref for TIMER0_S {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0_S::ptr() }
    }
}
#[doc = "Timer/Counter 2"]
pub struct TIMER1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1_NS {}
impl TIMER1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TIMER1_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1_NS::ptr() }
    }
}
#[doc = "Timer/Counter 3"]
pub struct TIMER1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1_S {}
impl TIMER1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x5001_0000 as *const _
    }
}
impl Deref for TIMER1_S {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1_S::ptr() }
    }
}
#[doc = "Timer/Counter 4"]
pub struct TIMER2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2_NS {}
impl TIMER2_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for TIMER2_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2_NS::ptr() }
    }
}
#[doc = "Timer/Counter 5"]
pub struct TIMER2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2_S {}
impl TIMER2_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x5001_1000 as *const _
    }
}
impl Deref for TIMER2_S {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2_S::ptr() }
    }
}
#[doc = "Real-time counter 0"]
pub struct RTC0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0_NS {}
impl RTC0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0_ns::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for RTC0_NS {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC0_NS::ptr() }
    }
}
#[doc = "Real-time counter 0"]
pub mod rtc0_ns;
#[doc = "Real-time counter 1"]
pub struct RTC0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0_S {}
impl RTC0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0_ns::RegisterBlock {
        0x5001_4000 as *const _
    }
}
impl Deref for RTC0_S {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC0_S::ptr() }
    }
}
#[doc = "Real-time counter 2"]
pub struct RTC1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1_NS {}
impl RTC1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0_ns::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for RTC1_NS {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC1_NS::ptr() }
    }
}
#[doc = "Real-time counter 3"]
pub struct RTC1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1_S {}
impl RTC1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0_ns::RegisterBlock {
        0x5001_5000 as *const _
    }
}
impl Deref for RTC1_S {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC1_S::ptr() }
    }
}
#[doc = "Distributed programmable peripheral interconnect controller 0"]
pub struct DPPIC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DPPIC_NS {}
impl DPPIC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dppic_ns::RegisterBlock {
        0x4001_7000 as *const _
    }
}
impl Deref for DPPIC_NS {
    type Target = dppic_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DPPIC_NS::ptr() }
    }
}
#[doc = "Distributed programmable peripheral interconnect controller 0"]
pub mod dppic_ns;
#[doc = "Distributed programmable peripheral interconnect controller 1"]
pub struct DPPIC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DPPIC_S {}
impl DPPIC_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dppic_ns::RegisterBlock {
        0x5001_7000 as *const _
    }
}
impl Deref for DPPIC_S {
    type Target = dppic_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DPPIC_S::ptr() }
    }
}
#[doc = "Watchdog Timer 0"]
pub struct WDT0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT0_NS {}
impl WDT0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt0_ns::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for WDT0_NS {
    type Target = wdt0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT0_NS::ptr() }
    }
}
#[doc = "Watchdog Timer 0"]
pub mod wdt0_ns;
#[doc = "Watchdog Timer 1"]
pub struct WDT0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT0_S {}
impl WDT0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt0_ns::RegisterBlock {
        0x5001_8000 as *const _
    }
}
impl Deref for WDT0_S {
    type Target = wdt0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT0_S::ptr() }
    }
}
#[doc = "Watchdog Timer 2"]
pub struct WDT1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT1_NS {}
impl WDT1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt0_ns::RegisterBlock {
        0x4001_9000 as *const _
    }
}
impl Deref for WDT1_NS {
    type Target = wdt0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT1_NS::ptr() }
    }
}
#[doc = "Watchdog Timer 3"]
pub struct WDT1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT1_S {}
impl WDT1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt0_ns::RegisterBlock {
        0x5001_9000 as *const _
    }
}
impl Deref for WDT1_S {
    type Target = wdt0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT1_S::ptr() }
    }
}
#[doc = "Comparator 0"]
pub struct COMP_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP_NS {}
impl COMP_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp_ns::RegisterBlock {
        0x4001_a000 as *const _
    }
}
impl Deref for COMP_NS {
    type Target = comp_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP_NS::ptr() }
    }
}
#[doc = "Comparator 0"]
pub mod comp_ns;
#[doc = "Low-power comparator 0"]
pub struct LPCOMP_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCOMP_NS {}
impl LPCOMP_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpcomp_ns::RegisterBlock {
        0x4001_a000 as *const _
    }
}
impl Deref for LPCOMP_NS {
    type Target = lpcomp_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPCOMP_NS::ptr() }
    }
}
#[doc = "Low-power comparator 0"]
pub mod lpcomp_ns;
#[doc = "Comparator 1"]
pub struct COMP_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP_S {}
impl COMP_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp_ns::RegisterBlock {
        0x5001_a000 as *const _
    }
}
impl Deref for COMP_S {
    type Target = comp_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP_S::ptr() }
    }
}
#[doc = "Low-power comparator 1"]
pub struct LPCOMP_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCOMP_S {}
impl LPCOMP_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpcomp_ns::RegisterBlock {
        0x5001_a000 as *const _
    }
}
impl Deref for LPCOMP_S {
    type Target = lpcomp_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPCOMP_S::ptr() }
    }
}
#[doc = "Event generator unit 0"]
pub struct EGU0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0_NS {}
impl EGU0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x4001_b000 as *const _
    }
}
impl Deref for EGU0_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU0_NS::ptr() }
    }
}
#[doc = "Event generator unit 0"]
pub mod egu0_ns;
#[doc = "Event generator unit 1"]
pub struct EGU0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0_S {}
impl EGU0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x5001_b000 as *const _
    }
}
impl Deref for EGU0_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU0_S::ptr() }
    }
}
#[doc = "Event generator unit 2"]
pub struct EGU1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU1_NS {}
impl EGU1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for EGU1_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU1_NS::ptr() }
    }
}
#[doc = "Event generator unit 3"]
pub struct EGU1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU1_S {}
impl EGU1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x5001_c000 as *const _
    }
}
impl Deref for EGU1_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU1_S::ptr() }
    }
}
#[doc = "Event generator unit 4"]
pub struct EGU2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU2_NS {}
impl EGU2_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x4001_d000 as *const _
    }
}
impl Deref for EGU2_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU2_NS::ptr() }
    }
}
#[doc = "Event generator unit 5"]
pub struct EGU2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU2_S {}
impl EGU2_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x5001_d000 as *const _
    }
}
impl Deref for EGU2_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU2_S::ptr() }
    }
}
#[doc = "Event generator unit 6"]
pub struct EGU3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU3_NS {}
impl EGU3_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x4001_e000 as *const _
    }
}
impl Deref for EGU3_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU3_NS::ptr() }
    }
}
#[doc = "Event generator unit 7"]
pub struct EGU3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU3_S {}
impl EGU3_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x5001_e000 as *const _
    }
}
impl Deref for EGU3_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU3_S::ptr() }
    }
}
#[doc = "Event generator unit 8"]
pub struct EGU4_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU4_NS {}
impl EGU4_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x4001_f000 as *const _
    }
}
impl Deref for EGU4_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU4_NS::ptr() }
    }
}
#[doc = "Event generator unit 9"]
pub struct EGU4_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU4_S {}
impl EGU4_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x5001_f000 as *const _
    }
}
impl Deref for EGU4_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU4_S::ptr() }
    }
}
#[doc = "Event generator unit 10"]
pub struct EGU5_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU5_NS {}
impl EGU5_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for EGU5_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU5_NS::ptr() }
    }
}
#[doc = "Event generator unit 11"]
pub struct EGU5_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU5_S {}
impl EGU5_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for EGU5_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU5_S::ptr() }
    }
}
#[doc = "Pulse width modulation unit 0"]
pub struct PWM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0_NS {}
impl PWM0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0_ns::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for PWM0_NS {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0_NS::ptr() }
    }
}
#[doc = "Pulse width modulation unit 0"]
pub mod pwm0_ns;
#[doc = "Pulse width modulation unit 1"]
pub struct PWM0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0_S {}
impl PWM0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0_ns::RegisterBlock {
        0x5002_1000 as *const _
    }
}
impl Deref for PWM0_S {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0_S::ptr() }
    }
}
#[doc = "Pulse width modulation unit 2"]
pub struct PWM1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1_NS {}
impl PWM1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0_ns::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for PWM1_NS {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1_NS::ptr() }
    }
}
#[doc = "Pulse width modulation unit 3"]
pub struct PWM1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1_S {}
impl PWM1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0_ns::RegisterBlock {
        0x5002_2000 as *const _
    }
}
impl Deref for PWM1_S {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1_S::ptr() }
    }
}
#[doc = "Pulse width modulation unit 4"]
pub struct PWM2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2_NS {}
impl PWM2_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0_ns::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for PWM2_NS {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM2_NS::ptr() }
    }
}
#[doc = "Pulse width modulation unit 5"]
pub struct PWM2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2_S {}
impl PWM2_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0_ns::RegisterBlock {
        0x5002_3000 as *const _
    }
}
impl Deref for PWM2_S {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM2_S::ptr() }
    }
}
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 0"]
pub struct PDM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM0_NS {}
impl PDM0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdm0_ns::RegisterBlock {
        0x4002_6000 as *const _
    }
}
impl Deref for PDM0_NS {
    type Target = pdm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDM0_NS::ptr() }
    }
}
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 0"]
pub mod pdm0_ns;
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 1"]
pub struct PDM0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM0_S {}
impl PDM0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdm0_ns::RegisterBlock {
        0x5002_6000 as *const _
    }
}
impl Deref for PDM0_S {
    type Target = pdm0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDM0_S::ptr() }
    }
}
#[doc = "Inter-IC Sound 0"]
pub struct I2S0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0_NS {}
impl I2S0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0_ns::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for I2S0_NS {
    type Target = i2s0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0_NS::ptr() }
    }
}
#[doc = "Inter-IC Sound 0"]
pub mod i2s0_ns;
#[doc = "Inter-IC Sound 1"]
pub struct I2S0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0_S {}
impl I2S0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0_ns::RegisterBlock {
        0x5002_8000 as *const _
    }
}
impl Deref for I2S0_S {
    type Target = i2s0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0_S::ptr() }
    }
}
#[doc = "Interprocessor communication 0"]
pub struct IPC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPC_NS {}
impl IPC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ipc_ns::RegisterBlock {
        0x4002_a000 as *const _
    }
}
impl Deref for IPC_NS {
    type Target = ipc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IPC_NS::ptr() }
    }
}
#[doc = "Interprocessor communication 0"]
pub mod ipc_ns;
#[doc = "Interprocessor communication 1"]
pub struct IPC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPC_S {}
impl IPC_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ipc_ns::RegisterBlock {
        0x5002_a000 as *const _
    }
}
impl Deref for IPC_S {
    type Target = ipc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IPC_S::ptr() }
    }
}
#[doc = "External flash interface 0"]
pub struct QSPI_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI_NS {}
impl QSPI_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi_ns::RegisterBlock {
        0x4002_b000 as *const _
    }
}
impl Deref for QSPI_NS {
    type Target = qspi_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI_NS::ptr() }
    }
}
#[doc = "External flash interface 0"]
pub mod qspi_ns;
#[doc = "External flash interface 1"]
pub struct QSPI_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI_S {}
impl QSPI_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi_ns::RegisterBlock {
        0x5002_b000 as *const _
    }
}
impl Deref for QSPI_S {
    type Target = qspi_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI_S::ptr() }
    }
}
#[doc = "NFC-A compatible radio 0"]
pub struct NFCT_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NFCT_NS {}
impl NFCT_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nfct_ns::RegisterBlock {
        0x4002_d000 as *const _
    }
}
impl Deref for NFCT_NS {
    type Target = nfct_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NFCT_NS::ptr() }
    }
}
#[doc = "NFC-A compatible radio 0"]
pub mod nfct_ns;
#[doc = "NFC-A compatible radio 1"]
pub struct NFCT_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NFCT_S {}
impl NFCT_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nfct_ns::RegisterBlock {
        0x5002_d000 as *const _
    }
}
impl Deref for NFCT_S {
    type Target = nfct_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NFCT_S::ptr() }
    }
}
#[doc = "GPIO Tasks and Events 1"]
pub struct GPIOTE1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE1_NS {}
impl GPIOTE1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiote0_s::RegisterBlock {
        0x4002_f000 as *const _
    }
}
impl Deref for GPIOTE1_NS {
    type Target = gpiote0_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOTE1_NS::ptr() }
    }
}
#[doc = "MUTEX 0"]
pub struct MUTEX_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MUTEX_NS {}
impl MUTEX_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mutex_ns::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for MUTEX_NS {
    type Target = mutex_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MUTEX_NS::ptr() }
    }
}
#[doc = "MUTEX 0"]
pub mod mutex_ns;
#[doc = "MUTEX 1"]
pub struct MUTEX_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MUTEX_S {}
impl MUTEX_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mutex_ns::RegisterBlock {
        0x5003_0000 as *const _
    }
}
impl Deref for MUTEX_S {
    type Target = mutex_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MUTEX_S::ptr() }
    }
}
#[doc = "Quadrature Decoder 0"]
pub struct QDEC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QDEC_NS {}
impl QDEC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qdec_ns::RegisterBlock {
        0x4003_3000 as *const _
    }
}
impl Deref for QDEC_NS {
    type Target = qdec_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QDEC_NS::ptr() }
    }
}
#[doc = "Quadrature Decoder 0"]
pub mod qdec_ns;
#[doc = "Quadrature Decoder 1"]
pub struct QDEC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QDEC_S {}
impl QDEC_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qdec_ns::RegisterBlock {
        0x5003_3000 as *const _
    }
}
impl Deref for QDEC_S {
    type Target = qdec_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QDEC_S::ptr() }
    }
}
#[doc = "Universal serial bus device 0"]
pub struct USBD_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBD_NS {}
impl USBD_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbd_ns::RegisterBlock {
        0x4003_6000 as *const _
    }
}
impl Deref for USBD_NS {
    type Target = usbd_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBD_NS::ptr() }
    }
}
#[doc = "Universal serial bus device 0"]
pub mod usbd_ns;
#[doc = "Universal serial bus device 1"]
pub struct USBD_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBD_S {}
impl USBD_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbd_ns::RegisterBlock {
        0x5003_6000 as *const _
    }
}
impl Deref for USBD_S {
    type Target = usbd_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBD_S::ptr() }
    }
}
#[doc = "USB Regulator 0"]
pub struct USBREGULATOR_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBREGULATOR_NS {}
impl USBREGULATOR_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbregulator_ns::RegisterBlock {
        0x4003_7000 as *const _
    }
}
impl Deref for USBREGULATOR_NS {
    type Target = usbregulator_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBREGULATOR_NS::ptr() }
    }
}
#[doc = "USB Regulator 0"]
pub mod usbregulator_ns;
#[doc = "USB Regulator 1"]
pub struct USBREGULATOR_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBREGULATOR_S {}
impl USBREGULATOR_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbregulator_ns::RegisterBlock {
        0x5003_7000 as *const _
    }
}
impl Deref for USBREGULATOR_S {
    type Target = usbregulator_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBREGULATOR_S::ptr() }
    }
}
#[doc = "Key management unit 0"]
pub struct KMU_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KMU_NS {}
impl KMU_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const kmu_ns::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for KMU_NS {
    type Target = kmu_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*KMU_NS::ptr() }
    }
}
#[doc = "Key management unit 0"]
pub mod kmu_ns;
#[doc = "Non-volatile memory controller 0"]
pub struct NVMC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC_NS {}
impl NVMC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmc_ns::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for NVMC_NS {
    type Target = nvmc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMC_NS::ptr() }
    }
}
#[doc = "Non-volatile memory controller 0"]
pub mod nvmc_ns;
#[doc = "Key management unit 1"]
pub struct KMU_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KMU_S {}
impl KMU_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const kmu_ns::RegisterBlock {
        0x5003_9000 as *const _
    }
}
impl Deref for KMU_S {
    type Target = kmu_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*KMU_S::ptr() }
    }
}
#[doc = "Non-volatile memory controller 1"]
pub struct NVMC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC_S {}
impl NVMC_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmc_ns::RegisterBlock {
        0x5003_9000 as *const _
    }
}
impl Deref for NVMC_S {
    type Target = nvmc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMC_S::ptr() }
    }
}
#[doc = "GPIO Port 0"]
pub struct P0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0_NS {}
impl P0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p0_ns::RegisterBlock {
        0x4084_2500 as *const _
    }
}
impl Deref for P0_NS {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P0_NS::ptr() }
    }
}
#[doc = "GPIO Port 0"]
pub mod p0_ns;
#[doc = "GPIO Port 1"]
pub struct P1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P1_NS {}
impl P1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p0_ns::RegisterBlock {
        0x4084_2800 as *const _
    }
}
impl Deref for P1_NS {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P1_NS::ptr() }
    }
}
#[doc = "GPIO Port 2"]
pub struct P0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0_S {}
impl P0_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p0_ns::RegisterBlock {
        0x5084_2500 as *const _
    }
}
impl Deref for P0_S {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P0_S::ptr() }
    }
}
#[doc = "GPIO Port 3"]
pub struct P1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P1_S {}
impl P1_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p0_ns::RegisterBlock {
        0x5084_2800 as *const _
    }
}
impl Deref for P1_S {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P1_S::ptr() }
    }
}
#[doc = "ARM TrustZone CryptoCell register interface"]
pub struct CRYPTOCELL_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTOCELL_S {}
impl CRYPTOCELL_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryptocell_s::RegisterBlock {
        0x5084_4000 as *const _
    }
}
impl Deref for CRYPTOCELL_S {
    type Target = cryptocell_s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYPTOCELL_S::ptr() }
    }
}
#[doc = "ARM TrustZone CryptoCell register interface"]
pub mod cryptocell_s;
#[doc = "Volatile Memory controller 0"]
pub struct VMC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VMC_NS {}
impl VMC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vmc_ns::RegisterBlock {
        0x4008_1000 as *const _
    }
}
impl Deref for VMC_NS {
    type Target = vmc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VMC_NS::ptr() }
    }
}
#[doc = "Volatile Memory controller 0"]
pub mod vmc_ns;
#[doc = "Volatile Memory controller 1"]
pub struct VMC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VMC_S {}
impl VMC_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vmc_ns::RegisterBlock {
        0x5008_1000 as *const _
    }
}
impl Deref for VMC_S {
    type Target = vmc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VMC_S::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CACHEDATA_S"]
    pub CACHEDATA_S: CACHEDATA_S,
    #[doc = "CACHEINFO_S"]
    pub CACHEINFO_S: CACHEINFO_S,
    #[doc = "FICR_S"]
    pub FICR_S: FICR_S,
    #[doc = "UICR_S"]
    pub UICR_S: UICR_S,
    #[doc = "CTI_S"]
    pub CTI_S: CTI_S,
    #[doc = "TAD_S"]
    pub TAD_S: TAD_S,
    #[doc = "DCNF_NS"]
    pub DCNF_NS: DCNF_NS,
    #[doc = "FPU_NS"]
    pub FPU_NS: FPU_NS,
    #[doc = "DCNF_S"]
    pub DCNF_S: DCNF_S,
    #[doc = "FPU_S"]
    pub FPU_S: FPU_S,
    #[doc = "CACHE_S"]
    pub CACHE_S: CACHE_S,
    #[doc = "SPU_S"]
    pub SPU_S: SPU_S,
    #[doc = "OSCILLATORS_NS"]
    pub OSCILLATORS_NS: OSCILLATORS_NS,
    #[doc = "REGULATORS_NS"]
    pub REGULATORS_NS: REGULATORS_NS,
    #[doc = "OSCILLATORS_S"]
    pub OSCILLATORS_S: OSCILLATORS_S,
    #[doc = "REGULATORS_S"]
    pub REGULATORS_S: REGULATORS_S,
    #[doc = "CLOCK_NS"]
    pub CLOCK_NS: CLOCK_NS,
    #[doc = "POWER_NS"]
    pub POWER_NS: POWER_NS,
    #[doc = "RESET_NS"]
    pub RESET_NS: RESET_NS,
    #[doc = "CLOCK_S"]
    pub CLOCK_S: CLOCK_S,
    #[doc = "POWER_S"]
    pub POWER_S: POWER_S,
    #[doc = "RESET_S"]
    pub RESET_S: RESET_S,
    #[doc = "CTRLAP_NS"]
    pub CTRLAP_NS: CTRLAP_NS,
    #[doc = "CTRLAP_S"]
    pub CTRLAP_S: CTRLAP_S,
    #[doc = "SPIM0_NS"]
    pub SPIM0_NS: SPIM0_NS,
    #[doc = "SPIS0_NS"]
    pub SPIS0_NS: SPIS0_NS,
    #[doc = "TWIM0_NS"]
    pub TWIM0_NS: TWIM0_NS,
    #[doc = "TWIS0_NS"]
    pub TWIS0_NS: TWIS0_NS,
    #[doc = "UARTE0_NS"]
    pub UARTE0_NS: UARTE0_NS,
    #[doc = "SPIM0_S"]
    pub SPIM0_S: SPIM0_S,
    #[doc = "SPIS0_S"]
    pub SPIS0_S: SPIS0_S,
    #[doc = "TWIM0_S"]
    pub TWIM0_S: TWIM0_S,
    #[doc = "TWIS0_S"]
    pub TWIS0_S: TWIS0_S,
    #[doc = "UARTE0_S"]
    pub UARTE0_S: UARTE0_S,
    #[doc = "SPIM1_NS"]
    pub SPIM1_NS: SPIM1_NS,
    #[doc = "SPIS1_NS"]
    pub SPIS1_NS: SPIS1_NS,
    #[doc = "TWIM1_NS"]
    pub TWIM1_NS: TWIM1_NS,
    #[doc = "TWIS1_NS"]
    pub TWIS1_NS: TWIS1_NS,
    #[doc = "UARTE1_NS"]
    pub UARTE1_NS: UARTE1_NS,
    #[doc = "SPIM1_S"]
    pub SPIM1_S: SPIM1_S,
    #[doc = "SPIS1_S"]
    pub SPIS1_S: SPIS1_S,
    #[doc = "TWIM1_S"]
    pub TWIM1_S: TWIM1_S,
    #[doc = "TWIS1_S"]
    pub TWIS1_S: TWIS1_S,
    #[doc = "UARTE1_S"]
    pub UARTE1_S: UARTE1_S,
    #[doc = "SPIM2_NS"]
    pub SPIM2_NS: SPIM2_NS,
    #[doc = "SPIM2_S"]
    pub SPIM2_S: SPIM2_S,
    #[doc = "GPIOTE0_S"]
    pub GPIOTE0_S: GPIOTE0_S,
    #[doc = "SAADC_NS"]
    pub SAADC_NS: SAADC_NS,
    #[doc = "SAADC_S"]
    pub SAADC_S: SAADC_S,
    #[doc = "TIMER0_NS"]
    pub TIMER0_NS: TIMER0_NS,
    #[doc = "TIMER0_S"]
    pub TIMER0_S: TIMER0_S,
    #[doc = "TIMER1_NS"]
    pub TIMER1_NS: TIMER1_NS,
    #[doc = "TIMER1_S"]
    pub TIMER1_S: TIMER1_S,
    #[doc = "TIMER2_NS"]
    pub TIMER2_NS: TIMER2_NS,
    #[doc = "TIMER2_S"]
    pub TIMER2_S: TIMER2_S,
    #[doc = "RTC0_NS"]
    pub RTC0_NS: RTC0_NS,
    #[doc = "RTC0_S"]
    pub RTC0_S: RTC0_S,
    #[doc = "RTC1_NS"]
    pub RTC1_NS: RTC1_NS,
    #[doc = "RTC1_S"]
    pub RTC1_S: RTC1_S,
    #[doc = "DPPIC_NS"]
    pub DPPIC_NS: DPPIC_NS,
    #[doc = "DPPIC_S"]
    pub DPPIC_S: DPPIC_S,
    #[doc = "WDT0_NS"]
    pub WDT0_NS: WDT0_NS,
    #[doc = "WDT0_S"]
    pub WDT0_S: WDT0_S,
    #[doc = "WDT1_NS"]
    pub WDT1_NS: WDT1_NS,
    #[doc = "WDT1_S"]
    pub WDT1_S: WDT1_S,
    #[doc = "COMP_NS"]
    pub COMP_NS: COMP_NS,
    #[doc = "LPCOMP_NS"]
    pub LPCOMP_NS: LPCOMP_NS,
    #[doc = "COMP_S"]
    pub COMP_S: COMP_S,
    #[doc = "LPCOMP_S"]
    pub LPCOMP_S: LPCOMP_S,
    #[doc = "EGU0_NS"]
    pub EGU0_NS: EGU0_NS,
    #[doc = "EGU0_S"]
    pub EGU0_S: EGU0_S,
    #[doc = "EGU1_NS"]
    pub EGU1_NS: EGU1_NS,
    #[doc = "EGU1_S"]
    pub EGU1_S: EGU1_S,
    #[doc = "EGU2_NS"]
    pub EGU2_NS: EGU2_NS,
    #[doc = "EGU2_S"]
    pub EGU2_S: EGU2_S,
    #[doc = "EGU3_NS"]
    pub EGU3_NS: EGU3_NS,
    #[doc = "EGU3_S"]
    pub EGU3_S: EGU3_S,
    #[doc = "EGU4_NS"]
    pub EGU4_NS: EGU4_NS,
    #[doc = "EGU4_S"]
    pub EGU4_S: EGU4_S,
    #[doc = "EGU5_NS"]
    pub EGU5_NS: EGU5_NS,
    #[doc = "EGU5_S"]
    pub EGU5_S: EGU5_S,
    #[doc = "PWM0_NS"]
    pub PWM0_NS: PWM0_NS,
    #[doc = "PWM0_S"]
    pub PWM0_S: PWM0_S,
    #[doc = "PWM1_NS"]
    pub PWM1_NS: PWM1_NS,
    #[doc = "PWM1_S"]
    pub PWM1_S: PWM1_S,
    #[doc = "PWM2_NS"]
    pub PWM2_NS: PWM2_NS,
    #[doc = "PWM2_S"]
    pub PWM2_S: PWM2_S,
    #[doc = "PDM0_NS"]
    pub PDM0_NS: PDM0_NS,
    #[doc = "PDM0_S"]
    pub PDM0_S: PDM0_S,
    #[doc = "I2S0_NS"]
    pub I2S0_NS: I2S0_NS,
    #[doc = "I2S0_S"]
    pub I2S0_S: I2S0_S,
    #[doc = "IPC_NS"]
    pub IPC_NS: IPC_NS,
    #[doc = "IPC_S"]
    pub IPC_S: IPC_S,
    #[doc = "QSPI_NS"]
    pub QSPI_NS: QSPI_NS,
    #[doc = "QSPI_S"]
    pub QSPI_S: QSPI_S,
    #[doc = "NFCT_NS"]
    pub NFCT_NS: NFCT_NS,
    #[doc = "NFCT_S"]
    pub NFCT_S: NFCT_S,
    #[doc = "GPIOTE1_NS"]
    pub GPIOTE1_NS: GPIOTE1_NS,
    #[doc = "MUTEX_NS"]
    pub MUTEX_NS: MUTEX_NS,
    #[doc = "MUTEX_S"]
    pub MUTEX_S: MUTEX_S,
    #[doc = "QDEC_NS"]
    pub QDEC_NS: QDEC_NS,
    #[doc = "QDEC_S"]
    pub QDEC_S: QDEC_S,
    #[doc = "USBD_NS"]
    pub USBD_NS: USBD_NS,
    #[doc = "USBD_S"]
    pub USBD_S: USBD_S,
    #[doc = "USBREGULATOR_NS"]
    pub USBREGULATOR_NS: USBREGULATOR_NS,
    #[doc = "USBREGULATOR_S"]
    pub USBREGULATOR_S: USBREGULATOR_S,
    #[doc = "KMU_NS"]
    pub KMU_NS: KMU_NS,
    #[doc = "NVMC_NS"]
    pub NVMC_NS: NVMC_NS,
    #[doc = "KMU_S"]
    pub KMU_S: KMU_S,
    #[doc = "NVMC_S"]
    pub NVMC_S: NVMC_S,
    #[doc = "P0_NS"]
    pub P0_NS: P0_NS,
    #[doc = "P1_NS"]
    pub P1_NS: P1_NS,
    #[doc = "P0_S"]
    pub P0_S: P0_S,
    #[doc = "P1_S"]
    pub P1_S: P1_S,
    #[doc = "CRYPTOCELL_S"]
    pub CRYPTOCELL_S: CRYPTOCELL_S,
    #[doc = "VMC_NS"]
    pub VMC_NS: VMC_NS,
    #[doc = "VMC_S"]
    pub VMC_S: VMC_S,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
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
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CACHEDATA_S: CACHEDATA_S {
                _marker: PhantomData,
            },
            CACHEINFO_S: CACHEINFO_S {
                _marker: PhantomData,
            },
            FICR_S: FICR_S {
                _marker: PhantomData,
            },
            UICR_S: UICR_S {
                _marker: PhantomData,
            },
            CTI_S: CTI_S {
                _marker: PhantomData,
            },
            TAD_S: TAD_S {
                _marker: PhantomData,
            },
            DCNF_NS: DCNF_NS {
                _marker: PhantomData,
            },
            FPU_NS: FPU_NS {
                _marker: PhantomData,
            },
            DCNF_S: DCNF_S {
                _marker: PhantomData,
            },
            FPU_S: FPU_S {
                _marker: PhantomData,
            },
            CACHE_S: CACHE_S {
                _marker: PhantomData,
            },
            SPU_S: SPU_S {
                _marker: PhantomData,
            },
            OSCILLATORS_NS: OSCILLATORS_NS {
                _marker: PhantomData,
            },
            REGULATORS_NS: REGULATORS_NS {
                _marker: PhantomData,
            },
            OSCILLATORS_S: OSCILLATORS_S {
                _marker: PhantomData,
            },
            REGULATORS_S: REGULATORS_S {
                _marker: PhantomData,
            },
            CLOCK_NS: CLOCK_NS {
                _marker: PhantomData,
            },
            POWER_NS: POWER_NS {
                _marker: PhantomData,
            },
            RESET_NS: RESET_NS {
                _marker: PhantomData,
            },
            CLOCK_S: CLOCK_S {
                _marker: PhantomData,
            },
            POWER_S: POWER_S {
                _marker: PhantomData,
            },
            RESET_S: RESET_S {
                _marker: PhantomData,
            },
            CTRLAP_NS: CTRLAP_NS {
                _marker: PhantomData,
            },
            CTRLAP_S: CTRLAP_S {
                _marker: PhantomData,
            },
            SPIM0_NS: SPIM0_NS {
                _marker: PhantomData,
            },
            SPIS0_NS: SPIS0_NS {
                _marker: PhantomData,
            },
            TWIM0_NS: TWIM0_NS {
                _marker: PhantomData,
            },
            TWIS0_NS: TWIS0_NS {
                _marker: PhantomData,
            },
            UARTE0_NS: UARTE0_NS {
                _marker: PhantomData,
            },
            SPIM0_S: SPIM0_S {
                _marker: PhantomData,
            },
            SPIS0_S: SPIS0_S {
                _marker: PhantomData,
            },
            TWIM0_S: TWIM0_S {
                _marker: PhantomData,
            },
            TWIS0_S: TWIS0_S {
                _marker: PhantomData,
            },
            UARTE0_S: UARTE0_S {
                _marker: PhantomData,
            },
            SPIM1_NS: SPIM1_NS {
                _marker: PhantomData,
            },
            SPIS1_NS: SPIS1_NS {
                _marker: PhantomData,
            },
            TWIM1_NS: TWIM1_NS {
                _marker: PhantomData,
            },
            TWIS1_NS: TWIS1_NS {
                _marker: PhantomData,
            },
            UARTE1_NS: UARTE1_NS {
                _marker: PhantomData,
            },
            SPIM1_S: SPIM1_S {
                _marker: PhantomData,
            },
            SPIS1_S: SPIS1_S {
                _marker: PhantomData,
            },
            TWIM1_S: TWIM1_S {
                _marker: PhantomData,
            },
            TWIS1_S: TWIS1_S {
                _marker: PhantomData,
            },
            UARTE1_S: UARTE1_S {
                _marker: PhantomData,
            },
            SPIM2_NS: SPIM2_NS {
                _marker: PhantomData,
            },
            SPIM2_S: SPIM2_S {
                _marker: PhantomData,
            },
            GPIOTE0_S: GPIOTE0_S {
                _marker: PhantomData,
            },
            SAADC_NS: SAADC_NS {
                _marker: PhantomData,
            },
            SAADC_S: SAADC_S {
                _marker: PhantomData,
            },
            TIMER0_NS: TIMER0_NS {
                _marker: PhantomData,
            },
            TIMER0_S: TIMER0_S {
                _marker: PhantomData,
            },
            TIMER1_NS: TIMER1_NS {
                _marker: PhantomData,
            },
            TIMER1_S: TIMER1_S {
                _marker: PhantomData,
            },
            TIMER2_NS: TIMER2_NS {
                _marker: PhantomData,
            },
            TIMER2_S: TIMER2_S {
                _marker: PhantomData,
            },
            RTC0_NS: RTC0_NS {
                _marker: PhantomData,
            },
            RTC0_S: RTC0_S {
                _marker: PhantomData,
            },
            RTC1_NS: RTC1_NS {
                _marker: PhantomData,
            },
            RTC1_S: RTC1_S {
                _marker: PhantomData,
            },
            DPPIC_NS: DPPIC_NS {
                _marker: PhantomData,
            },
            DPPIC_S: DPPIC_S {
                _marker: PhantomData,
            },
            WDT0_NS: WDT0_NS {
                _marker: PhantomData,
            },
            WDT0_S: WDT0_S {
                _marker: PhantomData,
            },
            WDT1_NS: WDT1_NS {
                _marker: PhantomData,
            },
            WDT1_S: WDT1_S {
                _marker: PhantomData,
            },
            COMP_NS: COMP_NS {
                _marker: PhantomData,
            },
            LPCOMP_NS: LPCOMP_NS {
                _marker: PhantomData,
            },
            COMP_S: COMP_S {
                _marker: PhantomData,
            },
            LPCOMP_S: LPCOMP_S {
                _marker: PhantomData,
            },
            EGU0_NS: EGU0_NS {
                _marker: PhantomData,
            },
            EGU0_S: EGU0_S {
                _marker: PhantomData,
            },
            EGU1_NS: EGU1_NS {
                _marker: PhantomData,
            },
            EGU1_S: EGU1_S {
                _marker: PhantomData,
            },
            EGU2_NS: EGU2_NS {
                _marker: PhantomData,
            },
            EGU2_S: EGU2_S {
                _marker: PhantomData,
            },
            EGU3_NS: EGU3_NS {
                _marker: PhantomData,
            },
            EGU3_S: EGU3_S {
                _marker: PhantomData,
            },
            EGU4_NS: EGU4_NS {
                _marker: PhantomData,
            },
            EGU4_S: EGU4_S {
                _marker: PhantomData,
            },
            EGU5_NS: EGU5_NS {
                _marker: PhantomData,
            },
            EGU5_S: EGU5_S {
                _marker: PhantomData,
            },
            PWM0_NS: PWM0_NS {
                _marker: PhantomData,
            },
            PWM0_S: PWM0_S {
                _marker: PhantomData,
            },
            PWM1_NS: PWM1_NS {
                _marker: PhantomData,
            },
            PWM1_S: PWM1_S {
                _marker: PhantomData,
            },
            PWM2_NS: PWM2_NS {
                _marker: PhantomData,
            },
            PWM2_S: PWM2_S {
                _marker: PhantomData,
            },
            PDM0_NS: PDM0_NS {
                _marker: PhantomData,
            },
            PDM0_S: PDM0_S {
                _marker: PhantomData,
            },
            I2S0_NS: I2S0_NS {
                _marker: PhantomData,
            },
            I2S0_S: I2S0_S {
                _marker: PhantomData,
            },
            IPC_NS: IPC_NS {
                _marker: PhantomData,
            },
            IPC_S: IPC_S {
                _marker: PhantomData,
            },
            QSPI_NS: QSPI_NS {
                _marker: PhantomData,
            },
            QSPI_S: QSPI_S {
                _marker: PhantomData,
            },
            NFCT_NS: NFCT_NS {
                _marker: PhantomData,
            },
            NFCT_S: NFCT_S {
                _marker: PhantomData,
            },
            GPIOTE1_NS: GPIOTE1_NS {
                _marker: PhantomData,
            },
            MUTEX_NS: MUTEX_NS {
                _marker: PhantomData,
            },
            MUTEX_S: MUTEX_S {
                _marker: PhantomData,
            },
            QDEC_NS: QDEC_NS {
                _marker: PhantomData,
            },
            QDEC_S: QDEC_S {
                _marker: PhantomData,
            },
            USBD_NS: USBD_NS {
                _marker: PhantomData,
            },
            USBD_S: USBD_S {
                _marker: PhantomData,
            },
            USBREGULATOR_NS: USBREGULATOR_NS {
                _marker: PhantomData,
            },
            USBREGULATOR_S: USBREGULATOR_S {
                _marker: PhantomData,
            },
            KMU_NS: KMU_NS {
                _marker: PhantomData,
            },
            NVMC_NS: NVMC_NS {
                _marker: PhantomData,
            },
            KMU_S: KMU_S {
                _marker: PhantomData,
            },
            NVMC_S: NVMC_S {
                _marker: PhantomData,
            },
            P0_NS: P0_NS {
                _marker: PhantomData,
            },
            P1_NS: P1_NS {
                _marker: PhantomData,
            },
            P0_S: P0_S {
                _marker: PhantomData,
            },
            P1_S: P1_S {
                _marker: PhantomData,
            },
            CRYPTOCELL_S: CRYPTOCELL_S {
                _marker: PhantomData,
            },
            VMC_NS: VMC_NS {
                _marker: PhantomData,
            },
            VMC_S: VMC_S {
                _marker: PhantomData,
            },
        }
    }
}
