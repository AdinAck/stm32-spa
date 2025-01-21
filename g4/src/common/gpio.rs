use proto_hal::macros::block;

#[block(
    base_addr = 0x4800_0000,
    entitlements = [super::super::rcc::ahb2enr::gpioaen::Enabled],
    auto_increment,
)]
pub mod gpioa {
    #[register(schema = mode, read, write, auto_increment)]
    mod moder {
        #[schema(width = 2, auto_increment)]
        mod mode {
            #[variant]
            struct Input;
            #[variant]
            struct Output;
            #[variant]
            struct Alternate;
            #[variant]
            struct Analog;
        }

        #[field_array(range = ..13, reset = Analog)]
        mod modeX {}

        #[field(reset = Alternate)]
        mod mode13 {}

        #[field(reset = Alternate)]
        mod mode14 {}

        #[field(reset = Alternate)]
        mod mode15 {}
    }

    #[register(schema = otype, read, write, auto_increment)]
    mod otyper {
        #[schema(width = 1)]
        mod otype {
            #[variant(bits = 0)]
            struct PushPull;
            #[variant(bits = 1)]
            struct OpenDrain;
        }

        #[field_array(range = ..16, reset = PushPull)]
        mod otX {}
    }

    #[register(schema = ospeed, read, write, auto_increment)]
    mod ospeedr {
        #[schema(width = 2, auto_increment)]
        mod ospeed {
            #[variant]
            struct Low;
            #[variant]
            struct Medium;
            #[variant]
            struct High;
            #[variant]
            struct VeryHigh;
        }

        #[field_array(range = ..13, reset = Low)]
        mod ospeedX {}

        #[field(reset = VeryHigh)]
        mod ospeed13 {}

        #[field(reset = Low)]
        mod ospeed14 {}

        #[field(reset = Low)]
        mod ospeed15 {}
    }

    #[register(schema = pupd, read, write, auto_increment)]
    mod pupdr {
        #[schema(width = 2, auto_increment)]
        mod pupd {
            #[variant]
            struct None;
            #[variant]
            struct PullUp;
            #[variant]
            struct PullDown;
        }

        #[field_array(range = ..13, reset = None)]
        mod pupdX {}

        #[field(reset = PullUp)]
        mod pupd13 {}

        #[field(reset = PullDown)]
        mod pupd14 {}

        #[field(reset = PullUp)]
        mod pupd15 {}
    }

    #[schema(width = 1)]
    mod level {
        #[variant(bits = 0)]
        struct Low;
        #[variant(bits = 1)]
        struct High;
    }

    #[register(schema = level, read)]
    mod idr {
        // since this is read only and is not registered under any effects,
        // this is unresolvable
        #[field_array(offset = 0, range = ..16)]
        mod idX {}
    }

    #[register(schema = level, read, write, reset = Low)]
    mod odr {
        #[field_array(offset = 0, range = ..16)]
        mod odX {}
    }

    // this could probably be
    // stateful
    #[register(width = 1, write, auto_increment)]
    mod bsrr {
        #[field_array(range = ..16)]
        mod bsX {
            #[variant(bits = 1)]
            struct Reset;
        }

        #[field_array(range = ..16)]
        mod brX {
            #[variant(bits = 1)]
            struct Set;
        }
    }

    #[schema(width = 4, auto_increment)]
    mod afr {
        #[variant_array(range = ..16)]
        struct AFX;
    }

    #[register(offset = 0x20, schema = afr, read, write, reset = AF0)]
    mod afrl {
        #[field_array(offset = 0, range = ..8)]
        mod afselX {}
    }

    #[register(schema = afr, read, write, reset = AF0)]
    mod afrh {
        #[field_array(offset = 0, range = 8..16)]
        mod afselX {}
    }

    #[register(write)]
    mod brr {
        #[field_array(offset = 0, range = ..16, width = 1)]
        mod brX {
            #[variant(bits = 1)]
            struct Reset;
        }
    }
}

// #[block(
//     base_addr = 0x4800_0400,
//     entitlements = [super::super::rcc::ahb2enr::gpioben::Enabled],
//     auto_increment
// )]
// pub mod gpiob {
//     #[register(auto_increment)]
//     mod moder {
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode0 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode1 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode2 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode3 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant(reset)]
//             struct Alternate;
//             #[variant]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode4 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant(reset)]
//             struct Alternate;
//             #[variant]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode5 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode6 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode7 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode8 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode9 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode10 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode11 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode12 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode13 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode14 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode15 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//     }

