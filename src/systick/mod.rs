use core::cell::RefCell;
use cortex_m::interrupt::{free, CriticalSection, Mutex};
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use cortex_m_rt::exception;
use heapless::consts::U8;
use heapless::Vec;

pub struct SysTick;

impl SysTick {
    pub fn new(cs: &CriticalSection, syst: SYST, sys_freq: u32, cycle: u32) -> SysTick {
        MY_SYSTICK.borrow(cs).replace(Some(syst));
        INTERRUPT_FUNC.borrow(cs).replace(Some(Vec::new()));

        let mut syst = MY_SYSTICK.borrow(cs).borrow_mut();
        syst.as_mut().unwrap().set_reload(sys_freq / cycle);
        syst.as_mut().unwrap().clear_current();
        syst.as_mut().unwrap().set_clock_source(SystClkSource::Core);
        syst.as_mut().unwrap().enable_interrupt();

        SysTick
    }

    pub fn enable_counter(&self, cs: &CriticalSection) {
        let mut syst = MY_SYSTICK.borrow(cs).borrow_mut();
        syst.as_mut().unwrap().enable_counter();
    }

    pub fn add_callback_function(&self, cs: &CriticalSection, func: fn() -> ()) {
        let mut table = INTERRUPT_FUNC.borrow(cs).borrow_mut();
        table
            .as_mut()
            .unwrap()
            .push(func)
            .expect("Interrupt table over fullow");
    }
}

static MY_SYSTICK: Mutex<RefCell<Option<SYST>>> = Mutex::new(RefCell::new(None));

static INTERRUPT_FUNC: Mutex<RefCell<Option<Vec<fn() -> (), U8>>>> = Mutex::new(RefCell::new(None));

#[exception]
fn SysTick() {
    free(|cs| {
        let work = INTERRUPT_FUNC.borrow(cs).borrow();
        let table = work.as_ref().unwrap();
        for func in table.into_iter() {
            func();
        }
    });
}
