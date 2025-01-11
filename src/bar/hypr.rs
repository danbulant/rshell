use std::sync::Arc;

use cushy::{
    styles::{
        components::{CornerRadius, TextColor, WidgetBackground},
        Styles,
    },
    value::{Destination, Dynamic, Source},
    widget::{MakeWidget, WidgetList},
    widgets::Style,
};
use hyprland::{
    data::{Clients, Devices, Monitors, Workspaces},
    event_listener::AsyncEventListener,
    shared::{HyprData, HyprDataVec, HyprError},
};

use crate::{
    rt::tokio_runtime,
    theme::{
        BG_WORKSPACE_ACTIVE, TEXT_TOOL, TEXT_WORKSPACE_ACTIVE, WIDGET_PADDING,
        WORKSPACE_CORNER_RADIUS, WORKSPACE_PADDING,
    },
};

use super::HyprlandState;

pub fn get_hyprland_state_sync() -> Result<HyprlandState, HyprError> {
    let monitors = Monitors::get()?.to_vec();
    let mut workspaces = Workspaces::get()?.to_vec();
    workspaces.sort_by_key(|a| a.id);
    let devices = Devices::get()?;
    let client = Clients::get()?.to_vec();
    Ok(HyprlandState {
        monitors,
        workspaces,
        devices,
        clients: client,
    })
}

pub async fn get_hyprland_state() -> Result<HyprlandState, HyprError> {
    let monitors = Monitors::get_async().await?.to_vec();
    let mut workspaces = Workspaces::get_async().await?.to_vec();
    workspaces.sort_by_key(|a| a.id);
    let devices = Devices::get_async().await?;
    let client = Clients::get_async().await?.to_vec();
    Ok(HyprlandState {
        monitors,
        workspaces,
        devices,
        clients: client,
    })
}

pub fn init_callbacks(state: Dynamic<HyprlandState>) {
    tokio_runtime().spawn({
        let state = state.clone();
        async move {
            let mut event_listener = AsyncEventListener::new();

            let update = Arc::new(move || {
                let state = state.clone();
                Box::pin(async move {
                    if let Ok(unwrap) = get_hyprland_state().await {
                        state.set(unwrap);
                    }
                })
            });

            // I prefer winit-like event loop iterators/single callback with enum...

            event_listener.add_workspace_added_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_workspace_changed_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_workspace_deleted_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_window_opened_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_window_closed_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_window_title_changed_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_urgent_state_changed_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_screencast_handler({
                let update = update.clone();
                move |_| update()
            });
            event_listener.add_layout_changed_handler({
                let update = update.clone();
                move |_| update()
            });

            event_listener.start_listener_async().await.unwrap();
        }
    });
}

pub fn hyprland_workspaces(state: Dynamic<HyprlandState>) -> impl MakeWidget {
    state.map_each(|state| {
        let monitor = state.monitors.first().unwrap();
        let workspaces = &state.workspaces;
        WidgetList::from_iter(workspaces.iter().map(|w| {
            let active = monitor.active_workspace.id == w.id;

            let name = w.name.clone();

            if active {
                name.pad_by(WORKSPACE_PADDING)
                    .centered()
                    .with(&WidgetBackground, BG_WORKSPACE_ACTIVE)
                    .with(&TextColor, TEXT_WORKSPACE_ACTIVE)
                    .with(&CornerRadius, WORKSPACE_CORNER_RADIUS)
                    .make_widget()
            } else {
                name.pad_by(WORKSPACE_PADDING).centered().make_widget()
            }
        }))
        .into_columns()
        .make_widget()
    })
}

pub fn hyprland_active_title(state: Dynamic<HyprlandState>) -> impl MakeWidget {
    state.map_each(|state| {
        let monitor = state.monitors.first().unwrap();
        let active_workspace = state
            .workspaces
            .iter()
            .find(|w| w.id == monitor.active_workspace.id)
            .unwrap();

        format!("󱄅 {}", active_workspace.last_window_title.clone())
            .with(&TextColor, TEXT_TOOL)
            .pad_by(WIDGET_PADDING)
            .centered()
            .make_widget()
    })
}
