use proto_hal::macros::block;

#[block(base_addr = 0x4001_0400, auto_increment, erase_mod)]
mod exti {
    #[schema(width = 1)]
    mod mask {
        #[state(bits = 0)]
        struct Masked;

        #[state(bits = 1)]
        struct Unmasked;
    }

    #[register(auto_increment)]
    mod imr1 {
        #[field_array(range = ..32, schema = mask, read, write, reset = Masked)]
        mod imX {}
    }

    #[register(auto_increment)]
    mod emr1 {
        #[field_array(range = ..32, schema = mask, read, write, reset = Masked)]
        mod emX {}
    }

    #[schema(width = 1)]
    mod enable {
        #[state(bits = 0)]
        struct Disabled;

        #[state(bits = 1)]
        struct Enabled;
    }

    #[register(auto_increment)]
    mod rtsr1 {
        #[field_array(range = ..18, schema = enable, read, write, reset = Disabled)]
        mod rtX {}

        #[field_array(range = 19..23, offset = 19, schema = enable, read, write, reset = Disabled)]
        mod rtX {}

        #[field_array(range = 29..32, offset = 29, schema = enable, read, write, reset = Disabled)]
        mod rtX {}
    }

    #[register(auto_increment)]
    mod ftsr1 {
        #[field_array(range = ..18, schema = enable, read, write, reset = Disabled)]
        mod ftX {}

        #[field_array(range = 19..23, offset = 19, schema = enable, read, write, reset = Disabled)]
        mod ftX {}

        #[field_array(range = 29..32, offset = 29, schema = enable, read, write, reset = Disabled)]
        mod ftX {}
    }

    // a little weird, RM doesn't
    // say anything about writing 0
    // to these fields...
    #[register(auto_increment)]
    mod swier1 {
        #[field_array(range = ..18, schema = enable, read, write, reset = Disabled)]
        mod swiX {}

        #[field_array(range = 19..23, offset = 19, schema = enable, read, write, reset = Disabled)]
        mod swiX {}

        #[field_array(range = 29..32, offset = 29, schema = enable, read, write, reset = Disabled)]
        mod swiX {}
    }

    // a little weird, RM doesn't
    // say anything about writing 0
    // to these fields...
    #[register(auto_increment)]
    mod pr1 {
        #[field_array(range = ..18, schema = enable, read, write, reset = Disabled)]
        mod pifX {}

        #[field_array(range = 19..23, offset = 19, schema = enable, read, write, reset = Disabled)]
        mod pifX {}

        #[field_array(range = 29..32, offset = 29, schema = enable, read, write, reset = Disabled)]
        mod pifX {}
    }

    // TODO...
}
