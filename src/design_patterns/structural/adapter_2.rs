use std::fmt::{Display, Formatter};

enum AudioType {
    MP3,
    MP4,
    VLC,
    AVI,
}

impl AudioType {
    fn as_str(&self) -> &'static str {
        match *self {
            AudioType::MP3 => "MP3",
            AudioType::MP4 => "MP4",
            AudioType::VLC => "VLC",
            AudioType::AVI => "AVI",
        }
    }
}

impl Display for AudioType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

trait MediaPlayer {
    fn play(&self, r#type: AudioType, file_name: String);
}

trait AdvancedMediaPlayer {
    fn play_vlc(&self, file_name: String);
    fn play_mp4(&self, file_name: String);
}

struct VlcPlayer;

impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: String) {
        println!("Playing vlc file. Name: {}", file_name);
    }

    fn play_mp4(&self, _file_name: String) {
        unimplemented!()
    }
}

struct Mp4Player;

impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, _file_name: String) {
        unimplemented!()
    }

    fn play_mp4(&self, file_name: String) {
        println!("Playing mp4 file. Name: {}", file_name);
    }
}

struct MediaAdapter {
    advanced_music_player: Box<dyn AdvancedMediaPlayer>
}

impl MediaAdapter {
    fn new(r#type: AudioType) -> Option<MediaAdapter> {
        match r#type {
            AudioType::VLC => Some(MediaAdapter { advanced_music_player: Box::new(VlcPlayer) }),
            AudioType::MP4 => Some(MediaAdapter { advanced_music_player: Box::new(Mp4Player) }),
            _ => None
        }
    }
}

impl MediaPlayer for MediaAdapter {
    fn play(&self, r#type: AudioType, file_name: String) {
        match r#type {
            AudioType::VLC => self.advanced_music_player.play_vlc(file_name),
            AudioType::MP4 => self.advanced_music_player.play_mp4(file_name),
            _ => ()
        };
    }
}

struct AudioPlayer;

impl MediaPlayer for AudioPlayer {
    fn play(&self, r#type: AudioType, file_name: String) {
        match r#type {
            AudioType::MP3 => println!("Playing mp3 file. Name: {}", file_name),
            AudioType::MP4 => MediaAdapter::new(AudioType::MP4).unwrap().play(AudioType::MP4, file_name),
            AudioType::VLC => MediaAdapter::new(AudioType::VLC).unwrap().play(AudioType::VLC, file_name),
            x @ _ => println!("Invalid media. {} format not supported", x)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter() {
        let audio_player = AudioPlayer;
        audio_player.play(AudioType::MP3, String::from("beyond the horizon.mp3"));
        audio_player.play(AudioType::MP4, String::from("alone.mp4"));
        audio_player.play(AudioType::VLC, String::from("far far away.vlc"));
        audio_player.play(AudioType::AVI, String::from("mind me.avi"));
    }
}


