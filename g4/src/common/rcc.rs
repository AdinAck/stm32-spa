use proto_hal::macros::block;

#[block(base_addr = 0x4002_1000, auto_increment, erase_mod)]
mod rcc {
    #[schema(width = 1)]
    mod enable {
        #[state(bits = 0)]
        struct Disabled;
        #[state(bits = 1)]
        struct Enabled;
    }

    #[register(offset = 0x48)]
    mod ahb1enr {
        #[field(offset = 3, schema = enable, read, write, reset = Disabled)]
        mod cordicen {}
    }

    #[register(auto_increment)]
    mod ahb2enr {
        #[field(schema = enable, read, write, reset = Disabled)]
        mod gpioaen {}
        #[field(schema = enable, read, write, reset = Disabled)]
        mod gpioben {}
        #[field(schema = enable, read, write, reset = Disabled)]
        mod gpiocen {}
        #[field(schema = enable, read, write, reset = Disabled)]
        mod gpioden {}
        #[field(schema = enable, read, write, reset = Disabled)]
        mod gpioeen {}
        #[field(schema = enable, read, write, reset = Disabled)]
        mod gpiofen {}
        #[field(schema = enable, read, write, reset = Disabled)]
        mod gpiogen {}
    }

    #[register(offset = 0x60)]
    mod apb2enr {
        #[field(offset = 0, schema = enable, read, write, reset = Disabled)]
        mod syscfgen {}
    }
}
