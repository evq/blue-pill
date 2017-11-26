//! Blocking version of blinky using watchdog

#![allow(unreachable_code)] // for the `block!` macro
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate blue_pill;

extern crate cortex_m_rtfm as rtfm;

#[macro_use(block)]
extern crate nb;

use blue_pill::Iwdg;
use blue_pill::Timer;
use blue_pill::led::{self, LED};
use blue_pill::prelude::*;
use blue_pill::time::{Hertz, Milliseconds};
use rtfm::{app, Threshold};

const FREQUENCY: Hertz = Hertz(1);

app! {
    device: blue_pill::stm32f103xx,

    idle: {
        resources: [TIM3, IWDG],
    }
}

fn init(p: init::Peripherals) {
    led::init(p.GPIOC, p.RCC);

    let timer = Timer(&*p.TIM3);

    timer.init(FREQUENCY.invert(), p.RCC);

    let watchdog = Iwdg(&*p.IWDG);

    watchdog.init(Milliseconds(1250));
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    let timer = Timer(&*r.TIM3);
    let watchdog = Iwdg(&*r.IWDG);

    timer.resume();
    let mut state = false;
    let mut count = 0;
    loop {
        block!(timer.wait()).unwrap(); // NOTE(unwrap) E = !

        watchdog.reset();

        state = !state;

        if state {
            LED.on();
        } else {
            LED.off();
            count += 1;
        }
        if count > 3 {
            loop { }
        }
    }
}
