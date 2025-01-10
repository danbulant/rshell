use battery::State;
use cushy::{styles::components::TextColor, widget::MakeWidget};

use crate::theme::{TEXT_BATTERY, TEXT_TEMP, WIDGET_PADDING};

const BATTERY_LOW: &str = "󱃍";

const BATTERY_CHARGING: [&str; 11] = ["󰢟", "󰢜", "󰂆", "󰂇", "󰂈", "󰢝", "󰂉", "󰢞", "󰂊", "󰂋", "󰂅"];
const BATTERY_NORMAL: [&str; 11] = ["󰂎", "󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];

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

    let state = battery.state();
    let charge = battery.state_of_charge();

    let icon = match state {
        State::Charging => BATTERY_CHARGING[(charge.value * 10.) as usize],
        State::Discharging => BATTERY_NORMAL[(charge.value * 10.) as usize],
        State::Empty => BATTERY_LOW,
        State::Full => "󱟢",
        State::Unknown | _ => "󰂑",
    };

    let percent = (charge.value * 100.) as u8;

    format!(" {} {}% ", icon, percent)
        .with(&TextColor, TEXT_BATTERY)
        .pad_by(WIDGET_PADDING)
        .centered()
        .make_widget()
}
