use proto_hal::macros::block;

/*
references:
- RM0440
- PM0214
- https://developer.arm.com/documentation/100166/0001/Nested-Vectored-Interrupt-Controller/NVIC-programmers-model/Table-of-NVIC-registers?lang=en
*/

#[block(base_addr = 0xe000_e000, erase_mod)]
mod nvic {
    #[schema(width = 1)]
    mod enable {
        #[state(bits = 0)]
        struct Disabled;
        #[state(bits = 1)]
        struct Enabled;
    }

    // TODO: wish there was 'register_array'
    // but how would that be done?

    // TODO: write(values = [Enabled])
    #[register(offset = 0x100, schema = enable, read, write, reset = Disabled)]
    mod iser0 {
        #[field_array(offset = 0, range = ..32)]
        mod setenaX {}
    }

    #[register(offset = 0x104, schema = enable, read, write, reset = Disabled)]
    mod iser1 {
        #[field_array(offset = 0, range = 32..64)]
        mod setenaX {}
    }

    #[register(offset = 0x108, schema = enable, read, write, reset = Disabled)]
    mod iser2 {
        #[field_array(offset = 0, range = 64..96)]
        mod setenaX {}
    }

    #[register(offset = 0x10c, schema = enable, read, write, reset = Disabled)]
    mod iser3 {
        #[field_array(offset = 0, range = 96..128)]
        mod setenaX {}
    }

    #[register(offset = 0x110, schema = enable, read, write, reset = Disabled)]
    mod iser4 {
        #[field_array(offset = 0, range = 128..160)]
        mod setenaX {}
    }

    #[register(offset = 0x114, schema = enable, read, write, reset = Disabled)]
    mod iser5 {
        #[field_array(offset = 0, range = 160..192)]
        mod setenaX {}
    }

    #[register(offset = 0x118, schema = enable, read, write, reset = Disabled)]
    mod iser6 {
        #[field_array(offset = 0, range = 192..224)]
        mod setenaX {}
    }

    #[register(offset = 0x11c, schema = enable, read, write, reset = Disabled)]
    mod iser7 {
        #[field_array(offset = 0, range = 224..240)]
        mod setenaX {}
    }

    #[register(offset = 0x180, schema = enable, read, write, reset = Disabled)]
    mod icer0 {
        #[field_array(offset = 0, range = ..32)]
        mod clrenaX {}
    }

    #[register(offset = 0x184, schema = enable, read, write, reset = Disabled)]
    mod icer1 {
        #[field_array(offset = 0, range = 32..64)]
        mod clrenaX {}
    }

    #[register(offset = 0x188, schema = enable, read, write, reset = Disabled)]
    mod icer2 {
        #[field_array(offset = 0, range = 64..96)]
        mod clrenaX {}
    }

    #[register(offset = 0x18c, schema = enable, read, write, reset = Disabled)]
    mod icer3 {
        #[field_array(offset = 0, range = 96..128)]
        mod clrenaX {}
    }

    #[register(offset = 0x190, schema = enable, read, write, reset = Disabled)]
    mod icer4 {
        #[field_array(offset = 0, range = 128..160)]
        mod clrenaX {}
    }

    #[register(offset = 0x194, schema = enable, read, write, reset = Disabled)]
    mod icer5 {
        #[field_array(offset = 0, range = 160..192)]
        mod clrenaX {}
    }

    #[register(offset = 0x198, schema = enable, read, write, reset = Disabled)]
    mod icer6 {
        #[field_array(offset = 0, range = 192..224)]
        mod clrenaX {}
    }

    #[register(offset = 0x19c, schema = enable, read, write, reset = Disabled)]
    mod icer7 {
        #[field_array(offset = 0, range = 224..240)]
        mod clrenaX {}
    }

    // TODO...
}