//     #[register(auto_increment)]
//     mod otyper {
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot0 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot1 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot2 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot3 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot4 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot5 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot6 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot7 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot8 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot9 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot10 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot11 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot12 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot13 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot14 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot15 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//     }

//     #[register(auto_increment)]
//     mod ospeedr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed0 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed1 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed2 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed3 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed4 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed5 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed6 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed7 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed8 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed9 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed10 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed11 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed12 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed13 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed14 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed15 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//     }

//     #[register(auto_increment)]
//     mod pupdr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd0 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd1 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd2 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd3 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd4 {
//             #[variant]
//             struct None;
//             #[variant(reset)]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd5 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd6 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd7 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd8 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd9 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd10 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd11 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd12 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd13 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd14 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd15 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//     }

//     #[register(auto_increment)]
//     mod idr {
//         #[field(width = 1, read)]
//         mod id0 {}
//         #[field(width = 1, read)]
//         mod id1 {}
//         #[field(width = 1, read)]
//         mod id2 {}
//         #[field(width = 1, read)]
//         mod id3 {}
//         #[field(width = 1, read)]
//         mod id4 {}
//         #[field(width = 1, read)]
//         mod id5 {}
//         #[field(width = 1, read)]
//         mod id6 {}
//         #[field(width = 1, read)]
//         mod id7 {}
//         #[field(width = 1, read)]
//         mod id8 {}
//         #[field(width = 1, read)]
//         mod id9 {}
//         #[field(width = 1, read)]
//         mod id10 {}
//         #[field(width = 1, read)]
//         mod id11 {}
//         #[field(width = 1, read)]
//         mod id12 {}
//         #[field(width = 1, read)]
//         mod id13 {}
//         #[field(width = 1, read)]
//         mod id14 {}
//         #[field(width = 1, read)]
//         mod id15 {}
//     }

//     #[register(auto_increment)]
//     mod odr {
//         #[field(width = 1, read, write)]
//         mod od0 {}
//         #[field(width = 1, read, write)]
//         mod od1 {}
//         #[field(width = 1, read, write)]
//         mod od2 {}
//         #[field(width = 1, read, write)]
//         mod od3 {}
//         #[field(width = 1, read, write)]
//         mod od4 {}
//         #[field(width = 1, read, write)]
//         mod od5 {}
//         #[field(width = 1, read, write)]
//         mod od6 {}
//         #[field(width = 1, read, write)]
//         mod od7 {}
//         #[field(width = 1, read, write)]
//         mod od8 {}
//         #[field(width = 1, read, write)]
//         mod od9 {}
//         #[field(width = 1, read, write)]
//         mod od10 {}
//         #[field(width = 1, read, write)]
//         mod od11 {}
//         #[field(width = 1, read, write)]
//         mod od12 {}
//         #[field(width = 1, read, write)]
//         mod od13 {}
//         #[field(width = 1, read, write)]
//         mod od14 {}
//         #[field(width = 1, read, write)]
//         mod od15 {}
//     }

//     // this could probably be
//     // stateful
//     #[register(auto_increment)]
//     mod bsrr {
//         #[field(width = 1, write, reset = 0)]
//         mod bs0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs15 {}

//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }

//     #[register(offset = 0x20, auto_increment)]
//     mod afrl {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel0 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel1 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel2 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel3 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel4 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel5 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel6 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel7 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod afrh {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel8 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel9 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel10 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel11 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel12 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel13 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel14 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel15 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod brr {
//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }
// }

// #[block(
//     base_addr = 0x4800_0800,
//     entitlements = [super::super::rcc::ahb2enr::gpiocen::Enabled],
//     auto_increment,
// )]
// pub mod gpioc {
//     #[register(auto_increment)]
//     mod moder {
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode0 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode1 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode2 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode3 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode4 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode5 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode6 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode7 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode8 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode9 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode10 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode11 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode12 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode13 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode14 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode15 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//     }

