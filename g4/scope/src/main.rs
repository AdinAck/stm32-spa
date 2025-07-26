use std::{collections::HashMap, time::Duration};

use probe_rs::{Core, MemoryInterface, Permissions};
use proto_hal_build::ir::structures::field::Numericity;

fn main() {
    let cortex_m_model = cortex_m_model::generate().unwrap();
    let model = model::generate(model::DeviceVariant::G474).unwrap();

    let mut session = probe_rs::probe::list::Lister::new()
        .list_all()
        .first()
        .unwrap()
        .open()
        .unwrap()
        .attach_under_reset("stm32g474re", Permissions::new())
        .unwrap();
    let mut core = session.core(0).unwrap();
    core.reset_and_halt(Duration::from_millis(100)).unwrap();

    let mut values = HashMap::new();

    let paths = [
        ["nvic", "iser1"],
        ["nvic", "iabr1"],
        ["rcc", "ahb2enr"],
        ["exti", "imr1"],
        ["exti", "rtsr1"],
        ["exti", "pr1"],
        ["gpioa", "moder"],
        ["gpioa", "odr"],
    ];

    let mut peripherals = cortex_m_model
        .peripherals
        .values()
        .chain(model.peripherals.values())
        .filter(|peripheral| {
            paths
                .iter()
                .any(|[p, _]| p == &peripheral.ident.to_string().as_str())
        })
        .collect::<Vec<_>>();

    peripherals.sort_by(|lhs, rhs| lhs.base_addr.cmp(&rhs.base_addr));

    let stream = rerun::RecordingStreamBuilder::new("proto-hal-scope")
        .spawn()
        .unwrap();

    let mut i = 0;

    let mut report = |core: &mut Core<'_>| {
        stream.set_time_sequence("poll", i);
        for peripheral in &peripherals {
            let mut registers = peripheral
                .registers
                .values()
                .filter(|register| {
                    paths.iter().any(|[p, r]| {
                        p == &peripheral.ident.to_string().as_str()
                            && r == &register.ident.to_string().as_str()
                    }) && register.fields.values().any(|field| field.access.is_read())
                })
                .collect::<Vec<_>>();
            registers.sort_by(|lhs, rhs| lhs.offset.cmp(&rhs.offset));

            for register in registers {
                let key = format!("{}::{}", peripheral.ident, register.ident);
                let empty = values.get(&key).is_none();
                let stored_value = values.entry(key).or_insert(0);

                let new_value = core
                    .read_word_32((peripheral.base_addr + register.offset) as _)
                    .unwrap();

                if *stored_value != new_value || empty {
                    *stored_value = new_value;

                    let mut fields = register.fields.values().collect::<Vec<_>>();
                    fields.sort_by(|lhs, rhs| lhs.offset.cmp(&rhs.offset));

                    for field in fields.iter().filter(|field| field.access.is_read()) {
                        let mask = u32::MAX >> (32 - field.width);
                        let raw = (new_value >> field.offset) & mask;

                        let value = match &field.access.get_read().unwrap().numericity {
                            Numericity::Numeric => format!("0x{raw:x}"),
                            Numericity::Enumerated { variants } => {
                                if let Some(variant) =
                                    variants.values().find(|variant| variant.bits == raw)
                                {
                                    format!("{} (0x{raw:x})", variant.ident)
                                } else {
                                    format!("Invalid(0x{raw:x})")
                                }
                            }
                        };

                        stream
                            .log(
                                format!("{}/{}/{}", peripheral.ident, register.ident, field.ident),
                                &rerun::AnyValues::new("field")
                                    .with_component::<rerun::components::Text>(
                                        field.ident.to_string(),
                                        [value],
                                    ),
                            )
                            .unwrap();
                    }
                }
            }
        }

        i += 1;
    };

    report(&mut core);

    core.run().unwrap();

    loop {
        report(&mut core);
    }
}
