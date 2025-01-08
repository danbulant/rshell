use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    rt::tokio_runtime,
    theme::{BG_DEFAULT, CORNER_RADIUS, TEXT_SPOTIFY},
    vibrancy::Vibrancy,
};
use cushy::{
    figures::{units::Lp, Size, Zero},
    kludgine::{AnyTexture, LazyTexture},
    styles::{
        components::{FontWeight, LineHeight, TextColor, TextSize, WidgetBackground},
        Color, CornerRadii, Dimension, DimensionRange, Weight,
    },
    value::{Destination, Dynamic, Source},
    widget::MakeWidget,
    widgets::{image::ImageCornerRadius, label::Displayable, Image},
};
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use image::{self, imageops::FilterType, Rgb};
use mpris::{LoopStatus, PlaybackStatus, PlayerFinder};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use tokio::{runtime, task::JoinHandle};

#[derive(PartialEq)]
struct PlayingTrack {
    title: String,
    artist: String,
    album: String,
    duration: Duration,

    album_art: Option<String>,

    status: PlaybackStatus,
    shuffle: bool,
    loop_status: LoopStatus,
}

/// Renders spotify control widget, the small one
pub fn spotify_controls() -> impl MakeWidget {
    let (progress, track) = get_track_dynamics();
    let (texture, vibrancy) = get_texture_dynamic(track.clone());

    const IMAGE_SIDE: i32 = 10 /* lineheight */ + 2 * 6 /* padding */;
    let image_size = Size::squared(DimensionRange::from(Dimension::Lp(Lp::points(IMAGE_SIDE))));

    Image::new(texture)
        .aspect_fit()
        .with(
            &ImageCornerRadius,
            CornerRadii {
                top_left: CORNER_RADIUS,
                top_right: Dimension::ZERO,
                bottom_left: CORNER_RADIUS,
                bottom_right: Dimension::ZERO,
            },
        )
        // default pad is 6, default line height is 16
        .size(image_size)
        .and(
            track
                .map_each(|track| {
                    if let Some(track) = track {
                        format!("{} - {}", track.artist, track.title,)
                    } else {
                        "No track playing".to_string()
                    }
                })
                .into_label()
                .with(&TextColor, TEXT_SPOTIFY)
                .with(&FontWeight, Weight::BOLD)
                .with(&TextSize, Dimension::Lp(Lp::points(8)))
                .with(&LineHeight, Dimension::Lp(Lp::points(10)))
                .centered()
                .pad(),
        )
        .into_columns()
        .with(&WidgetBackground, BG_DEFAULT)
    // .with(&WidgetBackground, vibrancy.map_each(|vib| vib.primary.unwrap_or(Color::BLACK).into()))
}

