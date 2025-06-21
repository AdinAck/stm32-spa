fn main() {
    env_logger::init();
    proto_hal_build::codegen::validate(model::generate);
}
