use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::fs;
use rodio::Source; 

use crate::settings;

pub fn playback(state_player: Arc<Mutex<crate::State>>) {
    let current_dir = std::env::current_dir().expect("Can't find current directory");

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    loop {
        let mut counter = 0;
        for entry in fs::read_dir(current_dir.clone()).unwrap() {
            let path = entry.unwrap().path();
            let pstr = format!("{}", path.display());
            let file = std::fs::File::open(path).unwrap();
            let res = rodio::Decoder::new(BufReader::new(file));
            let sink = rodio::Sink::try_new(&handle).unwrap();

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
    }
    
}

fn track_name(text: &String) -> String {
    let letters: Vec<char> = text.chars().collect();
    let mut new: String = String::new();
    for i in 0..(letters.len()-2).min(30) {
        new.push(letters[i+2]);
    }
    new
}