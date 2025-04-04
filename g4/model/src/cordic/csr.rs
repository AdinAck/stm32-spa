use proto_hal_build::ir::structures::{
    entitlement::Entitlement,
    field::{Field, Numericity},
    register::Register,
    variant::Variant,
};

// // #[field(width = 3, auto_increment)]
// enum Scale {
//     // #[variant_array(bits = 0, range = ..8)]
//     // NX
//     N0,
//     N1,
//     N2,
//     N3,
//     N4,
//     N5,
//     N6,
//     N7,
// }

// // #[field(width = 4, auto_increment)]
// enum Func {
//     // #[variant(entitlements = [cordic::csr::scale::N0])]
//     Cos,
//     // #[variant(entitlements = [cordic::csr::scale::N0])]
//     Sin,
//     // #[variant(entitlements = [cordic::csr::scale::N0])]
//     Atan2,
//     // #[variant(entitlements = [cordic::csr::scale::N0])]
//     Magnitude,
//     // #[variant]
//     Atan,
//     // #[variant(entitlements = [cordic::csr::scale::N1])]
//     CosH,
//     // #[variant(entitlements = [cordic::csr::scale::N1])]
//     SinH,
//     // #[variant(entitlements = [cordic::csr::scale::N1])]
//     ATanH,
//     // #[variant(entitlements = [cordic::csr::scale::N1, cordic::csr::scale::N2, cordic::csr::scale::N3, cordic::csr::scale::N4])]
//     Ln,
//     // #[variant(entitlements = [cordic::csr::scale::N0, cordic::csr::scale::N1, cordic::csr::scale::N2])]
//     Sqrt,
// }

// // #[register]
// pub struct Csr {
//     // #[field(write, reset = Cos)]
//     func: Func,
//     // #[field(write, reset = P20)]
//     // precision: Precision,
//     // #[field(write, reset = N0)]
//     scale: Scale,
//     // #[field(offset = 16, write, reset = Disabled)]
//     // ien: Enable,
//     // #[field(write, reset = Disabled)]
//     // dmaren: Enable,
//     // #[field(write, reset = Disabled)]
//     // dmawen: Enable,
//     // #[field(write, reset = OneRead)]
//     // nres: Nres,
//     // #[field(write, reset = OneWrite)]
//     // nargs: Nargs,
//     // #[field(write, reset = Q31)]
//     // ressize: DataSize,
//     // #[field(write, reset = Q31)]
//     // argsize: DataSize,
//     // #[field(offset = 31)]
//     // rrdy: Rrdy,
// }

fn func() -> Field {
    let variants = [
        ("cos", vec![Entitlement::to("cordic::csr::scale::N0")]),
        ("sin", vec![Entitlement::to("cordic::csr::scale::N0")]),
        ("atan2", vec![Entitlement::to("cordic::csr::scale::N0")]),
        ("magnitude", vec![Entitlement::to("cordic::csr::scale::N0")]),
        ("atan", vec![]),
        ("cosh", vec![Entitlement::to("cordic::csr::scale::N0")]),
        ("sinh", vec![Entitlement::to("cordic::csr::scale::N0")]),
        ("atanh", vec![Entitlement::to("cordic::csr::scale::N0")]),
        (
            "ln",
            vec![
                Entitlement::to("cordic::csr::scale::N1"),
                Entitlement::to("cordic::csr::scale::N2"),
                Entitlement::to("cordic::csr::scale::N3"),
                Entitlement::to("cordic::csr::scale::N4"),
            ],
        ),
        (
            "sqrt",
            vec![
                Entitlement::to("cordic::csr::scale::N0"),
                Entitlement::to("cordic::csr::scale::N1"),
                Entitlement::to("cordic::csr::scale::N2"),
            ],
        ),
    ];

    let variants = variants
        .into_iter()
        .enumerate()
        .map(|(bits, (ident, entitlements))| {
            Variant::new(ident, bits as u32).entitlements(entitlements)
        });

    Field::new("func", 0, 4, Numericity::enumerated(variants))
}

fn precision() -> Field {
    let variants = (1..16).map(|i| Variant::new(format!("P{}", i * 4), i));

    Field::new("precision", 4, 4, Numericity::enumerated(variants))
}

fn scale() -> Field {
    let variants = (0..8).map(|i| Variant::new(format!("N{i}"), i));

    Field::new("scale", 8, 3, Numericity::enumerated(variants))
}

pub fn generate() -> Register {
    Register::new("csr", 0, [func(), precision(), scale()])
}

// // == experimenting ==

// struct Entitlement2<T> {
//     _p: PhantomData<T>,
// }

// trait Variant2: Sized {
//     const BITS: u32;

//     fn entitlement(&self) -> Entitlement2<Self> {
//         Entitlement2 { _p: PhantomData }
//     }
// }

// macro_rules! variant {
//     {
//         $name: ident,
//         $bits: expr,
//         $( ( $( $entitlement:path $(,)? )* ) )?
//     } => {
//         pub struct $name {
//             #[allow(unused_parens)]
//             pub entitlements: ( $( $(Entitlement2<$entitlement>),* )? ),
//         }

//         impl Variant2 for $name {
//             const BITS: u32 = $bits;
//         }
//     };
// }

// mod scale {
//     use super::*;

//     variant! {
//         N0,
//         0,
//     }

//     pub struct Scale {
//         pub n0: N0,
//     }
// }

// mod func {
//     use super::*;

//     variant! {
//         Cos,
//         0,
//         (super::cordic::csr::scale::N0)
//     }

//     variant! {
//         Sin,
//         1,
//         (super::cordic::csr::scale::N0)
//     }

//     pub struct Func {
//         pub cos: Cos,
//         pub sin: Sin,
//     }
// }

// pub struct Csr {
//     scale: cordic::csr::scale::Scale,
//     func: func::Func,
// }

// impl Csr {
//     pub fn new() -> Self {
//         let scale = cordic::csr::scale::Scale {
//             n0: cordic::csr::scale::N0 { entitlements: () },
//         };

//         let func = func::Func {
//             cos: func::Cos {
//                 entitlements: scale.n0.entitlement(),
//             },
//             sin: func::Sin {
//                 entitlements: scale.n0.entitlement(),
//             },
//         };

//         Self { scale, func }
//     }
// }
