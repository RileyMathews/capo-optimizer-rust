use strum_macros::Display;

fn main() {
    let song = Song { keys: vec![Key::from_index(0)] };
    let new_song = song.translate_up(2);
    let new_song_root = &new_song.keys[0].root;
    println!("{new_song_root}")
}

struct Song {
    keys: Vec<Key>
}

impl Song {
    fn translate_up(&self, steps: i32) -> Song {
        let new_keys = self.keys.iter().map(|key| key.translate_up(steps)).collect();
        return Song { keys: new_keys }
    }
}

#[derive(Display)]
enum Note {
    A,
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab
}

struct Key {
    chromatic_index: i32,
    root: Note
}

impl Key {
    fn from_root(root: Note) -> Key {
        let chromatic_index = match root {
            Note::A => 0,
            Note::Bb => 1,
            Note::B => 2,
            Note::C => 3,
            Note::Db => 4,
            Note::D => 5,
            Note::Eb => 6,
            Note::E => 7,
            Note::F => 8,
            Note::Gb => 9,
            Note::G => 10,
            Note::Ab => 11
        };
        return Key { chromatic_index, root }
    }

    fn from_index(chromatic_index: i32) -> Key {
        let root = match chromatic_index {
            0 => Note::A,
            1 => Note::Bb,
            2 => Note::B,
            3 => Note::C,
            4 => Note::Db,
            5 => Note::D,
            6 => Note::Eb,
            7 => Note::E,
            8 => Note::F,
            9 => Note::Gb,
            10 => Note::G,
            11 => Note::Ab,
            _ => panic!("could not find note for given index {chromatic_index}")
        };
        return Key { chromatic_index, root }
    }

    fn translate_up(&self, steps: i32) -> Key {
        let mut new_index = self.chromatic_index + steps;
        if new_index >= 12 {
            new_index -= 12
        }

        return Key::from_index(new_index)
    }
}
