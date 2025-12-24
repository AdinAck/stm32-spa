use model::Configuration;
use proto_hal_macros::generate_macros;

generate_macros!({
    if cfg!(feature = "c092") {
        Configuration::c092()
    } else {
        Configuration::default()
    }
});
