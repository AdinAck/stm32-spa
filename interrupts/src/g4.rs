use proto_hal::macros::interrupts;

#[interrupts]
pub enum Interrupts {
    ///0 - Window Watchdog interrupt
    WWDG = 0,
    ///1 - PVD through EXTI line detection
    PVD_PVM = 1,
    ///2 - RTC_TAMP_CSS_LSE
    RTC_TAMP_CSS_LSE = 2,
    ///3 - RTC Wakeup timer
    RTC_WKUP = 3,
    ///4 - FLASH
    FLASH = 4,
    ///5 - RCC
    RCC = 5,
    ///6 - EXTI Line0 interrupt
    EXTI0 = 6,
    ///7 - EXTI Line1 interrupt
    EXTI1 = 7,
    ///8 - EXTI Line2 interrupt
    EXTI2 = 8,
    ///9 - EXTI Line3 interrupt
    EXTI3 = 9,
    ///10 - EXTI Line4 interrupt
    EXTI4 = 10,
    ///11 - DMA1 channel 1 interrupt
    DMA1_CH1 = 11,
    ///12 - DMA1 channel 2 interrupt
    DMA1_CH2 = 12,
    ///13 - DMA1 channel 3 interrupt
    DMA1_CH3 = 13,
    ///14 - DMA1 channel 4 interrupt
    DMA1_CH4 = 14,
    ///15 - DMA1 channel 5 interrupt
    DMA1_CH5 = 15,
    ///16 - DMA1 channel 6 interrupt
    DMA1_CH6 = 16,
    ///18 - ADC1 and ADC2 global interrupt
    ADC1_2 = 18,
    ///19 - USB_HP
    USB_HP = 19,
    ///20 - USB_LP
    USB_LP = 20,
    ///21 - fdcan1_intr0_it
    FDCAN1_INTR0_IT = 21,
    ///22 - fdcan1_intr1_it
    FDCAN1_INTR1_IT = 22,
    ///23 - EXTI9_5
    EXTI9_5 = 23,
    ///24 - TIM1_BRK_TIM15
    TIM1_BRK_TIM15 = 24,
    ///25 - TIM1_UP_TIM16
    TIM1_UP_TIM16 = 25,
    ///26 - TIM1_TRG_COM/
    TIM1_TRG_COM = 26,
    ///27 - TIM1 capture compare interrupt
    TIM1_CC = 27,
    ///28 - TIM2
    TIM2 = 28,
    ///29 - TIM3
    TIM3 = 29,
    ///30 - TIM4
    TIM4 = 30,
    ///31 - I2C1_EV
    I2C1_EV = 31,
    ///32 - I2C1_ER
    I2C1_ER = 32,
    ///33 - I2C2_EV
    I2C2_EV = 33,
    ///34 - I2C2_ER
    I2C2_ER = 34,
    ///35 - SPI1
    SPI1 = 35,
    ///36 - SPI2
    SPI2 = 36,
    ///37 - USART1
    USART1 = 37,
    ///38 - USART2
    USART2 = 38,
    ///39 - USART3
    USART3 = 39,
    ///40 - EXTI15_10
    EXTI15_10 = 40,
    ///41 - RTC_ALARM
    RTC_ALARM = 41,
    ///42 - USBWakeUP
    USBWAKE_UP = 42,
    ///43 - TIM8_BRK
    TIM8_BRK = 43,
    ///44 - TIM8_UP
    TIM8_UP = 44,
    ///45 - TIM8_TRG_COM
    TIM8_TRG_COM = 45,
    ///46 - TIM8_CC
    TIM8_CC = 46,
    ///49 - LPTIM1
    LPTIM1 = 49,
    ///51 - SPI3
    SPI3 = 51,
    ///52 - UART4
    UART4 = 52,
    ///54 - TIM6_DACUNDER
    TIM6_DACUNDER = 54,
    ///55 - TIM7
    TIM7 = 55,
    ///56 - DMA2_CH1
    DMA2_CH1 = 56,
    ///57 - DMA2_CH2
    DMA2_CH2 = 57,
    ///58 - DMA2_CH3
    DMA2_CH3 = 58,
    ///59 - DMA2_CH4
    DMA2_CH4 = 59,
    ///60 - DMA2_CH5
    DMA2_CH5 = 60,
    ///63 - UCPD1
    UCPD1 = 63,
    ///64 - COMP1_2_3
    COMP1_2_3 = 64,
    ///65 - COMP4_5_6
    COMP4 = 65,
    ///75 - CRS
    CRS = 75,
    ///76 - SAI
    SAI = 76,
    ///81 - Floating point unit interrupt
    FPU = 81,
    ///85 - AES
    AES = 85,
    ///90 - RNG
    RNG = 90,
    ///91 - LPUART
    LPUART = 91,
    ///92 - I2C3_EV
    I2C3_EV = 92,
    ///93 - I2C3_ER
    I2C3_ER = 93,
    ///94 - DMAMUX_OVR
    DMAMUX_OVR = 94,
    ///97 - DMA2_CH6
    DMA2_CH6 = 97,
    ///100 - Cordic
    CORDIC = 100,
    ///101 - FMAC
    FMAC = 101,
}