//     #[register(auto_increment)]
//     mod otyper {
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot0 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot1 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot2 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot3 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot4 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot5 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot6 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot7 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot8 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot9 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot10 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot11 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot12 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot13 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot14 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot15 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//     }

//     #[register(auto_increment)]
//     mod ospeedr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed0 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed1 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed2 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed3 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed4 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed5 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed6 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed7 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed8 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed9 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed10 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed11 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed12 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed13 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed14 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed15 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//     }

//     #[register(auto_increment)]
//     mod pupdr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd0 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd1 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd2 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd3 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd4 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd5 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd6 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd7 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd8 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd9 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd10 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd11 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd12 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd13 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd14 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd15 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//     }

//     #[register(auto_increment)]
//     mod idr {
//         #[field(width = 1, read)]
//         mod id0 {}
//         #[field(width = 1, read)]
//         mod id1 {}
//         #[field(width = 1, read)]
//         mod id2 {}
//         #[field(width = 1, read)]
//         mod id3 {}
//         #[field(width = 1, read)]
//         mod id4 {}
//         #[field(width = 1, read)]
//         mod id5 {}
//         #[field(width = 1, read)]
//         mod id6 {}
//         #[field(width = 1, read)]
//         mod id7 {}
//         #[field(width = 1, read)]
//         mod id8 {}
//         #[field(width = 1, read)]
//         mod id9 {}
//         #[field(width = 1, read)]
//         mod id10 {}
//         #[field(width = 1, read)]
//         mod id11 {}
//         #[field(width = 1, read)]
//         mod id12 {}
//         #[field(width = 1, read)]
//         mod id13 {}
//         #[field(width = 1, read)]
//         mod id14 {}
//         #[field(width = 1, read)]
//         mod id15 {}
//     }

//     #[register(auto_increment)]
//     mod odr {
//         #[field(width = 1, read, write)]
//         mod od0 {}
//         #[field(width = 1, read, write)]
//         mod od1 {}
//         #[field(width = 1, read, write)]
//         mod od2 {}
//         #[field(width = 1, read, write)]
//         mod od3 {}
//         #[field(width = 1, read, write)]
//         mod od4 {}
//         #[field(width = 1, read, write)]
//         mod od5 {}
//         #[field(width = 1, read, write)]
//         mod od6 {}
//         #[field(width = 1, read, write)]
//         mod od7 {}
//         #[field(width = 1, read, write)]
//         mod od8 {}
//         #[field(width = 1, read, write)]
//         mod od9 {}
//         #[field(width = 1, read, write)]
//         mod od10 {}
//         #[field(width = 1, read, write)]
//         mod od11 {}
//         #[field(width = 1, read, write)]
//         mod od12 {}
//         #[field(width = 1, read, write)]
//         mod od13 {}
//         #[field(width = 1, read, write)]
//         mod od14 {}
//         #[field(width = 1, read, write)]
//         mod od15 {}
//     }

//     // this could probably be
//     // stateful
//     #[register(auto_increment)]
//     mod bsrr {
//         #[field(width = 1, write, reset = 0)]
//         mod bs0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs15 {}

//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }

//     #[register(offset = 0x20, auto_increment)]
//     mod afrl {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel0 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel1 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel2 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel3 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel4 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel5 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel6 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel7 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod afrh {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel8 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel9 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel10 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel11 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel12 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel13 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel14 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel15 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod brr {
//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }
// }

// #[block(
//     base_addr = 0x4800_0c00,
//     entitlements = [super::super::rcc::ahb2enr::gpioden::Enabled],
//     auto_increment,
// )]
// pub mod gpiod {
//     #[register(auto_increment)]
//     mod moder {
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode0 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode1 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode2 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode3 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode4 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode5 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode6 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode7 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode8 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode9 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode10 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode11 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode12 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode13 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode14 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode15 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//     }

//     #[register(auto_increment)]
//     mod otyper {
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot0 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot1 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot2 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot3 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot4 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot5 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot6 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot7 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot8 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot9 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot10 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot11 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot12 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot13 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot14 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot15 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//     }

