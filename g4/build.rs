use interrupts::g4::INTERRUPT_IDENTS;

fn main() {
    proto_hal_build::interrupts::build(INTERRUPT_IDENTS);
}
