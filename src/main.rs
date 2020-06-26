#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
//use cortex_m_semihosting::dbg;
use stm32f4::stm32f401;

use clock::systick::SysTick;

use clock::matrix::display_led;
use clock::matrix::display_led::DisplayLed;
use clock::print_led;

use clock::rtc::DateTime;
use clock::rtc::Rtc;

use core::cell::RefCell;
static WAKE_TIMER: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[entry]
fn main() -> ! {
    let device = stm32f401::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    init_clock(&device);
    gpio_setup(&device);
    const SYS_FQ: u32 = 48_000_000;
    let systick = free(|cs| {
        let s = SysTick::new(cs, core.SYST, SYS_FQ, 10);
        s.add_callback_function(cs, tim_sys);
        s.add_callback_function(cs, count_sw3_pushing);
        s
    });

    let mut led = DisplayLed::new(&device);
    let rtc = Rtc::new(&device);
    if !rtc.time_set_done() {
        let date = DateTime {
            year: 20,
            month: 6,
            date: 5,
            hour: 20,
            min: 46,
            sec: 0,
        };
        Rtc::set_time_pre(&device, &date);
        Rtc::exec_set_time(&device);
    }

    free(|cs| *WAKE_TIMER.borrow(cs).borrow_mut() = true);
    free(|cs| systick.enable_counter(cs));

    let mut ctr_clock = ControllerClock::new(&device, &rtc, &mut led);
    loop {
        if free(|cs| *WAKE_TIMER.borrow(cs).borrow()) {
            ctr_clock.run();
            free(|cs| *WAKE_TIMER.borrow(cs).borrow_mut() = false);
        }

        device.GPIOA.bsrr.write(|w| w.br1().reset());
        cortex_m::asm::wfi();
        device.GPIOA.bsrr.write(|w| w.bs1().set());
    }
}

fn tim_sys() {
    free(|cs| {
        *WAKE_TIMER.borrow(cs).borrow_mut() = true;
    });
}

enum AdjModeKind {
    Off,
    Start,
    Year,
    Month,
    Day,
    Hour,
    Min,
    Sec,
}

struct ControllerClock<'a> {
    led: &'a mut DisplayLed<'a>,
    rtc: &'a Rtc<'a>,
    device: &'a stm32f401::Peripherals,
    old_sec: u32,
    cur_adj_mode: AdjModeKind,
    adj_date: (u32, u32, u32, u32),
    adj_time: (u32, u32, u32),
}

static PUSH_SW1: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
static PUSH_SW2: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
static PUSH_SW3: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
static HOLD_SW3: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
static HOLD_SW3_COUNT: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

