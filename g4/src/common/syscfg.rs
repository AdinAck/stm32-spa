use proto_hal::macros::block;

#[block(base_addr = 0x4001_0000, entitlements = [super::rcc::apb2enr::syscfgen::Enabled], erase_mod)]
mod syscfg {
    #[schema(width = 4, auto_increment)]
    mod port {
        #[variant]
        struct PA;
        #[variant]
        struct PB;
        #[variant]
        struct PC;
        #[variant]
        struct PD;
        #[variant]
        struct PE;
        #[variant]
        struct PF;
    }

    #[register(offset = 0x14, auto_increment)]
    mod exticr4 {
        #[field_array(range = 12..16, schema = port, read, write, reset = PA)]
        mod extiX {}
    }
}
