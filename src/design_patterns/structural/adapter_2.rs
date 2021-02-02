//!
//! 适配器模式（Adapter Pattern）是作为两个不兼容的接口之间的桥梁。
//! 这种类型的设计模式属于结构型模式，它结合了两个独立接口的功能。
//! 这种模式涉及到一个单一的类，该类负责加入独立的或不兼容的接口功能。
//!
//! 我们有一个 MediaPlayer 接口和一个实现了 MediaPlayer 接口的实体类 AudioPlayer。
//! 默认情况下，AudioPlayer 可以播放 mp3 格式的音频文件。
//!
//! 我们还有另一个接口 AdvancedMediaPlayer 和实现了 AdvancedMediaPlayer 接口的实体类。
//! 该类可以播放 vlc 和 mp4 格式的文件。
//!
//! 我们想要让 AudioPlayer 播放其他格式的音频文件。
//! 为了实现这个功能，我们需要创建一个实现了 MediaPlayer 接口的适配器类 MediaAdapter，并使用 AdvancedMediaPlayer 对象来播放所需的格式。
//!
//! AudioPlayer 使用适配器类 MediaAdapter 传递所需的音频类型，不需要知道能播放所需格式音频的实际类。
//! AdapterPatternDemo 类使用 AudioPlayer 类来播放各种格式。
//!


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


