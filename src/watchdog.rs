//! Watchdog Timer

use time::*;

use stm32f103xx::{IWDG};

/// IWDG Commands
pub enum IwdgCommand {
    ///Disable write protection of IWDG registers
    DisableWriteProtect = 0x5555,
    ///Reload IWDG
    Reload = 0xAAAA,
    ///Start IWDG
    Start = 0xCCCC,
}

const LSI_FREQUENCY_KHZ: u32 = 40;
const BASE_MAX_TIMEOUT_MICROS: u32 = ((4000 * 0xFFF) / LSI_FREQUENCY_KHZ) + 100;
const BASE_MAX_TIMEOUT_MILLIS: u32 = (4 * 0xFFF) / LSI_FREQUENCY_KHZ;

/// timeout = (4 * ReloadValue * 2^PreScaler) / (LSI clock freq)
pub struct IwdgParams {
    prescaler: u8,
    reload_value: u16
}

impl From<Milliseconds> for IwdgParams {
    fn from(ms: Milliseconds) -> IwdgParams {
        for prescaler in 0..6 {
            let max_timeout_millis = BASE_MAX_TIMEOUT_MILLIS * (1 << prescaler);
            if ms.0 <= max_timeout_millis {
                let reload_value = ((0xFFF * ms.0) / max_timeout_millis) as u16;
                return IwdgParams { prescaler, reload_value }
            }
        }
        panic!("timeout specified exceeds maximum watchdog timeout") 
    }
}

impl From<Microseconds> for IwdgParams {
    fn from(us: Microseconds) -> IwdgParams {
        for prescaler in 0..1 {
            let max_timeout_micros = BASE_MAX_TIMEOUT_MICROS * (1 << prescaler);
            if us.0 <= max_timeout_micros {
                let reload_value = ((0xFFF * us.0) / max_timeout_micros) as u16;
                return IwdgParams { prescaler, reload_value }
            }
        }
        panic!("timeout specified exceeds maximum watchdog timeout") 
    }
}

/// IWDG ( Independent Watchdog )
pub struct Iwdg<'a>(pub &'a IWDG);

impl<'a> Iwdg<'a> {
    /// Initializes IWDG
    pub fn init<R>(&self, timeout: R)
    where
        R: Into<IwdgParams>,
    {
        let iwdg = self.0;

        iwdg.kr.write(|w| unsafe { w.key().bits(IwdgCommand::DisableWriteProtect as u16) });

        let params = timeout.into();
        iwdg.pr.write(|w| unsafe { w.pr().bits(params.prescaler) });
        iwdg.rlr.write(|w| unsafe { w.rl().bits(params.reload_value) });

        iwdg.kr.write(|w| unsafe { w.key().bits(IwdgCommand::Reload as u16) });
        iwdg.kr.write(|w| unsafe { w.key().bits(IwdgCommand::Start as u16) });
    }

    /// Reset watchdog timer 
    pub fn reset(&self) {
        let iwdg = self.0;
        iwdg.kr.write(|w| unsafe { w.key().bits(IwdgCommand::Reload as u16) });
    }
}