fn get_empty_texture() -> AnyTexture {
    AnyTexture::Lazy(LazyTexture::from_image(
        image::DynamicImage::ImageRgba8(image::ImageBuffer::new(1, 1)),
        cushy::kludgine::wgpu::FilterMode::Linear,
    ))
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ImageVibrancy {
    primary: Option<Color>,
    dark: Option<Color>,
    light: Option<Color>,
    muted: Option<Color>,
    dark_muted: Option<Color>,
    light_muted: Option<Color>,
}

fn get_texture_dynamic(
    track: Dynamic<Option<PlayingTrack>>,
) -> (Dynamic<AnyTexture>, Dynamic<ImageVibrancy>) {
    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: HttpCacheOptions::default(),
        }))
        .build();

    let texture = Dynamic::new(get_empty_texture());
    let vibrancy = Dynamic::new(ImageVibrancy::default());

    let prev_request_join = Arc::new(Mutex::new(None::<JoinHandle<()>>));
    track
        .for_each({
            let texture = texture.clone();
            let vibrancy = vibrancy.clone();
            move |track| {
                if let Some(track) = track {
                    let mut prev_request_join = prev_request_join.lock().unwrap();
                    if let Some(prev_request_join) = prev_request_join.take() {
                        prev_request_join.abort();
                    }
                    let texture = texture.clone();
                    let vibrancy = vibrancy.clone();
                    let client = client.clone();
                    let track_url = track.album_art.clone().unwrap();
                    *prev_request_join = Some(tokio_runtime().spawn(async move {
                        let response = client.get(track_url).send().await.unwrap();
                        let bytes = response.bytes().await.unwrap();
                        let image = image::load_from_memory(&bytes).unwrap();
                        let image = image.resize(128, 128, FilterType::Lanczos3);
                        let image_vibrancy = Vibrancy::new(&image);
                        vibrancy.set(ImageVibrancy {
                            primary: image_vibrancy.primary.map(|c| rgb_to_color(c)),
                            dark: image_vibrancy.dark.map(|c| rgb_to_color(c)),
                            light: image_vibrancy.light.map(|c| rgb_to_color(c)),
                            muted: image_vibrancy.muted.map(|c| rgb_to_color(c)),
                            dark_muted: image_vibrancy.dark_muted.map(|c| rgb_to_color(c)),
                            light_muted: image_vibrancy.light_muted.map(|c| rgb_to_color(c)),
                        });
                        let image_texture = LazyTexture::from_image(
                            image,
                            cushy::kludgine::wgpu::FilterMode::Linear,
                        );
                        let image_texture = AnyTexture::Lazy(image_texture);
                        texture.set(image_texture);
                    }));
                } else {
                    vibrancy.set(ImageVibrancy::default());
                    texture.set(get_empty_texture());
                }
            }
        })
        .persist();

    (texture, vibrancy)
}

fn rgb_to_color(rgb: Rgb<u8>) -> Color {
    Color::new(rgb[0], rgb[1], rgb[2], 255)
}

/// This spawns a new thread to track the current playing track and its progress.
/// The two objects are separate as track info is updated less frequently than progress.
fn get_track_dynamics() -> (Dynamic<Duration>, Dynamic<Option<PlayingTrack>>) {
    let track = Dynamic::new(None::<PlayingTrack>);
    let progress = Dynamic::new(Duration::from_secs(0));
    thread::spawn({
        let track = track.clone();
        let progress = progress.clone();
        move || {
            let player_finder = PlayerFinder::new();
            let player_finder = match player_finder {
                Ok(finder) => finder,
                Err(e) => {
                    eprintln!("Failed to find player: {:?}. Dbus/libdbus may not be installed? Track data will be unavailable", e);
                    return;
                }
            };
            const SLEEP_TIME: Duration = Duration::from_millis(200);

            loop {
                let player = player_finder.find_active();
                let player = match player {
                    Ok(player) => player,
                    Err(_) => {
                        track.set(None);
                        thread::sleep(SLEEP_TIME);
                        continue;
                    }
                };
                let tracker = player.track_progress(200);
                let mut tracker = match tracker {
                    Ok(tracker) => tracker,
                    Err(_) => {
                        track.set(None);
                        thread::sleep(SLEEP_TIME);
                        continue;
                    }
                };
                let mut first_run = true;
                loop {
                    let tick = tracker.tick();
                    if tick.player_quit {
                        track.set(None);
                        thread::sleep(SLEEP_TIME);
                        break;
                    }
                    if tick.progress_changed || first_run {
                        first_run = false;
                        let p = tick.progress;
                        let meta = p.metadata();
                        track.set(Some(PlayingTrack {
                            title: meta.title().unwrap_or_default().to_string(),
                            artist: meta.artists().unwrap_or_default().join(", "),
                            album: meta.album_name().unwrap_or_default().to_string(),
                            duration: meta.length().unwrap_or_default(),

                            album_art: meta.art_url().map(|url| url.to_string()),

                            status: p.playback_status(),
                            shuffle: p.shuffle(),
                            loop_status: p.loop_status(),
                        }));
                    }
                    progress.set(tick.progress.position());
                }
            }
        }
    });

    (progress, track)
}
