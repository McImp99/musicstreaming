use std::fs;
use std::path::Path;
use rodio::{Decoder, Sink, stream::OutputStreamBuilder};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub enum MusicError {
    FolderNotFound,
    ReadError,
    NoMusicFiles,
    PlaybackError,
}


pub fn list_songs(folder: &str) -> Result<Vec<String>, MusicError> {
    let path = Path::new(folder);

    if !path.exists() {
        return Err(MusicError::FolderNotFound);
    }
    let entries = fs::read_dir(path).map_err(|_| MusicError::ReadError)?;

    let mut songs = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|_| MusicError::ReadError)?;
        let file_path = entry.path();


        if let Some(ext) = file_path.extension() {
            if ext.to_string_lossy().to_lowercase() == "mp3" {
                if let Some(name) = file_path.file_name() {
                    songs.push(name.to_string_lossy().to_string());
                }
            }
        }
    }

    if songs.is_empty() {
    return Err(MusicError::NoMusicFiles);
    }

    Ok(songs)
}

pub fn play_song(path: PathBuf) -> Result<Arc<Mutex<Sink>>, MusicError> {

    let stream = OutputStreamBuilder::open_default_stream().map_err(|_| MusicError::PlaybackError)?;
    

    let handle = stream.mixer();
    let sink = Sink::connect_new(&handle);

    let file = File::open(path).map_err(|_| MusicError::PlaybackError)?;
    let source = Decoder::new(BufReader::new(file)).map_err(|_| MusicError::PlaybackError)?;

    sink.append(source);


    let sink_arc = Arc::new(Mutex::new(sink));
    Ok(sink_arc)
}

pub fn pause_song(sink: &Arc<Mutex<Sink>>) {
    let s = sink.lock().unwrap();
    s.pause();
}

pub fn resume_song(sink: &Arc<Mutex<Sink>>) {
    let s = sink.lock().unwrap();
    s.play();
}

pub fn stop_song(sink: &Arc<Mutex<Sink>>) {
    let s = sink.lock().unwrap();
    s.stop();
}

pub fn find_song(folder: &str, song_name: &str) -> Option<PathBuf> {
    let path = Path::new(folder).join(song_name);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}
