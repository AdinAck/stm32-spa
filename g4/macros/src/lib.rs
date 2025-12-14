use model::Configuration;
use proto_hal_macros::generate_macros;

generate_macros!({
    if cfg!(feature = "g431") {
        Configuration::g431()
    } else if cfg!(feature = "g441") {
        Configuration::g441()
    } else if cfg!(feature = "g474") {
        Configuration::g474()
    } else if cfg!(feature = "g484") {
        Configuration::g484()
    } else {
        Configuration::default()
    }
});