impl ControllerClock<'_> {
    pub fn new<'a>(
        device: &'a stm32f401::Peripherals,
        rtc: &'a Rtc,
        led: &'a mut DisplayLed<'a>,
    ) -> ControllerClock<'a> {
        ControllerClock {
            led,
            rtc,
            device,
            old_sec: 0,
            cur_adj_mode: AdjModeKind::Off,
            adj_date: (0, 0, 0, 0),
            adj_time: (0, 0, 0),
        }
    }

    pub fn run(&mut self) {
        if let AdjModeKind::Off = self.cur_adj_mode {
            if !free(|cs| *HOLD_SW3.borrow(cs).borrow()) {
                self.display_clock();
            } else {
                free(|cs| {
                    self.cur_adj_mode = AdjModeKind::Start;
                    *HOLD_SW3.borrow(cs).borrow_mut() = false;
                });
            }
            if self.device.GPIOA.idr.read().idr12().bits() {
                free(|cs| *PUSH_SW3.borrow(cs).borrow_mut() = false);
            }
        } else {
            self.adj_clock();
        }
    }

    fn adj_clock(&mut self) {
        match self.cur_adj_mode {
            AdjModeKind::Start => {
                print_led!(self.led, "adj_mode\n");
                if self.is_sw3_release() {
                    self.cur_adj_mode = AdjModeKind::Year;
                }
                self.adj_date = self.rtc.get_date();
                self.adj_time = self.rtc.get_time();
            }
            AdjModeKind::Year => {
                print_led!(self.led, "year:{:>02}\n", self.adj_date.0 - 2000);
                self.adj_date.0 = self.adj_num_change(self.adj_date.0, 2100, 2000);
                if self.is_sw3_release() {
                    self.cur_adj_mode = AdjModeKind::Month;
                }
            }
            AdjModeKind::Month => {
                print_led!(self.led, "mon:{:>02}\n", self.adj_date.1);
                self.adj_date.1 = self.adj_num_change(self.adj_date.1, 12, 1);
                if self.is_sw3_release() {
                    self.cur_adj_mode = AdjModeKind::Day;
                }
            }
            AdjModeKind::Day => {
                print_led!(self.led, "day:{:>02}\n", self.adj_date.2);
                self.adj_date.2 = self.adj_num_change(self.adj_date.2, 31, 1);
                if self.is_sw3_release() {
                    self.cur_adj_mode = AdjModeKind::Hour;
                }
            }
            AdjModeKind::Hour => {
                print_led!(self.led, "hour:{:>2}\n", self.adj_time.0);
                self.adj_time.0 = self.adj_num_change(self.adj_time.0, 23, 0);
                if self.is_sw3_release() {
                    self.cur_adj_mode = AdjModeKind::Min;
                }
            }
            AdjModeKind::Min => {
                print_led!(self.led, "min:{:>2}\n", self.adj_time.1);
                self.adj_time.1 = self.adj_num_change(self.adj_time.1, 59, 0);
                if self.is_sw3_release() {
                    self.cur_adj_mode = AdjModeKind::Sec;
                }
            }
            AdjModeKind::Sec => {
                print_led!(self.led, "sec:{:>2}\n", self.adj_time.2);
                self.adj_time.2 = self.adj_num_change(self.adj_time.2, 59, 0);
                if self.is_sw3_release() {
                    let date = DateTime {
                        year: (self.adj_date.0 - 2000) as u8,
                        month: self.adj_date.1 as u8,
                        date: self.adj_date.2 as u8,
                        hour: self.adj_time.0 as u8,
                        min: self.adj_time.1 as u8,
                        sec: self.adj_time.2 as u8,
                    };
                    Rtc::set_time_pre(&self.device, &date);
                    Rtc::exec_set_time(&self.device);
                    self.cur_adj_mode = AdjModeKind::Off;
                }
            }
            _ => {
                if self.is_sw3_release() {
                    self.cur_adj_mode = AdjModeKind::Off;
                }
            }
        }

        if self.device.GPIOA.idr.read().idr12().bits() {
            free(|cs| {
                *PUSH_SW3.borrow(cs).borrow_mut() = false;
                *HOLD_SW3.borrow(cs).borrow_mut() = false;
            });
        }
        if self.is_sw1_release() {
            free(|cs| *PUSH_SW1.borrow(cs).borrow_mut() = false);
        }
        if self.is_sw2_release() {
            free(|cs| *PUSH_SW2.borrow(cs).borrow_mut() = false);
        }
    }

    fn adj_num_change(&self, value: u32, max: u32, min: u32) -> u32 {
        let mut ret = value as i32;
        if self.is_sw1_release() {
            ret += 1;
        }
        if self.is_sw2_release() {
            ret -= 1;
        }
        if ret > max as i32 {
            ret = min as i32;
        } else if ret < min as i32 {
            ret = max as i32;
        }
        ret as u32
    }

    fn display_clock(&mut self) {
        if free(|cs| *PUSH_SW1.borrow(cs).borrow()) {
            if self.device.GPIOA.idr.read().idr8().bits() {
                free(|cs| *PUSH_SW1.borrow(cs).borrow_mut() = false);
            } else {
                self.display_date();
            }
        } else if free(|cs| *PUSH_SW2.borrow(cs).borrow()) {
            if self.device.GPIOA.idr.read().idr15().bits() {
                free(|cs| *PUSH_SW2.borrow(cs).borrow_mut() = false);
            } else {
                self.display_week();
            }
        } else {
            self.display_time();
        }
    }

    fn is_sw1_release(&self) -> bool {
        free(|cs| *PUSH_SW1.borrow(cs).borrow()) && self.device.GPIOA.idr.read().idr8().bits()
    }

    fn is_sw2_release(&self) -> bool {
        free(|cs| *PUSH_SW2.borrow(cs).borrow()) && self.device.GPIOA.idr.read().idr15().bits()
    }

    fn is_sw3_release(&self) -> bool {
        free(|cs| *PUSH_SW3.borrow(cs).borrow()) && self.device.GPIOA.idr.read().idr12().bits()
    }

    fn display_time(&mut self) {
        let (hh, mm, ss) = self.rtc.get_time();
        if self.old_sec != ss {
            self.old_sec = ss;
            print_led!(self.led, "{:>2}:{:>02}:{:>02}\n", hh, mm, ss);
        }
    }

    fn display_date(&mut self) {
        let (yy, mm, dd, _) = self.rtc.get_date();
        let yy = yy - 2000;
        print_led!(self.led, "{:>02}/{:>2}/{:>2}\n", yy, mm, dd);
    }

    fn display_week(&mut self) {
        let (_, _, _, w) = self.rtc.get_date();
        let week_name = match w {
            1 => "Mon.",
            2 => "Tue.",
            3 => "Wed.",
            4 => "Thu.",
            5 => "Fri.",
            6 => "Sat.",
            7 => "Sun.",
            _ => "None",
        };
        print_led!(self.led, "{}\n", week_name);
    }
}

