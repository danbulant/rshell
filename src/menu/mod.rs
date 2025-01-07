use std::sync::Arc;

use cushy::{
    kludgine::{app::winit::window::WindowLevel, Color},
    value::{Dynamic, Source},
    widget::{MakeWidget, MakeWidgetList},
    widgets::{input::InputValue, Stack},
    Application, Open,
};
use freedesktop_desktop_entry::{default_paths, get_languages_from_env, Iter};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use which::which;

struct AppMenuEntry {
    name: String,
    icon: Option<String>,
    keywords: Vec<String>,
    start: AppMenuStart,
}

struct AppExecutable {
    exec: String,
    exec_path: Option<String>,
}

enum AppMenuStart {
    Executable(AppExecutable),
    Url(String),
}

struct SearchResult {
    app: usize,
    fuzzy: i64,
}

pub fn start_menu(app: &mut impl Application) -> cushy::Result {
    let locales = get_languages_from_env();

    let mut entries: Vec<AppMenuEntry> = Vec::new();

    for entry in Iter::new(default_paths()).entries(Some(&locales)) {
        let Some(name) = entry.name(&locales) else {
            continue;
        };
        let name = name.into();

        let Some(type_) = entry.type_() else { continue };
        if let Some(try_exec) = entry.desktop_entry("TryExec") {
            let path = std::path::Path::new(try_exec);
            if path.is_absolute() {
                if !path.exists() {
                    continue;
                }
            } else {
                let res = which(path);
                if res.is_err() {
                    continue;
                }
            }
        }

        let icon = entry.icon().map(Into::into);
        let keywords = entry
            .keywords(&locales)
            .map(|s| s.into_iter().map(Into::into).collect())
            .unwrap_or_default();

        let start = match type_.to_lowercase().as_str() {
            "application" => {
                let Some(exec) = entry.exec().map(Into::into) else {
                    continue;
                };
                let exec_path = entry.desktop_entry("Path").map(Into::into);
                AppMenuStart::Executable(AppExecutable { exec, exec_path })
            }
            "link" => {
                let Some(url) = entry.desktop_entry("URL") else {
                    continue;
                };
                AppMenuStart::Url(url.into())
            }
            _ => continue,
        };

        entries.push(AppMenuEntry {
            name,
            icon,
            keywords,
            start,
        });
    }

    let entries = Arc::new(entries);

    let search_text = Dynamic::new(String::new());
    let results = search_text.map_each({
        let entries = entries.clone();
        let matcher = SkimMatcherV2::default();
        move |search_text| {
            entries
                .iter()
                .enumerate()
                .filter_map(|(i, e)| {
                    let mut max_score = matcher.fuzzy_match(&e.name, &search_text);

                    for keyword in &e.keywords {
                        let score = matcher.fuzzy_match(&keyword, &search_text);
                        if score > max_score {
                            max_score = score;
                        }
                    }

                    max_score.map(|max_score| SearchResult {
                        app: i,
                        fuzzy: max_score,
                    })
                })
                .map(|res| {
                    let entry = &entries[res.app];

                    entry.name.clone()
                })
                .make_widget_list()
        }
    });

    let search = search_text.to_input().placeholder("Search");

    let mut window = search
        .and(Stack::rows(results))
        .into_rows()
        .pad()
        .background_color(Color::BLACK.with_alpha(1))
        .into_window()
        .transparent()
        .app_name("rshell")
        .decorated(false)
        .window_level(WindowLevel::AlwaysOnTop);

    window
        .sans_serif_font_family
        .push(cushy::styles::FamilyOwned::Name("Iosevka NF".into()));

    window.open(app).map(|_| ())
}
