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

/// A musical scale defined by ascending semitone intervals from the tonic.
#[derive(Debug)]
pub struct Scale {
    /// Display name, e.g. `"Harmonic Minor"`.
    pub name: &'static str,

    /// Alternative names accepted by [`find_scale`], e.g. `["minor", "naturalminor"]`.
    pub aliases: &'static [&'static str],

    /// Ascending semitone offsets from the tonic; the first element must be 0.
    pub intervals: &'static [u8],

    /// Circle-of-Fifths offset of this mode relative to Ionian (0).
    ///
    /// Used to determine flat/sharp key-signature spelling for out-of-scale
    /// notes.
    ///
    /// | Mode       | Offset | Rationale                       |
    /// |------------|--------|---------------------------------|
    /// | Lydian     |   +1   | raised IV = one fifth clockwise |
    /// | Ionian     |   --   | reference                       |
    /// | Mixolydian |   −1   | lowered VII = one fifth ccw     |
    /// | Dorian     |   −2   | two fifths ccw                  |
    /// | Aeolian    |   −3   | relative minor of Ionian        |
    /// | Phrygian   |   −4   | four fifths ccw                 |
    /// | Locrian    |   −5   | five fifths ccw                 |
    ///
    /// Harmonic Minor and Melodic Minor use −3 (same key signature as Aeolian);
    /// their raised degrees are chromatic accidentals inside the key.
    pub mode_fifth_offset: i8,
}

/// All built-in scales.
pub static SCALES: &[Scale] = &[
    Scale {
        name: "Major",
        aliases: &["ionian"],
        intervals: &[0, 2, 4, 5, 7, 9, 11],
        mode_fifth_offset: 0,
    },
    Scale {
        name: "Aeolian",
        aliases: &["minor", "naturalminor"],
        intervals: &[0, 2, 3, 5, 7, 8, 10],
        mode_fifth_offset: -3,
    },
    Scale {
        name: "Harmonic Minor",
        aliases: &["harmonic"],
        intervals: &[0, 2, 3, 5, 7, 8, 11],
        mode_fifth_offset: -3, // Key sig = natural minor; VII# is an in-key accidental
    },
    Scale {
        name: "Melodic Minor",
        aliases: &["melodic"],
        intervals: &[0, 2, 3, 5, 7, 9, 11],
        mode_fifth_offset: -3, // Key sig = natural minor; VI# VII# are in-key accidentals
    },
    Scale {
        name: "Dorian",
        aliases: &[],
        intervals: &[0, 2, 3, 5, 7, 9, 10],
        mode_fifth_offset: -2,
    },
    Scale {
        name: "Phrygian",
        aliases: &[],
        intervals: &[0, 1, 3, 5, 7, 8, 10],
        mode_fifth_offset: -4,
    },
    Scale {
        name: "Lydian",
        aliases: &[],
        intervals: &[0, 2, 4, 6, 7, 9, 11],
        mode_fifth_offset: 1,
    },
    Scale {
        name: "Mixolydian",
        aliases: &["myxolydian"],
        intervals: &[0, 2, 4, 5, 7, 9, 10],
        mode_fifth_offset: -1,
    },
    Scale {
        name: "Locrian",
        aliases: &[],
        intervals: &[0, 1, 3, 5, 6, 8, 10],
        mode_fifth_offset: -5,
    },
    Scale {
        name: "Pentatonic",
        aliases: &["majorpentatonic"],
        intervals: &[0, 2, 4, 7, 9],
        mode_fifth_offset: 0,
    },
    Scale {
        name: "No Scale",
        aliases: &["chromatic", "all"],
        intervals: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        mode_fifth_offset: 0,
    },
];

/// Strips non-alphanumeric characters and lowercases `s`, producing a
/// normalised key used for fuzzy scale matching.
fn normalize_string(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
}

/// Looks up a scale by name or alias, ignoring case and non-alphanumeric
/// characters.
///
/// Returns `None` when no scale matches.
///
/// # Examples
///
/// ```
/// use mixtuur::scale::find_scale;
///
/// assert_eq!(find_scale("Major").unwrap().name, "Major");
/// assert_eq!(find_scale("IONIAN").unwrap().name, "Major");
/// assert_eq!(find_scale("harmonicminor").unwrap().name, "Harmonic Minor");
/// assert!(find_scale("Lydian Dominant").is_none());
/// ```
pub fn find_scale(name: &str) -> Option<&'static Scale> {
    let key = normalize_string(name);
    SCALES.iter().find(|s| {
        normalize_string(s.name) == key || s.aliases.iter().any(|&a| normalize_string(a) == key)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_lookup_aliases_and_normalization() {
        assert_eq!(find_scale("Harmonic Minor").unwrap().name, "Harmonic Minor");
        assert_eq!(find_scale("harmonicminor").unwrap().name, "Harmonic Minor");
        assert_eq!(find_scale("IONIAN").unwrap().name, "Major");
        assert_eq!(find_scale("minor").unwrap().name, "Aeolian");
        assert_eq!(find_scale("Myxolydian").unwrap().name, "Mixolydian");
        assert!(find_scale("Lydian Dominant").is_none());
    }
}
