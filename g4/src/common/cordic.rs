use proto_hal::macros::block;

#[block(
    base_addr = 0x4002_0c00,
    entitlements = [super::rcc::ahb1enr::cordicen::Enabled],
    auto_increment,
    erase_mod,
)]
mod cordic {
    #[register(read, auto_increment)]
    mod csr {
        #[field(width = 4, write, reset = Cos, auto_increment)]
        mod func {
            #[variant(entitlements = [scale::N0])]
            struct Cos;

            #[variant(entitlements = [scale::N0])]
            struct Sin;

            #[variant(entitlements = [scale::N0])]
            struct ATan2;

            #[variant(entitlements = [scale::N0])]
            struct Magnitude;

            #[variant]
            struct ATan;

            #[variant(entitlements = [scale::N1])]
            struct CosH;

            #[variant(entitlements = [scale::N1])]
            struct SinH;

            #[variant(entitlements = [scale::N1])]
            struct ATanH;

            #[variant(entitlements = [scale::N1, scale::N2, scale::N3, scale::N4])]
            struct Ln;

            #[variant(entitlements = [scale::N0, scale::N1, scale::N2])]
            struct Sqrt;
        }

        #[field(width = 4, write, reset = P20)]
        /// custom docs
        mod precision {
            #[variant_array(bits = 1, range = 4..=60, step = 4)]
            struct PX;
        }

        #[field(width = 3, write, reset = N0)]
        mod scale {
            #[variant_array(bits = 0, range = ..8)]
            struct NX;
        }

        #[schema(width = 1)]
        mod enable {
            #[variant(bits = 0)]
            struct Disabled;
            #[variant(bits = 1)]
            struct Enabled;
        }

        #[field(offset = 16, schema = enable, write, reset = Disabled)]
        mod ien {}

        #[field(schema = enable, write, reset = Disabled)]
        mod dmaren {}

        #[field(schema = enable, write, reset = Disabled)]
        mod dmawen {}

        #[field(width = 1, write, reset = OneRead)]
        mod nres {
            #[variant(bits = 0)]
            struct OneRead;
            #[variant(bits = 1, entitlements = [ressize::Q31])]
            struct TwoReads;
        }

        #[field(width = 1, write, reset = OneWrite)]
        mod nargs {
            #[variant(bits = 0)]
            struct OneWrite;
            #[variant(bits = 1, entitlements = [argsize::Q31])]
            struct TwoWrites;
        }

        #[schema(width = 1)]
        mod data {
            #[variant(bits = 0)]
            struct Q31;
            #[variant(bits = 1)]
            struct Q15;
        }

        #[field(schema = data, write, reset = Q31)]
        mod ressize {}

        #[field(schema = data, write, reset = Q31)]
        mod argsize {}

        #[field(offset = 31, width = 1)]
        mod rrdy {
            #[variant(bits = 0)]
            struct NoData;
            #[variant(bits = 1)]
            struct DataReady;
        }
    }

    #[register(width = 32, write(effect = unresolve(csr::rrdy)))]
    mod wdata {
        #[field(offset = 0)]
        mod arg {}
    }

    #[register(width = 32, read(entitlements = [csr::rrdy::Ready], effect = unresolve(csr::rrdy)))]
    mod rdata {
        #[field(offset = 0)]
        mod res {}
    }

    // #[interrupt(
    //     position = 100,
    //     entitlements = [csr::ien::Enabled],
    //     effects = [resolve(csr::rrdy::DataReady)]
    // )]
    // mod interrupt {}
}
