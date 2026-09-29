#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use core::ptr::{write_volatile, read_volatile};

const RCC_BASE: u32 = 0x4002_3800;
const RCC_AHB1ENR: u32 = RCC_BASE + 0x30;
const GPIODEN_BIT: u32 = 3;

const GPIOD_BASE: u32 = 0x4002_0C00;
const GPIOD_MODER: u32 = GPIOD_BASE + 0x00;
const GPIOD_MODER_PIN12_SHIFT: u32 = 24;

const GPIOD_BSRR: u32 = GPIOD_BASE + 0x18;
const GPIOD_BSRR_BS12: u32 = 1<<12;
const GPIOD_BSRR_BR12: u32 = 1<<28;

const SYSTICK_BASE: u32 = 0xE000_E010;
const SYST_CSR: u32 = SYSTICK_BASE + 0x00;
const COUNTFLAG_BIT: u32 = 16;
const SYST_RVR: u32 = SYSTICK_BASE + 0x04;
const SYST_CVR: u32 = SYSTICK_BASE + 0x08;

#[entry]
fn main() -> !{
    let before_clock = read_reg(RCC_AHB1ENR);
    write_reg(RCC_AHB1ENR, before_clock | (1 << GPIODEN_BIT));

    let before_pinmode = read_reg(GPIOD_MODER);
    let clear_mask = !(0b11<<GPIOD_MODER_PIN12_SHIFT);
    let new_pinmode = 0b01<<GPIOD_MODER_PIN12_SHIFT;
    write_reg(GPIOD_MODER, (before_pinmode & clear_mask) | new_pinmode);
    write_reg(GPIOD_BSRR, GPIOD_BSRR_BS12);

    write_reg(SYST_RVR, 0x00F4_23FF);
    write_reg(SYST_CVR, 0);
    let enable_core_clock = 0b101;
    write_reg(SYST_CSR, enable_core_clock);
    wait();
    write_reg(GPIOD_BSRR, GPIOD_BSRR_BR12);
    loop {}
}

fn write_reg(addr: u32, val: u32){
    unsafe {write_volatile(addr as *mut u32, val)}
}

fn read_reg(addr: u32) -> u32{
    unsafe {read_volatile(addr as *const u32)}
}

fn wait(){
    loop{
        let csr = read_reg(SYST_CSR);
        if csr & (1<<COUNTFLAG_BIT) != 0{
            break
        }
    }
}