//! GPIO Alternate Function Register
//!
//! AFRx registers contain fields which are used to set the alternate function of the respective channels.
//!
//! Each Alternate Function Select field contains 16 variants AF0 through AF15.

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};

/// GPIO peripherals implement this trait to attach Alternate Function registers.
pub trait Afr {
    /// Add a lower Alternate Function register to this peripheral.
    fn afrl<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx>;

    /// Add an upper Alternate Function register to this peripheral.
    fn afrh<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx>;
}

impl<'cx> Afr for PeripheralEntry<'cx> {
    fn afrl<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx> {
        afr(self, "afrl", offset)
    }

    fn afrh<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx> {
        afr(self, "afrh", offset)
    }
}

fn afr<'cx, 'ncx>(
    afr: &'ncx mut PeripheralEntry<'cx>,
    name: &str,
    offset: u32,
) -> RegisterEntry<'ncx> {
    afr.add_register(Register::new(name, offset).reset(0))
}

/// Alternate Function registers implement this trait to attach Alternate Function Selection fields.
pub trait Afsel {
    /// Add an Alternate Function Selection field to this register.
    fn afsel<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store>;
}

impl<'cx> Afsel for RegisterEntry<'cx> {
    fn afsel<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store> {
        self.add_store_field(Field::new(
            format!("afsel{position}"),
            (position * 4) % 32,
            4,
        ))
    }
}

pub type Output = [afsel::Output; 8];

pub mod afsel {
    use std::array;

    use phm::{Entitlement, Variant, field::access, model::FieldEntry};

    pub trait Afsel {
        fn afsel(&mut self) -> Output;
    }

    impl<'cx> Afsel for FieldEntry<'cx, access::Store> {
        fn afsel(&mut self) -> Output {
            array::from_fn(|i| {
                self.add_variant(Variant::new(format!("AF{i}"), i as _))
                    .make_entitlement()
            })
        }
    }

    pub type Output = [Entitlement; 16];
}
