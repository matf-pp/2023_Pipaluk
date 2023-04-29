use std::collections::HashMap;
use sdl2::mixer::*;

pub struct Mixer {
    _mixer_context: Sdl2MixerContext,
    pub chunks: HashMap <String, Chunk>,
    music_channels: HashMap <i32, Channel>,
    current_track: Option<String>,
    current_music_channel: i32,
    other_channels: HashMap <String, Channel>
}

impl Mixer {
    pub fn init() -> Self {

        let frequency = 44_100;
        let format = sdl2::sys::AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        open_audio(frequency, format as u16, channels, chunk_size).unwrap();
        allocate_channels(4);

        let _mixer_context = sdl2::mixer::init(InitFlag::MP3).unwrap();

        let mut chunks: HashMap<String, Chunk> = HashMap::new();
        chunks.insert("slow".to_string(), Chunk::from_file("resources/audio/slowversion-01.mp3").unwrap());
        let mut fast_chunk = Chunk::from_file("resources/audio/fastversion-01.mp3").unwrap();
        fast_chunk.set_volume(24);
        chunks.insert("fast".to_string(), fast_chunk);
        chunks.insert("win".to_string(), Chunk::from_file("resources/audio/win-01.mp3").unwrap());

        let mut music_channels: HashMap <i32, Channel> = HashMap::new();
        music_channels.insert(1, Channel::all().to_owned());
        music_channels.insert(2, Channel::all().to_owned());
        let current_track = None;
        let current_music_channel: i32 = 2;

        let mut other_channels: HashMap <String, Channel> = HashMap::new();
        other_channels.insert("effects".to_string(), Channel::all().to_owned());

        Self { _mixer_context, chunks, music_channels, current_track, current_music_channel, other_channels }
    }

    pub fn play_song(&mut self, name: &str) {
        if self.current_track.is_some() && name.to_string().eq(self.current_track.as_mut().unwrap()) {
            return;
        }
        self.current_track = Some(name.to_string());
        if self.music_channels.get(&self.current_music_channel).unwrap().is_playing() {
            println!("Stopping music on channel {}", self.current_music_channel);
            self.music_channels.get(&self.current_music_channel).unwrap().fade_out(3000);
        }
        self.current_music_channel = 3 - self.current_music_channel;
        println!("Playing {} on channel {}", name, self.current_music_channel);
        self.music_channels.get(&self.current_music_channel).unwrap().fade_in(
            self.chunks.get(&name.to_string()).unwrap(), 
            -1, 
            1500
        ).unwrap();
    }

    pub fn stop_music(&mut self) {
        if self.music_channels.get(&self.current_music_channel).unwrap().is_playing() {
            println!("Stopping music on channel {}", self.current_music_channel);
            self.music_channels.get(&self.current_music_channel).unwrap().fade_out(3000);
            self.current_track = None;
        }
    }

    pub fn play_effect(&mut self, name: &str) {
        self.other_channels.get("effects").unwrap().fade_in(
            self.chunks.get(&name.to_string()).unwrap(), 
            0,
            150
        ).unwrap();
    }
}
