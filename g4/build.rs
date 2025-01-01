use g4_interrupts::INTERRUPT_IDENTS;

fn main() {
    proto_hal_build::interrupts::build(INTERRUPT_IDENTS);
}
