#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Executor;
use embassy_time::{Duration, Timer};
use esp32c2_hal::{
    clock::ClockControl,
    embassy,
    pac::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Rtc,
};
use esp_backtrace as _;
use static_cell::StaticCell;

#[embassy_executor::task]
async fn run1() {
    loop {
        esp_println::println!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[embassy_executor::task]
async fn run2() {
    loop {
        esp_println::println!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[riscv_rt::entry]
fn main() -> ! {
    esp_println::println!("Init!");
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    embassy::init(&clocks);

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();

    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(run1()).ok();
        spawner.spawn(run2()).ok();
    });
}