//     #[register(auto_increment)]
//     mod ospeedr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed0 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed1 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed2 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed3 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed4 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed5 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed6 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed7 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed8 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed9 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed10 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed11 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed12 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed13 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed14 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed15 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//     }

//     #[register(auto_increment)]
//     mod pupdr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd0 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd1 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd2 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd3 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd4 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd5 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd6 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd7 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd8 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd9 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd10 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd11 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd12 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd13 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd14 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd15 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//     }

//     #[register(auto_increment)]
//     mod idr {
//         #[field(width = 1, read)]
//         mod id0 {}
//         #[field(width = 1, read)]
//         mod id1 {}
//         #[field(width = 1, read)]
//         mod id2 {}
//         #[field(width = 1, read)]
//         mod id3 {}
//         #[field(width = 1, read)]
//         mod id4 {}
//         #[field(width = 1, read)]
//         mod id5 {}
//         #[field(width = 1, read)]
//         mod id6 {}
//         #[field(width = 1, read)]
//         mod id7 {}
//         #[field(width = 1, read)]
//         mod id8 {}
//         #[field(width = 1, read)]
//         mod id9 {}
//         #[field(width = 1, read)]
//         mod id10 {}
//         #[field(width = 1, read)]
//         mod id11 {}
//         #[field(width = 1, read)]
//         mod id12 {}
//         #[field(width = 1, read)]
//         mod id13 {}
//         #[field(width = 1, read)]
//         mod id14 {}
//         #[field(width = 1, read)]
//         mod id15 {}
//     }

//     #[register(auto_increment)]
//     mod odr {
//         #[field(width = 1, read, write)]
//         mod od0 {}
//         #[field(width = 1, read, write)]
//         mod od1 {}
//         #[field(width = 1, read, write)]
//         mod od2 {}
//         #[field(width = 1, read, write)]
//         mod od3 {}
//         #[field(width = 1, read, write)]
//         mod od4 {}
//         #[field(width = 1, read, write)]
//         mod od5 {}
//         #[field(width = 1, read, write)]
//         mod od6 {}
//         #[field(width = 1, read, write)]
//         mod od7 {}
//         #[field(width = 1, read, write)]
//         mod od8 {}
//         #[field(width = 1, read, write)]
//         mod od9 {}
//         #[field(width = 1, read, write)]
//         mod od10 {}
//         #[field(width = 1, read, write)]
//         mod od11 {}
//         #[field(width = 1, read, write)]
//         mod od12 {}
//         #[field(width = 1, read, write)]
//         mod od13 {}
//         #[field(width = 1, read, write)]
//         mod od14 {}
//         #[field(width = 1, read, write)]
//         mod od15 {}
//     }

//     // this could probably be
//     // stateful
//     #[register(auto_increment)]
//     mod bsrr {
//         #[field(width = 1, write, reset = 0)]
//         mod bs0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs15 {}

//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }

//     #[register(offset = 0x20, auto_increment)]
//     mod afrl {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel0 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel1 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel2 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel3 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel4 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel5 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel6 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel7 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod afrh {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel8 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel9 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel10 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel11 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel12 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel13 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel14 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel15 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod brr {
//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }
// }

// #[block(
//     base_addr = 0x4800_1000,
//     entitlements = [super::super::rcc::ahb2enr::gpioeen::Enabled],
//     auto_increment,
// )]
// pub mod gpioe {
//     #[register(auto_increment)]
//     mod moder {
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode0 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode1 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode2 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode3 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode4 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode5 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode6 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode7 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode8 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode9 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode10 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode11 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode12 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode13 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode14 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode15 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//     }

//     #[register(auto_increment)]
//     mod otyper {
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot0 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot1 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot2 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot3 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot4 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot5 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot6 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot7 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot8 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot9 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot10 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot11 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot12 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot13 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot14 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot15 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//     }

//     #[register(auto_increment)]
//     mod ospeedr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed0 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed1 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed2 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed3 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed4 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed5 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed6 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed7 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed8 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed9 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed10 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed11 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed12 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed13 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed14 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed15 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//     }

