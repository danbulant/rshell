use std::{sync::{Arc, Mutex}, thread, time::Duration};

use mpris::{LoopStatus, PlaybackStatus, PlayerFinder};
use cushy::{kludgine::{AnyTexture, LazyTexture}, styles::components::WidgetBackground, value::{Destination, Dynamic, IntoReader, Source}, widget::MakeWidget, widgets::Image, Open, PendingApp};
use reqwest::Client;
use image;
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

pub fn spotify_controls() -> impl MakeWidget {
    let (progress, track) = get_track_dynamics();
    let texture = get_texture_dynamic(track.clone());

    track.map_each(|track| {
        if let Some(track) = track {
            format!(
                "{} - {}",
                track.artist,
                track.title,
            )
        } else {
            "No track playing".to_string()
        }
    })
        .to_label()
        .and(
            Image::new(texture)
        )
        .into_rows()
}

fn get_empty_texture() -> AnyTexture {
    AnyTexture::Lazy(
        LazyTexture::from_image(
            image::DynamicImage::ImageRgba8(
                image::ImageBuffer::new(1, 1)
            ),
            cushy::kludgine::wgpu::FilterMode::Linear
        )
    )
}

fn tokio_runtime() -> &'static runtime::Handle {
    use std::sync::OnceLock;
    use std::time::Duration;

    use cushy::value::{Destination, Dynamic};
    use cushy::widget::MakeWidget;
    use cushy::widgets::progress::Progressable;
    use cushy::Run;
    use tokio::time::sleep;

    static RUNTIME: OnceLock<runtime::Handle> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio initialization error");
        let handle = rt.handle().clone();
        std::thread::spawn(move || {
            // Replace with the async main loop, or some sync structure to
            // control shutting it down if desired.
            rt.block_on(async {
                loop {
                    tokio::time::sleep(Duration::from_secs(10000)).await
                }
            });
        });
        handle
    })
}

fn get_texture_dynamic(track: Dynamic<Option<PlayingTrack>>) -> Dynamic<AnyTexture> {
    let client = Client::new();


    let texture = Dynamic::new(get_empty_texture());

    let prev_request_join = Arc::new(Mutex::new(None::<JoinHandle<()>>));
    track.for_each({
        let texture = texture.clone();
        move |track| {
            if let Some(track) = track {
                let mut prev_request_join = prev_request_join.lock().unwrap();
                if let Some(prev_request_join) = prev_request_join.take() {
                    prev_request_join.abort();
                }
                let texture = texture.clone();
                let client = client.clone();
                let track_url = track.album_art.clone().unwrap();
                *prev_request_join = Some(tokio_runtime().spawn(async move {
                    let response = client.get(track_url).send().await.unwrap();
                    let bytes = response.bytes().await.unwrap();
                    let image = image::load_from_memory(&bytes).unwrap();
                    let image_texture = LazyTexture::from_image(image, cushy::kludgine::wgpu::FilterMode::Linear);
                    let image_texture = AnyTexture::Lazy(image_texture);
                    texture.map_mut(move |mut t| *t = image_texture);
                    // texture.set(image_texture);
                }));
            } else {
                texture.map_mut(move |mut t| *t = get_empty_texture());
                // texture.set(empty_texture);
            }
        }
    }).persist();

    texture
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