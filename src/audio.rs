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
            let dir_name = track_name(&pstr);
            s_player.dir_name = dir_name;
            drop(s_player);
            let pstr = format!("{}", path.display());
            let file = std::fs::File::open(path).expect("Can't open file");
            let res = rodio::Decoder::new(file);

            match res {
                Ok(buff) => {
                    counter += 1;
                    let buffc = buff.buffered();
                    sink.append(buffc);
                    if sink.empty() {
                        println!("Couldn't append track to sink")
                    }
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
                Err(_) => {
                    //println!("Can't decode file {}", track_name(&pstr))
                }
            }
        }
    }
    
}

fn track_name(text: &String) -> String {
    let text1: Vec<&str> = text.split(|c| c == '/' || c == '\\').collect();
    let letters: Vec<char> = text1[text1.len()-1].chars().collect();
    let mut new: String = String::new();
    for i in 0..(letters.len()).min(30) {
        new.push(letters[i]);
    }
    new
}