use enum_iterator::all;
use stm32_spa_model::compose;

fn main() {
    for variant in all() {
        println!("=== Variant: {variant:?} ===");
        phm::validate(compose(Some(variant)));
    }
}
