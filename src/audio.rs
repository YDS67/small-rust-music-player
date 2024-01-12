use std::sync::{Arc, Mutex};
use std::fs;
use rodio::Source;

use crate::settings;

pub fn playback(state_player: Arc<Mutex<crate::State>>) {
    let current_dir = std::env::current_dir().expect("Can't find current directory");
    let (_stream, handle) = rodio::OutputStream::try_default().expect("Can't open output stream (Rodio)");
    let sink = rodio::Sink::try_new(&handle).expect("Can't create Rodio Sink");

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
                    let res = match ext {
                        MusicFormat::MP3 => rodio::Decoder::new_mp3(file),
                        MusicFormat::WAV => rodio::Decoder::new_wav(file),
                        MusicFormat::OGG => rodio::Decoder::new_vorbis(file),
                        MusicFormat::FLAC => rodio::Decoder::new_flac(file),
                        _ => rodio::Decoder::new(file),
                    };

                    match res {
                        Ok(buff) => {
                            counter += 1;
                            let buffc = buff.buffered();
                            sink.append(buffc);
        
                            while !sink.empty() {
                                let mut s_player = state_player.lock().unwrap();
                                s_player.file_num = counter;
                                let file_name = track_name(&pstr);
                                s_player.file_name = file_name;
                                s_player.file_ext = ext_text.clone();
                
                                if s_player.play {
                                    sink.play();
                                    if s_player.skip {
                                        s_player.skip = false;
                                        sink.skip_one();
                                        break;
                                    }
                                } else {
                                    sink.pause();
                                }
                                drop(s_player);
                
                                std::thread::sleep(std::time::Duration::from_secs_f64(settings::FT_DESIRED));
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
    for i in 0..(letters.len()).min(40) {
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

enum MusicFormat {
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