//     #[register(auto_increment)]
//     mod pupdr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd0 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd1 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd2 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd3 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd4 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd5 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd6 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd7 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd8 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd9 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd10 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd11 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd12 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd13 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd14 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd15 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//     }

//     #[register(auto_increment)]
//     mod idr {
//         #[field(width = 1, read)]
//         mod id0 {}
//         #[field(width = 1, read)]
//         mod id1 {}
//         #[field(width = 1, read)]
//         mod id2 {}
//         #[field(width = 1, read)]
//         mod id3 {}
//         #[field(width = 1, read)]
//         mod id4 {}
//         #[field(width = 1, read)]
//         mod id5 {}
//         #[field(width = 1, read)]
//         mod id6 {}
//         #[field(width = 1, read)]
//         mod id7 {}
//         #[field(width = 1, read)]
//         mod id8 {}
//         #[field(width = 1, read)]
//         mod id9 {}
//         #[field(width = 1, read)]
//         mod id10 {}
//         #[field(width = 1, read)]
//         mod id11 {}
//         #[field(width = 1, read)]
//         mod id12 {}
//         #[field(width = 1, read)]
//         mod id13 {}
//         #[field(width = 1, read)]
//         mod id14 {}
//         #[field(width = 1, read)]
//         mod id15 {}
//     }

//     #[register(auto_increment)]
//     mod odr {
//         #[field(width = 1, read, write)]
//         mod od0 {}
//         #[field(width = 1, read, write)]
//         mod od1 {}
//         #[field(width = 1, read, write)]
//         mod od2 {}
//         #[field(width = 1, read, write)]
//         mod od3 {}
//         #[field(width = 1, read, write)]
//         mod od4 {}
//         #[field(width = 1, read, write)]
//         mod od5 {}
//         #[field(width = 1, read, write)]
//         mod od6 {}
//         #[field(width = 1, read, write)]
//         mod od7 {}
//         #[field(width = 1, read, write)]
//         mod od8 {}
//         #[field(width = 1, read, write)]
//         mod od9 {}
//         #[field(width = 1, read, write)]
//         mod od10 {}
//         #[field(width = 1, read, write)]
//         mod od11 {}
//         #[field(width = 1, read, write)]
//         mod od12 {}
//         #[field(width = 1, read, write)]
//         mod od13 {}
//         #[field(width = 1, read, write)]
//         mod od14 {}
//         #[field(width = 1, read, write)]
//         mod od15 {}
//     }

//     // this could probably be
//     // stateful
//     #[register(auto_increment)]
//     mod bsrr {
//         #[field(width = 1, write, reset = 0)]
//         mod bs0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs15 {}

//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }

//     #[register(offset = 0x20, auto_increment)]
//     mod afrl {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel0 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel1 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel2 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel3 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel4 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel5 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel6 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel7 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod afrh {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel8 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel9 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel10 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel11 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel12 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel13 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel14 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel15 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod brr {
//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }
// }

// #[block(
//     base_addr = 0x4800_1400,
//     entitlements = [super::super::rcc::ahb2enr::gpiofen::Enabled],
//     auto_increment,
// )]
// pub mod gpiof {
//     #[register(auto_increment)]
//     mod moder {
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode0 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode1 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode2 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode3 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode4 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode5 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode6 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode7 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode8 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode9 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode10 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode11 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode12 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode13 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode14 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode15 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//     }

//     #[register(auto_increment)]
//     mod otyper {
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot0 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot1 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot2 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot3 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot4 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot5 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot6 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot7 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot8 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot9 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot10 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot11 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot12 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot13 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot14 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot15 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//     }

//     #[register(auto_increment)]
//     mod ospeedr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed0 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed1 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed2 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed3 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed4 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed5 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed6 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed7 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed8 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed9 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed10 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed11 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed12 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed13 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed14 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed15 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//     }

//     #[register(auto_increment)]
//     mod pupdr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd0 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd1 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd2 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd3 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd4 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd5 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd6 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd7 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd8 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd9 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd10 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd11 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd12 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd13 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd14 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd15 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//     }

