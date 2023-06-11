use std::panic;

use mpris::{PlaybackStatus, PlayerFinder};

fn main() {
    panic::set_hook(Box::new(|_| {}));

    let finder = PlayerFinder::new().unwrap();

    if let Ok(player) = finder.find_by_name("Spotify") {
        let metadata = player.get_metadata().unwrap();

        let status = match player.get_playback_status().unwrap() {
            PlaybackStatus::Playing => "▶",
            PlaybackStatus::Paused => "▮▮",
            PlaybackStatus::Stopped => "◼",
        };

        if let (Some(artists), Some(title)) = (metadata.artists(), metadata.title()) {
            let artists_str = artists.join(", ");
            println!("{} {} - {}", status, artists_str, title);
        }
    } else {
        println!("No music playing");
    }
}
