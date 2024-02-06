#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use robot_tour_robot_code as _; // global logger + panicking-behavior + memory layout

// TODO(7) Configure the `rtic::app` macro
#[rtic::app(
    device = stm32f4xx_hal::pac,
    dispatchers = [USART1, USART2, USART6]
)]
mod app {
    use rtic_monotonics::systick::*;
    use stm32f4xx_hal::{
        gpio::{Output, Pin},
        prelude::*,
        timer::{Channel1, Channel2, Timer},
    };

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        led: Pin<'C', 13, Output>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let freq = 48.MHz();
        let dp = cx.device;
        let rcc = dp.RCC.constrain();
        let gpioc = dp.GPIOC.split();
        let gpioa = dp.GPIOA.split();
        let led = gpioc.pc13.into_push_pull_output();

        // Setup monotonic
        let sysclk = rcc.cfgr.sysclk(freq).freeze();
        let token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, freq.to_Hz(), token);

        // Setup PWM
        let channels = (Channel1::new(gpioa.pa8), Channel2::new(gpioa.pa9));
        let pwm = Timer::new(dp.TIM1, &sysclk).pwm_hz(channels, 50.Hz());
        let (mut ch1, _ch2) = pwm.split();
        let max_duty = ch1.get_max_duty();
        ch1.set_duty(max_duty / 2);
        ch1.enable();

        blink::spawn().unwrap();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local { led },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    // TODO: Add tasks
    #[task(priority = 1, local = [led])]
    async fn blink(cx: blink::Context) {
        loop {
            cx.local.led.toggle();
            Systick::delay(1.secs()).await;
        }
    }
}