use stm32f401::interrupt;
/// SW1(EXTI8)の押下タイミング
#[interrupt]
fn EXTI9_5() {
    free(|cs| {
        let device = unsafe { stm32f401::Peripherals::steal() };
        if device.EXTI.pr.read().pr8().bits() {
            device.EXTI.pr.modify(|_, w| w.pr8().clear());
            *PUSH_SW1.borrow(cs).borrow_mut() = true;
        }
    });
}

/// SW2(EXTI15) SW3(EXTI12)の押下タイミング
#[interrupt]
fn EXTI15_10() {
    free(|cs| {
        let device = unsafe { stm32f401::Peripherals::steal() };
        if device.EXTI.pr.read().pr15().bits() {
            device.EXTI.pr.modify(|_, w| w.pr15().clear());
            *PUSH_SW2.borrow(cs).borrow_mut() = true;
        } else if device.EXTI.pr.read().pr12().bits() {
            device.EXTI.pr.modify(|_, w| w.pr12().clear());
            if !device.GPIOA.idr.read().idr12().bits() {
                *PUSH_SW3.borrow(cs).borrow_mut() = true;
            } else {
                *HOLD_SW3_COUNT.borrow(cs).borrow_mut() = 0;
            }
        }
    });
}

/// SW3の長押し判定
fn count_sw3_pushing() {
    free(|cs| {
        if *PUSH_SW3.borrow(cs).borrow() {
            *HOLD_SW3_COUNT.borrow(cs).borrow_mut() += 1;
            if *HOLD_SW3_COUNT.borrow(cs).borrow() > 20 {
                *HOLD_SW3.borrow(cs).borrow_mut() = true;
            }
        }
    });
}

/// システムクロックの初期設定
/// 　クロック周波数　48MHz
fn init_clock(device: &stm32f401::Peripherals) {
    // システムクロック　48MHz
    // PLLCFGR設定
    // hsi(16M)/8*192/8=48MHz
    {
        let pllcfgr = &device.RCC.pllcfgr;
        pllcfgr.modify(|_, w| w.pllsrc().hsi());
        pllcfgr.modify(|_, w| w.pllp().div8());
        pllcfgr.modify(|_, w| unsafe { w.plln().bits(192u16) });
        pllcfgr.modify(|_, w| unsafe { w.pllm().bits(8u8) });
    }

    // PLL起動
    device.RCC.cr.modify(|_, w| w.pllon().on());
    while device.RCC.cr.read().pllrdy().is_not_ready() {
        // PLLの安定をただひたすら待つ
    }

    // フラッシュ読み出し遅延の変更
    device
        .FLASH
        .acr
        .modify(|_, w| unsafe { w.latency().bits(1u8) });
    // システムクロックをPLLに切り替え
    device.RCC.cfgr.modify(|_, w| w.sw().pll());
    while !device.RCC.cfgr.read().sws().is_pll() { /*wait*/ }

    // APB2のクロックを1/16
    //device.RCC.cfgr.modify(|_,w| w.ppre2().div2());
}

/// gpioのセットアップ
fn gpio_setup(device: &stm32f401::Peripherals) {
    // GPIOA 電源
    device.RCC.ahb1enr.modify(|_, w| w.gpioaen().enabled());

    // GPIO LED セットアップ
    let gpioa = &device.GPIOA;
    gpioa.moder.modify(|_, w| w.moder1().output());
    gpioa.moder.modify(|_, w| w.moder0().output());
    gpioa.moder.modify(|_, w| w.moder11().output());

    // GPIO SW セットアップ
    gpioa.moder.modify(|_, w| {
        w.moder8().input();
        w.moder15().input();
        w.moder12().input()
    });
    gpioa.pupdr.modify(|_, w| {
        w.pupdr8().pull_up();
        w.pupdr15().pull_up();
        w.pupdr12().pull_up()
    });

    // GPIO SW 割込み　セットアップ
    let syscfg = &device.SYSCFG;
    syscfg
        .exticr3
        .modify(|_, w| unsafe { w.exti8().bits(0b0000) });
    syscfg.exticr4.modify(|_, w| unsafe {
        w.exti15().bits(0b0000);
        w.exti12().bits(0b0000)
    });

    let exti = &device.EXTI;
    exti.imr.modify(|_, w| {
        w.mr8().unmasked();
        w.mr15().unmasked();
        w.mr12().unmasked()
    });
    exti.rtsr.modify(|_, w| w.tr12().enabled());
    exti.ftsr.modify(|_, w| {
        w.tr8().enabled();
        w.tr15().enabled();
        w.tr12().enabled()
    });

    use cortex_m::peripheral::NVIC;
    unsafe {
        NVIC::unmask(stm32f401::Interrupt::EXTI9_5);
        NVIC::unmask(stm32f401::Interrupt::EXTI15_10);
    }
}
