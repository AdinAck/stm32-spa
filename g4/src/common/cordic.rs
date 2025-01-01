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
            #[state(entitlements = [scale::N0])]
            struct Cos;

            #[state(entitlements = [scale::N0])]
            struct Sin;

            #[state(entitlements = [scale::N0])]
            struct ATan2;

            #[state(entitlements = [scale::N0])]
            struct Magnitude;

            #[state]
            struct ATan;

            #[state(entitlements = [scale::N1])]
            struct CosH;

            #[state(entitlements = [scale::N1])]
            struct SinH;

            #[state(entitlements = [scale::N1])]
            struct ATanH;

            #[state(entitlements = [scale::N1, scale::N2, scale::N3, scale::N4])]
            struct Ln;

            #[state(entitlements = [scale::N0, scale::N1, scale::N2])]
            struct Sqrt;
        }

        #[field(width = 4, write, reset = P20)]
        /// custom docs
        mod precision {
            #[state_array(bits = 1, range = 4..=60, step = 4)]
            struct PX;
        }

        #[field(width = 3, write, reset = N0)]
        mod scale {
            #[state_array(bits = 0, range = ..8)]
            struct NX;
        }

        #[schema(width = 1)]
        mod enable {
            #[state(bits = 0)]
            struct Disabled;
            #[state(bits = 1)]
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
            #[state(bits = 0)]
            struct OneRead;
            #[state(bits = 1, entitlements = [ressize::Q31])]
            struct TwoReads;
        }

        #[field(width = 1, write, reset = OneWrite)]
        mod nargs {
            #[state(bits = 0)]
            struct OneWrite;
            #[state(bits = 1, entitlements = [argsize::Q31])]
            struct TwoWrites;
        }

        #[schema(width = 1)]
        mod data {
            #[state(bits = 0)]
            struct Q31;
            #[state(bits = 1)]
            struct Q15;
        }

        #[field(schema = data, write, reset = Q31)]
        mod ressize {}

        #[field(schema = data, write, reset = Q31)]
        mod argsize {}

        #[field(offset = 31, width = 1, reset = NoData)]
        mod rrdy {
            #[state(bits = 0)]
            struct NoData;
            #[state(bits = 1)]
            struct DataReady;
        }
    }

    #[register(width = 32, write(effect = unresolve(csr::rrdy)))]
    mod wdata {
        #[field(offset = 0)]
        mod arg {}
    }

    #[register(width = 32, read(entitlements = [csr::rrdy::Ready], effect = unresolve(csr::rrdy)), reset = 0)]
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
