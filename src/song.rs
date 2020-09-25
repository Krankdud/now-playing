use std::fs::File;
use std::io;
use std::io::Read;

pub struct SongReader {
    filename: String,
    song: Song,
}

#[derive(Clone)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub position: String,
    pub duration: String,
}

impl Song {
    fn new() -> Self {
        Song {
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            position: String::new(),
            duration: String::new(),
        }
    }

    fn from_tsv(string: &String) -> Option<Self> {
        let mut song = Song::new();
        let splits: Vec<&str> = string.split('\t').collect();
        if splits.len() < 5 {
            return None;
        }
        song.title.push_str(splits[0]);
        song.artist.push_str(splits[1]);
        song.album.push_str(splits[2]);
        song.position.push_str(splits[3]);
        song.duration.push_str(splits[4]);
        return Some(song);
    }
}

impl SongReader {
    pub fn from(filename: &str) -> Self {
        SongReader {
            filename: String::from(filename),
            song: Song::new(),
        }
    }

    pub fn update(&mut self) -> Result<Song, io::Error> {
        let mut f = File::open(&self.filename)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let song = match Song::from_tsv(&s) {
            None => return Ok(self.song.clone()),
            Some(song) => song,
        };

        self.song = song.clone();

        Ok(song)
    }

    pub fn get_song(&self) -> Song {
        return self.song.clone();
    }
}
