mod csr;

// use csr::Csr;
use proto_hal_build::ir::structures::{peripheral::Peripheral, register::Register};

// // #[peripheral(base_addr = 0x4002_0c00, auto_increment)]
// pub struct Cordic {
//     // #[register]
//     csr: Csr,
//     // #[register]
//     // wdata: Wdata,
//     // #[register]
//     // rdata: Rdata,
// }

pub fn generate() -> Peripheral {
    let wdata = Register::new("wdata", 4, []);
    let rdata = Register::new("rdata", 8, []);

    let cordic = Peripheral::new("cordic", 0x4002_0c00, [csr::generate(), wdata, rdata]);

    cordic
}