//     #[register(auto_increment)]
//     mod idr {
//         #[field(width = 1, read)]
//         mod id0 {}
//         #[field(width = 1, read)]
//         mod id1 {}
//         #[field(width = 1, read)]
//         mod id2 {}
//         #[field(width = 1, read)]
//         mod id3 {}
//         #[field(width = 1, read)]
//         mod id4 {}
//         #[field(width = 1, read)]
//         mod id5 {}
//         #[field(width = 1, read)]
//         mod id6 {}
//         #[field(width = 1, read)]
//         mod id7 {}
//         #[field(width = 1, read)]
//         mod id8 {}
//         #[field(width = 1, read)]
//         mod id9 {}
//         #[field(width = 1, read)]
//         mod id10 {}
//         #[field(width = 1, read)]
//         mod id11 {}
//         #[field(width = 1, read)]
//         mod id12 {}
//         #[field(width = 1, read)]
//         mod id13 {}
//         #[field(width = 1, read)]
//         mod id14 {}
//         #[field(width = 1, read)]
//         mod id15 {}
//     }

//     #[register(auto_increment)]
//     mod odr {
//         #[field(width = 1, read, write)]
//         mod od0 {}
//         #[field(width = 1, read, write)]
//         mod od1 {}
//         #[field(width = 1, read, write)]
//         mod od2 {}
//         #[field(width = 1, read, write)]
//         mod od3 {}
//         #[field(width = 1, read, write)]
//         mod od4 {}
//         #[field(width = 1, read, write)]
//         mod od5 {}
//         #[field(width = 1, read, write)]
//         mod od6 {}
//         #[field(width = 1, read, write)]
//         mod od7 {}
//         #[field(width = 1, read, write)]
//         mod od8 {}
//         #[field(width = 1, read, write)]
//         mod od9 {}
//         #[field(width = 1, read, write)]
//         mod od10 {}
//         #[field(width = 1, read, write)]
//         mod od11 {}
//         #[field(width = 1, read, write)]
//         mod od12 {}
//         #[field(width = 1, read, write)]
//         mod od13 {}
//         #[field(width = 1, read, write)]
//         mod od14 {}
//         #[field(width = 1, read, write)]
//         mod od15 {}
//     }

//     // this could probably be
//     // stateful
//     #[register(auto_increment)]
//     mod bsrr {
//         #[field(width = 1, write, reset = 0)]
//         mod bs0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs15 {}

//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }

//     #[register(offset = 0x20, auto_increment)]
//     mod afrl {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel0 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel1 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel2 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel3 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel4 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel5 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel6 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel7 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod afrh {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel8 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel9 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel10 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel11 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel12 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel13 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel14 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel15 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod brr {
//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }
// }

