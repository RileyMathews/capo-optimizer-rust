use std::cmp::Ordering;

use strum_macros::Display;

fn main() {
    let song = Song { keys: vec![Key::from_index(2)] };
    let translation_options = optimize(&song);
    print!("your song was ");
    song.print();
    println!("");
    for song in translation_options {
        song.print();
        println!("");
    }
}

fn order_by_caged_keys(a: &Song, b: &Song) -> Ordering {
    let a_count = a.count_caged_keys();
    let b_count = b.count_caged_keys();
    if a_count == b_count {
        return Ordering::Equal
    } else if a_count > b_count {
        return Ordering::Greater
    } else {
        return Ordering::Less
    }
}

fn optimize(song: &Song) -> Vec<Song> {
    let mut transpositions_vec: Vec<Song> = (0..5).map(|i| song.translate_up(i))
                                                  .filter(|song| song.has_any_caged_keys())
                                                  .collect();
    transpositions_vec.sort_by(order_by_caged_keys);
    return transpositions_vec
}

struct Song {
    keys: Vec<Key>
}

impl Song {
    fn translate_up(&self, steps: i32) -> Song {
        let new_keys = self.keys.iter().map(|key| key.translate_up(steps)).collect();
        return Song { keys: new_keys }
    }

    fn count_caged_keys(&self) -> i32 {
        let mut acc = 0;
        for key in self.keys.iter() {
            if key.is_caged_key() {
                acc += 1
            }
        }
        return acc
    }

    fn has_any_caged_keys(&self) -> bool {
        return self.keys.iter().any(|key| key.is_caged_key())
    }

    fn print(&self) {
        for key in self.keys.iter() {
            let root = &key.root;
            print!(" {root}");
        }
    }
}

#[derive(PartialEq)]
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

    fn is_caged_key(&self) -> bool {
        let caged_keys = vec![Note::C, Note::A, Note::G, Note::E, Note::D];
        return caged_keys.iter().any(|i| i == &self.root)
    }
}
