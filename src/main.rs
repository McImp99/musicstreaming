use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod lib;
use lib::{list_songs, play_song, pause_song, resume_song, stop_song, find_song, MusicError};


fn main() {
   
   let songs = match list_songs("music") {
        Ok(s) => s,
        Err(e) => {
            match e {
                MusicError::FolderNotFound => eprintln!("Folder 'music/' does not exist."),
                MusicError::ReadError => eprintln!("Could not read music folder."),
                MusicError::NoMusicFiles => eprintln!("No music files in music folder."),
                _ => eprintln!("Unknown error: {:?}", e),
            }
            return;
        }
    };

    println!("Availaible songs:");
    for (i, song) in songs.iter().enumerate() {
        println!("{}: {}", i + 1, song);
    }

    print!("Choose a track to play: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let choice: usize = match input.trim().parse() {
        Ok(n) if n > 0 && n <= songs.len() => n,
        _ => {
            eprintln!("Invalid choice!");
            return;
        }
    };

    let selected_song = &songs[choice - 1];
    let path: PathBuf = match find_song("music", selected_song) {
        Some(p) => p,
        None => {
            eprintln!("Could not find song");
            return;
        }
    };

    
    let sink_arc = match play_song(path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Could not play song");
            return;
        }
    };

    println!("Playing '{}'", selected_song);
    println!("Controller: P = Pause, R = Resume, Q = Stop, O = Offline");

    let mut offline = false;

    loop {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();

        match cmd.trim().to_uppercase().as_str() {
            "P" => {
                pause_song(&sink_arc);
                println!("Song paused");
            }
            "R" => {
                if !offline {
                    resume_song(&sink_arc);
                    println!("Song resumed");
                } else {
                    println!("Cant resume while offline");
                }
            }
            "Q" => {
                stop_song(&sink_arc);
                println!("Song stopped");
                break;
            }
            "O" => {
                offline = true;
                pause_song(&sink_arc);
                println!("Offline: music paused");
            }
            "C" => {
                if offline {
                    offline = false;
                    resume_song(&sink_arc);
                    println!("Online: music resumed");
                } else {
                    println!("online");
                }
            }
            _ => println!("Unknown command. Use P, R, Q, O or C."),
        }

        
        {
            let sink = sink_arc.lock().unwrap();
            if sink.empty() {
                println!("Song is done.");
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }




   

}
