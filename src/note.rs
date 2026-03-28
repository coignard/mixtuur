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

use crate::pitch::Pitch;
use crate::scale::Scale;

/// Semitone values for the natural notes C D E F G A B (letter indices 0-6).
const NATURAL_SEMITONES: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

/// Letter names in ascending order.
const LETTER_NAMES: [&str; 7] = ["C", "D", "E", "F", "G", "A", "B"];

/// Chromatic pitch names using sharp spelling.
const PITCH_NAMES_SHARP: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Chromatic pitch names using flat spelling.
const PITCH_NAMES_FLAT: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

/// Names a note by its **position within a 7-note (heptatonic) scale**.
///
/// Uses diatonic letter assignment: each successive scale degree steps one
/// letter up from the tonic.  The accidental is derived by comparing the
/// actual semitone to the natural semitone for that letter.
///
/// This is why D harmonic minor produces **both** Bb (degree 5) and C#
/// (degree 6, VII# leading tone), each degree gets its own correct letter.
pub fn diatonic_note_name(tonic: Pitch, degree_pos: usize, interval: u8) -> String {
    let letter_idx = (tonic.letter as usize + degree_pos) % 7;
    let nat_semi = NATURAL_SEMITONES[letter_idx];
    let abs_semi = (u32::from(tonic.semitone) + u32::from(interval)) % 12;
    let diff = (abs_semi as i32 - i32::from(nat_semi)).rem_euclid(12) as u8;
    let base = LETTER_NAMES[letter_idx];
    match diff {
        0 => base.to_string(),
        1 => format!("{base}#"),
        2 => format!("{base}##"),
        11 => format!("{base}b"),
        10 => format!("{base}bb"),
        _ => format!("{base}?"),
    }
}

/// Names a note from a non-heptatonic scale using an interval-to-letter-offset
/// map.
pub fn non_heptatonic_note_name(tonic: Pitch, interval: u8, scale: &Scale) -> String {
    if scale.name == "No Scale" {
        return chromatic_note_name(tonic, interval, scale.mode_fifth_offset).to_string();
    }

    // Vb vs IV#: choose letter based on key convention.
    let use_flat = tonic.uses_flats(scale.mode_fifth_offset);
    let letter_offset: usize = match interval {
        0 => 0,
        1 => 1, // IIb  second letter
        2 => 1, // II   second letter
        3 => 2, // IIIb third letter (blue note)
        4 => 2, // III  third letter
        5 => 3, // IV   fourth letter
        6 => {
            if use_flat {
                4
            } else {
                3
            }
        } // Vb or IV#
        7 => 4, // V
        8 => 5, // VIb
        9 => 5, // VI
        10 => 6, // VIIb
        11 => 6, // VII
        _ => 0,
    };

    let letter_idx = (tonic.letter as usize + letter_offset) % 7;
    let nat_semi = NATURAL_SEMITONES[letter_idx];
    let abs_semi = (u32::from(tonic.semitone) + u32::from(interval)) % 12;
    let diff = (abs_semi as i32 - i32::from(nat_semi)).rem_euclid(12) as u8;
    let base = LETTER_NAMES[letter_idx];
    match diff {
        0 => base.to_string(),
        1 => format!("{base}#"),
        11 => format!("{base}b"),
        2 => format!("{base}##"),
        10 => format!("{base}bb"),
        _ => chromatic_note_name(tonic, interval, scale.mode_fifth_offset).to_string(),
    }
}

/// Names an out-of-scale (chromatic) note using the key-signature convention.
///
/// Delegates to [`PITCH_NAMES_SHARP`] or [`PITCH_NAMES_FLAT`] according to
/// [`Pitch::uses_flats`].
pub fn chromatic_note_name(tonic: Pitch, interval: u8, mode_offset: i8) -> &'static str {
    let abs_semi = ((u32::from(tonic.semitone) + u32::from(interval)) % 12) as usize;
    if tonic.uses_flats(mode_offset) {
        PITCH_NAMES_FLAT[abs_semi]
    } else {
        PITCH_NAMES_SHARP[abs_semi]
    }
}

