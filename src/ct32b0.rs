#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register (IR). The IR can be written to clear interrupts. The IR can be read to identify which of five possible interrupt sources are pending"]
    pub ir: IR,
    #[doc = "0x04 - Timer Control Register (TCR). The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR"]
    pub tcr: TCR,
    #[doc = "0x08 - Timer Counter (TC). The 32-bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR"]
    pub tc: TC,
    #[doc = "0x0c - Prescale Register (PR). When the Prescale Counter (below) is equal to this value, the next clock increments the TC and clears the PC"]
    pub pr: PR,
    #[doc = "0x10 - Prescale Counter (PC). The 32-bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface"]
    pub pc: PC,
    #[doc = "0x14 - Match Control Register (MCR). The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs"]
    pub mcr: MCR,
    #[doc = "0x18 - Match Register. MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC"]
    pub mr: [MR; 4],
    #[doc = "0x28 - Capture Control Register (CCR). The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place"]
    pub ccr: CCR,
    #[doc = "0x2c - Capture Register (CR). CR is loaded with the value of TC when there is an event on the CT16Bn_CAPm input"]
    pub cr: [CR; 2],
    _reserved9: [u8; 8usize],
    #[doc = "0x3c - External Match Register (EMR). The EMR controls the match function and the external match pins CT32B0_MAT\\[3:0\\]"]
    pub emr: EMR,
    _reserved10: [u8; 48usize],
    #[doc = "0x70 - Count Control Register (CTCR). The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting"]
    pub ctcr: CTCR,
    #[doc = "0x74 - PWM Control Register (PWMCON). The PWMCON enables PWM mode for the external match pins CT32B0_MAT\\[3:0\\]"]
    pub pwmc: PWMC,
}
#[doc = "Interrupt Register (IR). The IR can be written to clear interrupts. The IR can be read to identify which of five possible interrupt sources are pending"]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register (IR). The IR can be written to clear interrupts. The IR can be read to identify which of five possible interrupt sources are pending"]
pub mod ir;
#[doc = "Timer Control Register (TCR). The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register (TCR). The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR"]
pub mod tcr;
#[doc = "Timer Counter (TC). The 32-bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR"]
pub struct TC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Counter (TC). The 32-bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR"]
pub mod tc;
#[doc = "Prescale Register (PR). When the Prescale Counter (below) is equal to this value, the next clock increments the TC and clears the PC"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescale Register (PR). When the Prescale Counter (below) is equal to this value, the next clock increments the TC and clears the PC"]
pub mod pr;
#[doc = "Prescale Counter (PC). The 32-bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface"]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescale Counter (PC). The 32-bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface"]
pub mod pc;
#[doc = "Match Control Register (MCR). The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Control Register (MCR). The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs"]
pub mod mcr;
#[doc = "Match Register. MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Register. MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC"]
pub mod mr;
#[doc = "Capture Control Register (CCR). The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Control Register (CCR). The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place"]
pub mod ccr;
#[doc = "Capture Register (CR). CR is loaded with the value of TC when there is an event on the CT16Bn_CAPm input"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register (CR). CR is loaded with the value of TC when there is an event on the CT16Bn_CAPm input"]
pub mod cr;
#[doc = "External Match Register (EMR). The EMR controls the match function and the external match pins CT32B0_MAT\\[3:0\\]"]
pub struct EMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Match Register (EMR). The EMR controls the match function and the external match pins CT32B0_MAT\\[3:0\\]"]
pub mod emr;
#[doc = "Count Control Register (CTCR). The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting"]
pub struct CTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Control Register (CTCR). The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting"]
pub mod ctcr;
#[doc = "PWM Control Register (PWMCON). The PWMCON enables PWM mode for the external match pins CT32B0_MAT\\[3:0\\]"]
pub struct PWMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control Register (PWMCON). The PWMCON enables PWM mode for the external match pins CT32B0_MAT\\[3:0\\]"]
pub mod pwmc;
