use cushy::{
    styles::components::TextColor,
    value::{Destination, Dynamic, Source},
    widget::MakeWidget,
};
use systemstat::{saturating_sub_bytes, Platform, System};

use crate::{
    rt::tokio_runtime,
    theme::{TEXT_MEM, WIDGET_PADDING},
};

fn get_memory_usage() -> f64 {
    let sys = System::new();
    sys.memory()
        .map(|mem| {
            let used = saturating_sub_bytes(mem.total, mem.free);
            let used_percentage = (used.as_u64() as f64 / mem.total.as_u64() as f64) * 100.;
            used_percentage
        })
        .unwrap_or(0.)
}

pub fn memory_widget() -> impl MakeWidget {
    let percentage = Dynamic::new(get_memory_usage());
    tokio_runtime().spawn({
        let current_time = percentage.clone();
        async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(1000));
            loop {
                interval.tick().await;
                current_time.set(get_memory_usage());
            }
        }
    });

    percentage
        .map_each(|percentage| {
            let icon = "".to_string();
            format!(" {}   {:.0}%", icon, percentage)
        })
        .with(&TextColor, TEXT_MEM)
        .pad_by(WIDGET_PADDING)
        .centered()
}
