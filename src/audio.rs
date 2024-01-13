use std::sync::{Arc, Mutex};
use std::fs;
use std::io::{Read, Seek};
use std::marker::Sync;
use std::time::Duration;
use rodio::decoder::{Decoder, DecoderError};
use rodio::source::Source;
use std::sync::mpsc::{self, Sender, Receiver};

use crate::settings;

const RATE: f64 = 1.0/30.0;

pub fn playback(state_player: Arc<Mutex<crate::State>>) {
    let current_dir = std::env::current_dir().expect("Can't find current directory");
    let (_stream, handle) = rodio::OutputStream::try_default().expect("Can't open output stream (Rodio)");
    let sink = rodio::Sink::try_new(&handle).expect("Can't create Rodio Sink");
    let (tx, rx): (Sender<[i16; settings::SAMPLES]>, Receiver<[i16; settings::SAMPLES]>) = mpsc::channel();

    loop {
        let mut counter = 0;
        let entries = fs::read_dir(current_dir.clone()).expect("ReadDir error");
        for entry in entries {
            let path = entry.expect("Reading entry path error").path();
            let pstr = format!("{}", current_dir.display());
            let mut s_player = state_player.lock().unwrap();
            let dir_name = dir_name(&pstr);
            s_player.dir_name = dir_name;
            drop(s_player);
            let pstr = format!("{}", path.display());
            let ext = track_format(&pstr);
            let ext_text = ext.display();
            let file_open = std::fs::File::open(path);
            match file_open {
                Ok(file) => {
                    let res = SpyDecoder::new(file, ext);

                    match res {
                        Ok(buff) => {
                            counter += 1;
                            let tx2 = tx.clone();
                            let buffc = buff.periodic_access(
                                std::time::Duration::from_secs_f64(RATE), 
                                move |s| {
                                    tx2.send(s.stats).unwrap()
                                });
                            sink.append(buffc);
                            while !sink.empty() {
                                let mut s_player = state_player.lock().unwrap();
                                s_player.file_num = counter;
                                let file_name = track_name(&pstr);
                                s_player.file_name = file_name;
                                s_player.file_ext = ext_text.clone();
                
                                if s_player.play {
                                    sink.play();
                                    let send_sample = rx.try_recv();
                                    match send_sample {
                                        Ok(stats) => s_player.sample_stats = stats,
                                        Err(_) => {}
                                    }
                                    if s_player.skip {
                                        s_player.skip = false;
                                        s_player.sample_stats = [0; settings::SAMPLES];
                                        sink.skip_one();
                                        break;
                                    }
                                } else {
                                    sink.pause();
                                }
                                drop(s_player);
                
                                std::thread::sleep(std::time::Duration::from_secs_f64(RATE));
                            }
                        },
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
    }
    
}

fn track_name(text: &String) -> String {
    let text1: Vec<&str> = text.split(|c| c == '/' || c == '\\').collect();
    let letters: Vec<char> = text1[text1.len()-1].chars().collect();
    let mut new: String = String::new();
    for i in 0..letters.len() {
        new.push(letters[i]);
    }
    let split: Vec<&str> = new.split(|c| c == '.').collect();
    let mut res = String::new();
    for i in 0..(split.len()-1) {
        let chars: Vec<char> = split[i].chars().collect();
        for j in 0..chars.len() {
            res.push(chars[j])
        }
    }
    res
}

fn dir_name(text: &String) -> String {
    let text1: Vec<&str> = text.split(|c| c == '/' || c == '\\').collect();
    let letters: Vec<char> = text1[text1.len()-1].chars().collect();
    let mut new: String = String::new();
    for i in 0..(letters.len()).min(40) {
        new.push(letters[i]);
    }
    new
}

fn track_format(text: &String) -> MusicFormat {
    let split: Vec<&str> = text.split(|c| c == '.').collect();
    let ext = split[split.len()-1];
    let answ: MusicFormat = match ext {
        "mp3" => MusicFormat::MP3,
        "wav" => MusicFormat::WAV,
        "ogg" => MusicFormat::OGG,
        "flac" => MusicFormat::FLAC,
        "MP3" => MusicFormat::MP3,
        "WAV" => MusicFormat::WAV,
        "OGG" => MusicFormat::OGG,
        "FLAC" => MusicFormat::FLAC,
        _ => MusicFormat::GEN,
    };
    answ
}

pub enum MusicFormat {
    MP3,
    WAV,
    OGG,
    FLAC,
    GEN,
}

impl MusicFormat {
    fn display(&self) -> String {
        match self {
            Self::MP3 => "mp3".to_string(),
            Self::WAV => "wav".to_string(),
            Self::OGG => "ogg".to_string(),
            Self::FLAC => "flac".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

pub struct SpyDecoder<R> where R: Read + Seek
{
    inner: Decoder<R>,
    stats: [i16; settings::SAMPLES],
    stats_index: usize,
    stats_wait: usize,
    stats_wait_index: usize,
    stats_collect: bool,
}

impl<R> SpyDecoder<R>
    where
        R: Read + Seek + Send + Sync + 'static,
{
    pub fn new(file: R, ext: MusicFormat) -> Result<SpyDecoder<R>, DecoderError> {
        let inner = match ext {
            MusicFormat::MP3 => Decoder::new_mp3(file),
            MusicFormat::WAV => Decoder::new_wav(file),
            MusicFormat::OGG => Decoder::new_vorbis(file),
            MusicFormat::FLAC => Decoder::new_flac(file),
            _ => Decoder::new(file),
        }?;
        Ok(Self {
            inner,
            stats: [0; settings::SAMPLES],
            stats_index: 0,
            stats_wait: 44000 / 10,
            stats_wait_index: 0,
            stats_collect: false,
        })
    }
}

impl<R> Iterator for SpyDecoder<R>
    where R: Read + Seek {
    type Item = i16;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next();
        if self.stats_collect {
            if self.stats_index < settings::SAMPLES {
                if sample.is_some() {
                    self.stats[self.stats_index] = sample.unwrap();
                    self.stats_index += 1;
                }
            } else {
                self.stats_collect = false;
                self.stats_index = 0;
                // Use the stats (try_send to a channel for stats computation, etc)
            }
        } else {
            if self.stats_wait_index < self.stats_wait {
                self.stats_wait_index += 1;
            } else {
                self.stats_collect = true;
                self.stats_wait_index = 0;
            }
        }
        sample
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<R> Source for SpyDecoder<R>
    where R: Read + Seek {
    fn current_frame_len(&self) -> Option<usize> {
        self.inner.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
}