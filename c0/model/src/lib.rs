pub mod gpio;
pub mod rcc;

use phm::{Composition, Interrupt};

use crate::{gpio::gpio, rcc::rcc};

// TODO
#[derive(Debug, Default)]
pub struct Configuration {
    pub variant: Variant,
}

impl Configuration {
    pub fn c092() -> Self {
        Self {
            variant: Variant::C092,
        }
    }
}

#[derive(Debug, Default)]
pub enum Variant {
    #[default]
    C092,
}

pub fn compose(config: Configuration) -> Composition {
    let mut composition = Composition::new().with_interrupts([
        Interrupt::handler("WWDG").docs(["Window watchdog interrupt"]),
        Interrupt::handler("PVM").docs(["VDDIO2 monitor interrupt (EXTI line 34)"]),
        Interrupt::handler("RTC").docs(["RTC interrupts (EXTI line 19)"]),
        Interrupt::handler("FLASH").docs(["Flash global interrupt"]),
        Interrupt::handler("RCC_CRS").docs(["RCC/CRS global interrupt"]),
        Interrupt::handler("EXTI0_1").docs(["EXTI line 0 & 1 interrupt"]),
        Interrupt::handler("EXTI2_3").docs(["EXTI line 2 & 3 interrupt"]),
        Interrupt::handler("EXTI4_15").docs(["EXTI line 4 to 15 interrupt"]),
        Interrupt::handler("USB").docs(["USB global interrupt (combined with EXTI line 36)"]),
        Interrupt::handler("DMA1_Channel1").docs(["DMA1 channel 1 interrupt"]),
        Interrupt::handler("DMA1_Channel2_3").docs(["DMA1 channel 2 & 3 interrupts"]),
        Interrupt::handler("DMAMUX_DMA1_Channel4_5_6_7")
            .docs(["DMAMUX and DMA1 channel 4, 5, 6, and 7 interrupts"]),
        Interrupt::handler("ADC").docs(["ADC interrupt"]),
        Interrupt::handler("TIM1_BRK_UP_TRG_COM")
            .docs(["TIM1 break, update, trigger and commutation interrupts"]),
        Interrupt::handler("TIM1_CC").docs(["TIM1 Capture Compare interrupt"]),
        Interrupt::handler("TIM2").docs(["TIM2 global interrupt"]),
        Interrupt::handler("TIM3").docs(["TIM3 global interrupt"]),
        Interrupt::handler("TIM14").docs(["TIM14 global interrupt"]),
        Interrupt::handler("TIM15").docs(["TIM15 global interrupt"]),
        Interrupt::handler("TIM16").docs(["TIM16 global interrupt"]),
        Interrupt::handler("TIM17").docs(["TIM17 global interrupt"]),
        Interrupt::handler("I2C1").docs(["I2C1 global interrupt (combined with EXTI line 23)"]),
        Interrupt::handler("I2C2").docs(["I2C2 global interrupt"]),
        Interrupt::handler("SPI1").docs(["SPI1 global interrupt"]),
        Interrupt::handler("SPI2").docs(["SPI2 global interrupt"]),
        Interrupt::handler("USART1").docs(["USART1 global interrupt (combined with EXTI line 25)"]),
        Interrupt::handler("USART2").docs(["USART2 global interrupt"]),
        Interrupt::handler("USART3_USART4")
            .docs(["USART3/4 global interrupt (combined with EXTI 28)"]),
        Interrupt::handler("FDCAN_IT0").docs(["FDCAN global interrupt 0"]),
        Interrupt::handler("FDCAN_IT1").docs(["FDCAN global interrupt 1"]),
    ]);

    let rcc = rcc(&mut composition);
    gpio(
        &mut composition,
        &config,
        gpio::Instance::A,
        rcc.iopenr.gpioaen,
    );
    gpio(
        &mut composition,
        &config,
        gpio::Instance::B,
        rcc.iopenr.gpioben,
    );
    gpio(
        &mut composition,
        &config,
        gpio::Instance::C,
        rcc.iopenr.gpiocen,
    );
    gpio(
        &mut composition,
        &config,
        gpio::Instance::D,
        rcc.iopenr.gpioden,
    );
    gpio(
        &mut composition,
        &config,
        gpio::Instance::F,
        rcc.iopenr.gpiofen,
    );

    composition
}
