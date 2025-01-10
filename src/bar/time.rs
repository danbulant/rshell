use chrono::{Local, Timelike};
use cushy::{
    styles::components::{TextColor, WidgetBackground},
    value::{Destination, Dynamic, MapEach},
    widget::MakeWidget,
};

use crate::{
    rt::tokio_runtime,
    theme::{BG_DEFAULT, TEXT_CLOCK, WIDGET_PADDING},
};

const FORMAT: &'static str = "  %H:%M %p 󰃭  %a %d ";
const FORMAT_ALT: &'static str = "  %H:%M  %b %Y ";

const CLOCK_ICONS: [&'static str; 12] =
    ["", "", "", "", "", "", "", "", "", "", "", ""];
const CLOCK_ICONS_ALT: [&'static str; 12] =
    ["󱑊", "󱐿", "󱑀", "󱑁", "󱑂", "󱑃", "󱑄", "󱑅", "󱑆", "󱑇", "󱑈", "󱑉"];

pub fn time_widget() -> impl MakeWidget {
    let current_time = Dynamic::new(Local::now());
    tokio_runtime().spawn({
        let current_time = current_time.clone();
        async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                current_time.set(Local::now());
            }
        }
    });
    let show_year = Dynamic::new(false);

    (&current_time, &show_year)
        .map_each(|(current_time, show_year)| {
            let (icon_set, format) = match show_year {
                true => (&CLOCK_ICONS_ALT, FORMAT_ALT),
                false => (&CLOCK_ICONS, FORMAT),
            };
            let icon = icon_set[current_time.hour12().1 as usize % 12].to_string();
            let icon = " ".to_string() + &icon;
            icon + &current_time.format(format).to_string()
        })
        .with(&TextColor, TEXT_CLOCK)
        .pad_by(WIDGET_PADDING)
        .centered()
}
