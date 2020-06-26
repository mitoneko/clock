//! matrix_ledをキャラクタディスプレイとして使用する
//! matrix_ledモジュールは、display_ledを経由して使用する。

use super::matrix_led::Matrix;
//use cortex_m;
//use cortex_m_semihosting::dbg;
use heapless::{consts::U50, Vec};
use stm32f4::stm32f401;

///ディスプレイドライバ
pub struct DisplayLed<'a> {
    led: Matrix<'a>,
    buff: Vec<u8, U50>,
}

impl<'a> DisplayLed<'a> {
    pub fn new(device: &'a stm32f401::Peripherals) -> Self {
        let mut display = DisplayLed {
            led: Matrix::new(&device),
            buff: Vec::new(),
        };
        display.led.clear();
        display
    }

    pub fn clear(&mut self) {
        self.led.clear();
    }
}

use crate::misakifont::font48::FONT48;
use core::fmt;
use core::fmt::Write;
//use misakifont::font88::FONT88;

impl fmt::Write for DisplayLed<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() == 0 {
            return Ok(());
        }
        let mut is_output = false;
        for c in s.chars() {
            match c {
                '\n' => {
                    is_output = true;
                }
                cc if cc.is_ascii_control() => {}
                cc => self.buff.push(cc as u8).unwrap_or(()),
            }
        }

        if is_output == true {
            self.led.clear();
            for (i, c) in self.buff.iter().enumerate() {
                let font = FONT48.get_char(*c);
                self.led.draw_bitmap((i * 4) as i32, 0, 4, font);
                while let Err(_) = self.led.flash_led() {}
            }
            self.buff.truncate(0);
        }

        Ok(())
    }
}

pub fn print_led_fmt(disp: &mut DisplayLed, args: fmt::Arguments) {
    disp.write_fmt(args).unwrap()
}

#[macro_export]
macro_rules! print_led {
    ($disp:expr, $($arg:tt)*) => {
        display_led::print_led_fmt(&mut $disp, format_args!($($arg)*));
    }
}
