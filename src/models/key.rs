pub struct SongKey {
    chromatic_index: i32,
    root: Note
}

pub enum Note {
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

impl SongKey {
    fn from_index(index: i32) -> SongKey {
        let root = match index {
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

            _ => panic!("Could not determine note for index {}", index)
        };

        return SongKey { chromatic_index: index, root: root }
    }

    fn from_root(root: Note) -> SongKey {
        let index = match root {
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
            Note::Ab => 11,

            _ => panic!("Could not determine note for root")
        };

        return SongKey { chromatic_index: index, root: root }
    }
}
