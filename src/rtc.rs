use stm32f4::stm32f401;
/// rtcモジュールの制御
pub struct Rtc<'a> {
    device: &'a stm32f401::Peripherals,
}

impl Rtc<'_> {
    /// RTCモジュールを初期化する。
    /// モードの変更等、init_pwr_blockを変更の際には、バックアップドメインリセットが必要。
    pub fn new(device: &stm32f401::Peripherals) -> Rtc {
        let rtc = Rtc { device };
        rtc.init_pwr_block();
        rtc
    }

    fn init_pwr_block(&self) {
        //バックアップドメインセットアップ
        self.device.RCC.apb1enr.modify(|_, w| w.pwren().enabled());
        self.device.PWR.cr.modify(|_, w| w.dbp().set_bit());
        if !self.device.PWR.csr.read().brr().bits() {
            self.device.PWR.csr.modify(|_, w| w.bre().set_bit());
            while !self.device.PWR.csr.read().brr().bits() {}
        }
        if self.device.RCC.bdcr.read().lserdy().is_not_ready() {
            self.device.RCC.bdcr.modify(|_, w| w.lseon().on());
            while self.device.RCC.bdcr.read().lserdy().is_not_ready() {}
        }
        if self.device.RCC.bdcr.read().rtcen().is_disabled() {
            self.device.RCC.bdcr.modify(|_, w| {
                w.rtcsel().lse();
                w.rtcen().enabled()
            });
        }
    }

    /// 過去に時刻がセットされているかを返す。
    pub fn time_set_done(&self) -> bool {
        self.device.RTC.isr.read().inits().bits()
    }

    /// RTCの時刻設定の準備
    /// 時刻の設定は、exec_set_timeで行う。
    pub fn set_time_pre(device: &stm32f401::Peripherals, date_time: &DateTime) {
        let r = &device.RTC;

        //書き込みロック解除
        r.wpr.write(|w| unsafe { w.key().bits(0xCA) });
        r.wpr.write(|w| unsafe { w.key().bits(0x53) });
        //initモード切り替え
        r.isr.modify(|_, w| w.init().set_bit());
        while r.isr.read().initf().bits() == false {
            cortex_m::asm::nop();
        }
        //24時間制の指定
        r.cr.modify(|_, w| w.fmt().clear_bit());
        //日時書き込み
        r.dr.modify(|_, w| unsafe {
            w.yt().bits(date_time.year / 10);
            w.yu().bits(date_time.year % 10);
            w.wdu().bits(Self::daytoweek(&date_time));
            w.mt().bit(date_time.month >= 10);
            w.mu().bits(date_time.month % 10);
            w.dt().bits(date_time.date / 10);
            w.du().bits(date_time.date % 10)
        });
        //時刻書き込み
        r.tr.modify(|_, w| unsafe {
            w.pm().clear_bit();
            w.ht().bits(date_time.hour / 10);
            w.hu().bits(date_time.hour % 10);
            w.mnt().bits(date_time.min / 10);
            w.mnu().bits(date_time.min % 10);
            w.st().bits(date_time.sec / 10);
            w.su().bits(date_time.sec % 10)
        });
    }

    /// 時刻設定を実行する。時計を再スタートする。
    pub fn exec_set_time(device: &stm32f401::Peripherals) {
        let r = &device.RTC;
        //initモード解除(時計スタート)
        r.isr.modify(|_, w| w.init().clear_bit());
        //書き込みロックの復帰
        r.wpr.write(|w| unsafe { w.key().bits(0xff) });
    }

    /// 日付より曜日を求める
    pub fn daytoweek(date_time: &DateTime) -> u8 {
        let c = date_time.year / 100;
        let g = 5 * c + c / 4;
        let ym100 = 20;
        let m_kou = 26 * (date_time.month + 1) / 10;
        (date_time.date + m_kou + ym100 + ym100 / 4 + g + 5) % 7 + 1
    }

    /// 時刻の取得
    /// 戻り値:(時,分,秒)のタプル
    pub fn get_time(&self) -> (u32, u32, u32) {
        let r = &self.device.RTC.tr.read();
        let h = (r.ht().bits() as u32) * 10 + (r.hu().bits() as u32);
        let m = (r.mnt().bits() as u32) * 10 + (r.mnu().bits() as u32);
        let s = (r.st().bits() as u32) * 10 + (r.su().bits() as u32);
        (h, m, s)
    }

    /// 日時の取得
    /// 戻り値:(年,月,日,曜日)のタプル
    /// 曜日の数値は、月曜日が1で日曜日が7
    pub fn get_date(&self) -> (u32, u32, u32, u32) {
        let r = &self.device.RTC.dr.read();
        let y = (r.yt().bits() as u32) * 10 + (r.yu().bits() as u32) + 2000;
        let m = if r.mt().bits() { 10 } else { 0 } + (r.mu().bits() as u32);
        let d = (r.dt().bits() as u32) * 10 + (r.du().bits() as u32);
        let w = r.wdu().bits() as u32;
        (y, m, d, w)
    }
}

/// 日時指定用の構造体
pub struct DateTime {
    pub year: u8,
    pub month: u8,
    pub date: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}
