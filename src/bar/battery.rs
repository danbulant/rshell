use battery::{Battery, State};
use cushy::{
    styles::components::TextColor,
    value::{Destination, Dynamic},
    widget::MakeWidget,
};

use crate::{
    rt::tokio_runtime,
    theme::{TEXT_BATTERY, WIDGET_PADDING},
};

const BATTERY_LOW: &str = "󱃍";

const BATTERY_CHARGING: [&str; 11] = ["󰢟", "󰢜", "󰂆", "󰂇", "󰂈", "󰢝", "󰂉", "󰢞", "󰂊", "󰂋", "󰂅"];
const BATTERY_NORMAL: [&str; 11] = ["󰂎", "󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];

const BATTERY_UNKNOWN: &str = "󰂑";
const BATTERY_FULL: &str = "󱟢";

fn format_battery(battery: &Battery) -> String {
    let state = battery.state();
    let charge = battery.state_of_charge();

    let icon = match state {
        State::Charging => BATTERY_CHARGING[(charge.value * 10.) as usize],
        State::Discharging => BATTERY_NORMAL[(charge.value * 10.) as usize],
        State::Empty => BATTERY_LOW,
        State::Full => BATTERY_FULL,
        State::Unknown | _ => BATTERY_UNKNOWN,
    };

    let percent = (charge.value * 100.) as u8;
    format!(" {} {}% ", icon, percent)
}

pub fn battery() -> impl MakeWidget {
    let manager = battery::Manager::new();
    let Ok(manager) = manager else {
        return "".make_widget();
    };
    let Ok(mut batteries) = manager.batteries() else {
        return "".make_widget();
    };

    // for (idx, maybe_battery) in batteries.enumerate() {
    //     let Ok(battery) = maybe_battery else { continue };
    //     println!("Battery #{}:", idx);
    //     println!("Vendor: {:?}", battery.vendor());
    //     println!("Model: {:?}", battery.model());
    //     println!("State: {:?}", battery.state());
    //     println!("Time to full charge: {:?}", battery.time_to_full());
    //     println!("Time to empty: {:?}", battery.time_to_empty());
    //     println!("State of charge: {:?}", battery.state_of_charge());
    //     println!("State of health: {:?}", battery.state_of_health());
    //     println!("Energy rate: {:?}", battery.energy_rate());
    //     println!("Temperature: {:?}", battery.temperature());
    //     println!("State: {:?}", battery.state());
    //     println!("");
    // }

    let Some(Ok(battery)) = batteries.next() else {
        return "".make_widget();
    };
    let info = Dynamic::new(format_battery(&battery));

    tokio_runtime().spawn({
        let info = info.clone();
        async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;

                let Ok(manager) = battery::Manager::new() else {
                    info.set(BATTERY_UNKNOWN.to_string());
                    return;
                };
                let Ok(mut batteries) = manager.batteries() else {
                    info.set(BATTERY_UNKNOWN.to_string());
                    return;
                };
                let Some(Ok(battery)) = batteries.next() else {
                    info.set(BATTERY_UNKNOWN.to_string());
                    return;
                };
                info.set(format_battery(&battery));
            }
        }
    });

    info.with(&TextColor, TEXT_BATTERY)
        .pad_by(WIDGET_PADDING)
        .centered()
        .make_widget()
}