/// Builds the complete array of note names for all 12 chromatic positions,
/// given a tonic and a scale.
///
/// In-scale notes use diatonic (or non-heptatonic) letter assignment;
/// out-of-scale positions use the flat/sharp key-signature convention.
pub fn build_note_names(tonic: Pitch, scale: &Scale) -> [String; 12] {
    let mut names: [String; 12] = std::array::from_fn(|_| String::new());

    if scale.intervals.len() == 7 {
        for (pos, &interval) in scale.intervals.iter().enumerate() {
            let abs_semi = ((u32::from(tonic.semitone) + u32::from(interval)) % 12) as usize;
            names[abs_semi] = diatonic_note_name(tonic, pos, interval);
        }
    } else {
        for &interval in scale.intervals {
            let abs_semi = ((u32::from(tonic.semitone) + u32::from(interval)) % 12) as usize;
            names[abs_semi] = non_heptatonic_note_name(tonic, interval, scale);
        }
    }

    // Fill remaining (out-of-scale) slots with key-sig convention names.
    for i in 0u8..12 {
        if names[i as usize].is_empty() {
            let interval = (i + 12 - tonic.semitone) % 12;
            names[i as usize] =
                chromatic_note_name(tonic, interval, scale.mode_fifth_offset).to_string();
        }
    }

    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::find_scale;

    #[test]
    fn diatonic_name_d_harmonic_minor() {
        // D harmonic minor: D E F G A Bb C#
        // Key sig: 1 flat (Bb) BUT the raised VII# is C#, a sharp.
        let scale = find_scale("Harmonic Minor").unwrap();
        let tonic = Pitch::parse("D").unwrap();
        let names = build_note_names(tonic, scale);

        assert_eq!(names[2], "D", "Tonic");
        assert_eq!(names[4], "E", "Supertonic");
        assert_eq!(names[5], "F", "Mediant");
        assert_eq!(names[7], "G", "Subdominant");
        assert_eq!(names[9], "A", "Dominant");
        assert_eq!(names[10], "Bb", "Submediant is flat, from key sig");
        assert_eq!(names[1], "C#", "Leading tone is sharp, raised VII#");
    }

    #[test]
    fn diatonic_name_g_harmonic_minor() {
        // G harmonic minor: G A Bb C D Eb F#
        let scale = find_scale("Harmonic Minor").unwrap();
        let tonic = Pitch::parse("G").unwrap();
        let names = build_note_names(tonic, scale);

        assert_eq!(names[7], "G", "Tonic");
        assert_eq!(names[9], "A", "Supertonic");
        assert_eq!(names[10], "Bb", "IIIb is from key sig (2 flats)");
        assert_eq!(names[0], "C", "Subdominant");
        assert_eq!(names[2], "D", "Dominant");
        assert_eq!(names[3], "Eb", "VIb is from key sig");
        assert_eq!(names[6], "F#", "VII# leading tone is sharp");
    }

    #[test]
    fn diatonic_name_bb_harmonic_minor() {
        // Bb harmonic minor: Bb C Db Eb F Gb A= (raised VII# = A natural)
        let scale = find_scale("Harmonic Minor").unwrap();
        let tonic = Pitch::parse("Bb").unwrap();
        let names = build_note_names(tonic, scale);

        assert_eq!(names[10], "Bb", "Tonic");
        assert_eq!(names[0], "C", "Supertonic");
        assert_eq!(names[1], "Db", "IIIb");
        assert_eq!(names[3], "Eb", "Subdominant");
        assert_eq!(names[5], "F", "Dominant");
        assert_eq!(names[6], "Gb", "VIb");
        assert_eq!(names[9], "A", "VII# is raised from Ab to A natural");
    }

    #[test]
    fn diatonic_name_gsharp_harmonic_minor() {
        // G# harmonic minor: G# A# B C# D# E F## (double sharp)
        let scale = find_scale("Harmonic Minor").unwrap();
        let tonic = Pitch::parse("G#").unwrap();
        let names = build_note_names(tonic, scale);

        assert_eq!(names[8], "G#", "Tonic");
        assert_eq!(names[10], "A#", "Supertonic");
        assert_eq!(names[11], "B", "Mediant");
        assert_eq!(names[1], "C#", "Subdominant");
        assert_eq!(names[3], "D#", "Dominant");
        assert_eq!(names[4], "E", "Submediant");
        // F## = semitone 7 (enharmonic G=); the diatonic letter for pos 6 is F,
        // raised by 2 semitones above F= (semitone 5).
        assert_eq!(
            names[7], "F##",
            "VII# leading tone is double sharp (enharmonic G=)"
        );
    }

    #[test]
    fn diatonic_name_melodic_minor() {
        // D melodic minor: D E F G A B C#
        let scale = find_scale("Melodic Minor").unwrap();
        let tonic = Pitch::parse("D").unwrap();
        let names = build_note_names(tonic, scale);

        assert_eq!(names[2], "D", "Tonic");
        assert_eq!(names[4], "E", "Supertonic");
        assert_eq!(names[5], "F", "Mediant");
        assert_eq!(names[7], "G", "Subdominant");
        assert_eq!(names[9], "A", "Dominant");
        assert_eq!(names[11], "B", "VI# is raised from Bb");
        assert_eq!(names[1], "C#", "VII# leading tone");
    }

    #[test]
    fn lydian_note_names_f() {
        // F Lydian: F G A B C D E  (IV# = B natural, not Cb)
        let scale = find_scale("Lydian").unwrap();
        let tonic = Pitch::parse("F").unwrap();
        let names = build_note_names(tonic, scale);
        assert_eq!(names[11], "B", "IV# = B natural in F Lydian");
        assert_eq!(names[5], "F", "Tonic");
    }

    #[test]
    fn locrian_note_names_c() {
        // C Locrian: C Db Eb F Gb Ab Bb  (Vb = Gb, not F#)
        let scale = find_scale("Locrian").unwrap();
        let tonic = Pitch::parse("C").unwrap();
        let names = build_note_names(tonic, scale);
        assert_eq!(names[6], "Gb", "Vb in C Locrian = Gb, not F#");
    }

    #[test]
    fn phrygian_note_names_e() {
        // E Phrygian: E F G A B C D
        let scale = find_scale("Phrygian").unwrap();
        let tonic = Pitch::parse("E").unwrap();
        let names = build_note_names(tonic, scale);
        assert_eq!(names[5], "F", "IIb in E Phrygian = F");
    }
}
