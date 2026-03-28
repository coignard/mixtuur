// This file is part of mixtuur.
//
// Copyright (c) 2026  René Coignard <contact@renecoignard.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// How the user spelled an enharmonic note.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    /// No accidental: C, D, E, F, G, A, B.
    Natural,
    /// Raised by a semitone: C#, D#, F#, G#, A#.
    Sharp,
    /// Lowered by a semitone: Db, Eb, Gb, Ab, Bb.
    Flat,
}

/// A musical pitch with its semitone position, accidental spelling, and
/// diatonic letter.
///
/// The `letter` field is an index into the natural note sequence
/// C=0, D=1, E=2, F=3, G=4, A=5, B=6.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pitch {
    /// Absolute semitone (0 = C ... 11 = B).
    pub semitone: u8,

    /// Accidental spelling as supplied by the user.
    pub accidental: Accidental,

    /// Diatonic letter index: 0 = C, 1 = D, 2 = E, 3 = F, 4 = G, 5 = A, 6 = B.
    pub letter: u8,
}

impl Pitch {
    /// Parses a note name into a [`Pitch`], returning `None` for unknown input.
    ///
    /// Recognised names (case-insensitive): C, C#, Db, D, D#, Eb, E, F, F#,
    /// Gb, G, G#, Ab, A, A#, Bb, B.
    pub fn parse(s: &str) -> Option<Self> {
        let (semitone, accidental, letter) = match s.trim().to_ascii_lowercase().as_str() {
            "c" => (0, Accidental::Natural, 0),
            "c#" => (1, Accidental::Sharp, 0),
            "db" => (1, Accidental::Flat, 1),
            "d" => (2, Accidental::Natural, 1),
            "d#" => (3, Accidental::Sharp, 1),
            "eb" => (3, Accidental::Flat, 2),
            "e" => (4, Accidental::Natural, 2),
            "f" => (5, Accidental::Natural, 3),
            "f#" => (6, Accidental::Sharp, 3),
            "gb" => (6, Accidental::Flat, 4),
            "g" => (7, Accidental::Natural, 4),
            "g#" => (8, Accidental::Sharp, 4),
            "ab" => (8, Accidental::Flat, 5),
            "a" => (9, Accidental::Natural, 5),
            "a#" => (10, Accidental::Sharp, 5),
            "bb" => (10, Accidental::Flat, 6),
            "b" => (11, Accidental::Natural, 6),
            _ => return None,
        };
        Some(Pitch {
            semitone,
            accidental,
            letter,
        })
    }

    /// Returns `true` when this tonic + mode combination should use flat
    /// spelling for out-of-scale (chromatic) notes.
    ///
    /// Rules (in priority order):
    /// 1. Flat-spelled input (Db, Eb ...) → always flats.
    /// 2. Sharp-spelled input (C#, D# ...) → always sharps.
    /// 3. Natural note → Circle-of-Fifths algorithm: CoF position of the
    ///    tonic, shifted by the mode's offset, ≥ 7 → flats.
    pub fn uses_flats(self, mode_offset: i8) -> bool {
        match self.accidental {
            Accidental::Flat => true,
            Accidental::Sharp => false,
            Accidental::Natural => {
                let cof = (i32::from(self.semitone) * 7).rem_euclid(12) as i8;
                (cof + mode_offset).rem_euclid(12) >= 7
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_parse_semitones_and_letters() {
        let cases: &[(&str, u8, u8, Accidental)] = &[
            ("C", 0, 0, Accidental::Natural),
            ("C#", 1, 0, Accidental::Sharp),
            ("Db", 1, 1, Accidental::Flat),
            ("D", 2, 1, Accidental::Natural),
            ("Eb", 3, 2, Accidental::Flat),
            ("F#", 6, 3, Accidental::Sharp),
            ("Gb", 6, 4, Accidental::Flat),
            ("G#", 8, 4, Accidental::Sharp),
            ("Ab", 8, 5, Accidental::Flat),
            ("Bb", 10, 6, Accidental::Flat),
            ("B", 11, 6, Accidental::Natural),
        ];
        for &(s, semi, letter, acc) in cases {
            let p = Pitch::parse(s).unwrap_or_else(|| panic!("failed to parse '{s}'"));
            assert_eq!(p.semitone, semi, "{s}: semitone");
            assert_eq!(p.letter, letter, "{s}: letter");
            assert_eq!(p.accidental, acc, "{s}: accidental");
        }
        assert!(Pitch::parse("H").is_none());
        assert!(Pitch::parse("").is_none());
    }

    #[test]
    fn uses_flats_major() {
        let maj = 0i8;
        // Flat major keys: F Bb Eb Ab Db Gb Cb
        for note in ["F", "Bb", "Eb", "Ab", "Db"] {
            assert!(
                Pitch::parse(note).unwrap().uses_flats(maj),
                "{note} major should use flats"
            );
        }
        // Sharp major keys: G D A E B F#
        for note in ["G", "D", "A", "E", "B"] {
            assert!(
                !Pitch::parse(note).unwrap().uses_flats(maj),
                "{note} major should use sharps"
            );
        }
        // C major: no accidentals, sharp convention
        assert!(!Pitch::parse("C").unwrap().uses_flats(maj));
    }

    #[test]
    fn uses_flats_minor() {
        let min = -3i8;
        for note in ["D", "G", "C", "F", "Bb", "Eb"] {
            assert!(
                Pitch::parse(note).unwrap().uses_flats(min),
                "{note} minor should use flats"
            );
        }
        for note in ["A", "E", "B"] {
            assert!(
                !Pitch::parse(note).unwrap().uses_flats(min),
                "{note} minor should use sharps"
            );
        }
        // Sharp-spelled input always sharps regardless of mode
        assert!(!Pitch::parse("F#").unwrap().uses_flats(min));
        assert!(!Pitch::parse("G#").unwrap().uses_flats(min));
        // Flat-spelled input always flats
        assert!(Pitch::parse("Bb").unwrap().uses_flats(min));
    }
}
