use proto_hal_build::ir::{
    access::Access,
    structures::{
        entitlement::Entitlement,
        field::{Field, Numericity},
    },
};

pub fn generate() -> Field {
    Field::new("res0", 0, 16, Access::read(Numericity::Numeric))
        .entitlements([Entitlement::to("cordic::csr::ressize::Q15")])
}