// #[block(
//     base_addr = 0x4800_1800,
//     entitlements = [super::super::rcc::ahb2enr::gpiogen::Enabled],
//     auto_increment,
// )]
// pub mod gpiog {
//     #[register(auto_increment)]
//     mod moder {
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode0 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode1 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode2 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode3 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode4 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode5 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode6 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode7 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode8 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode9 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode10 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode11 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode12 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode13 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode14 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod mode15 {
//             #[variant]
//             struct Input;
//             #[variant]
//             struct Output;
//             #[variant]
//             struct Alternate;
//             #[variant(reset)]
//             struct Analog;
//         }
//     }

//     #[register(auto_increment)]
//     mod otyper {
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot0 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot1 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot2 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot3 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot4 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot5 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot6 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot7 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot8 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot9 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot10 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot11 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot12 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot13 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot14 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//         #[field(width = 1, read, write, auto_increment)]
//         mod ot15 {
//             #[variant(reset)]
//             struct PushPull;
//             #[variant]
//             struct OpenDrain;
//         }
//     }

//     #[register(auto_increment)]
//     mod ospeedr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed0 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed1 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed2 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed3 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed4 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed5 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed6 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed7 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed8 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed9 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed10 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed11 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed12 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed13 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed14 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod ospeed15 {
//             #[variant(reset)]
//             struct Low;
//             #[variant]
//             struct Medium;
//             #[variant]
//             struct High;
//             #[variant]
//             struct VeryHigh;
//         }
//     }

//     #[register(auto_increment)]
//     mod pupdr {
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd0 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd1 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd2 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd3 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd4 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd5 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd6 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd7 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd8 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd9 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd10 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd11 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd12 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd13 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd14 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//         #[field(width = 2, read, write, auto_increment)]
//         mod pupd15 {
//             #[variant(reset)]
//             struct None;
//             #[variant]
//             struct PullUp;
//             #[variant]
//             struct PullDown;
//         }
//     }

//     #[register(auto_increment)]
//     mod idr {
//         #[field(width = 1, read)]
//         mod id0 {}
//         #[field(width = 1, read)]
//         mod id1 {}
//         #[field(width = 1, read)]
//         mod id2 {}
//         #[field(width = 1, read)]
//         mod id3 {}
//         #[field(width = 1, read)]
//         mod id4 {}
//         #[field(width = 1, read)]
//         mod id5 {}
//         #[field(width = 1, read)]
//         mod id6 {}
//         #[field(width = 1, read)]
//         mod id7 {}
//         #[field(width = 1, read)]
//         mod id8 {}
//         #[field(width = 1, read)]
//         mod id9 {}
//         #[field(width = 1, read)]
//         mod id10 {}
//         #[field(width = 1, read)]
//         mod id11 {}
//         #[field(width = 1, read)]
//         mod id12 {}
//         #[field(width = 1, read)]
//         mod id13 {}
//         #[field(width = 1, read)]
//         mod id14 {}
//         #[field(width = 1, read)]
//         mod id15 {}
//     }

//     #[register(auto_increment)]
//     mod odr {
//         #[field(width = 1, read, write)]
//         mod od0 {}
//         #[field(width = 1, read, write)]
//         mod od1 {}
//         #[field(width = 1, read, write)]
//         mod od2 {}
//         #[field(width = 1, read, write)]
//         mod od3 {}
//         #[field(width = 1, read, write)]
//         mod od4 {}
//         #[field(width = 1, read, write)]
//         mod od5 {}
//         #[field(width = 1, read, write)]
//         mod od6 {}
//         #[field(width = 1, read, write)]
//         mod od7 {}
//         #[field(width = 1, read, write)]
//         mod od8 {}
//         #[field(width = 1, read, write)]
//         mod od9 {}
//         #[field(width = 1, read, write)]
//         mod od10 {}
//         #[field(width = 1, read, write)]
//         mod od11 {}
//         #[field(width = 1, read, write)]
//         mod od12 {}
//         #[field(width = 1, read, write)]
//         mod od13 {}
//         #[field(width = 1, read, write)]
//         mod od14 {}
//         #[field(width = 1, read, write)]
//         mod od15 {}
//     }

//     // this could probably be
//     // stateful
//     #[register(auto_increment)]
//     mod bsrr {
//         #[field(width = 1, write, reset = 0)]
//         mod bs0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod bs15 {}

//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }

//     #[register(offset = 0x20, auto_increment)]
//     mod afrl {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel0 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel1 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel2 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel3 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel4 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel5 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel6 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel7 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod afrh {
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel8 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel9 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel10 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel11 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel12 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel13 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel14 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//         #[field(width = 4, read, write, auto_increment)]
//         mod afsel15 {
//             #[variant(reset)]
//             struct AF0;
//             #[variant]
//             struct AF1;
//             #[variant]
//             struct AF2;
//             #[variant]
//             struct AF3;
//             #[variant]
//             struct AF4;
//             #[variant]
//             struct AF5;
//             #[variant]
//             struct AF6;
//             #[variant]
//             struct AF7;
//             #[variant]
//             struct AF8;
//             #[variant]
//             struct AF9;
//             #[variant]
//             struct AF10;
//             #[variant]
//             struct AF11;
//             #[variant]
//             struct AF12;
//             #[variant]
//             struct AF13;
//             #[variant]
//             struct AF14;
//             #[variant]
//             struct AF15;
//         }
//     }

//     #[register(auto_increment)]
//     mod brr {
//         #[field(width = 1, write, reset = 0)]
//         mod br0 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br1 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br2 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br3 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br4 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br5 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br6 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br7 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br8 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br9 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br10 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br11 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br12 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br13 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br14 {}
//         #[field(width = 1, write, reset = 0)]
//         mod br15 {}
//     }
// }
