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

use crate::scale::Scale;

/// Ionian (major) intervals, reference for quality suffixes (♭/♯ annotations).
const IONIAN_INTERVALS: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

/// Roman-numeral degree symbol for any chromatic interval (0-11).
///
/// This is the canonical harmonic label regardless of scale membership;
/// used in the "Deg" display column and in comment strings for chromatic notes.
pub fn interval_to_degree_symbol(interval: u8) -> &'static str {
    match interval {
        0 => "I",
        1 => "IIb",
        2 => "II",
        3 => "IIIb",
        4 => "III",
        5 => "IV",
        6 => "IV#/Vb",
        7 => "V",
        8 => "VIb",
        9 => "VI",
        10 => "VIIb",
        11 => "VII",
        _ => "?",
    }
}

/// Degree name for a scale degree at position `pos` (0-indexed) in a 7-note
/// scale.
///
/// Quality suffix (♭/♯) is computed by comparing to the parallel Ionian
/// interval, so Dorian's pos-2 (interval 3) yields `"Mediant♭"` and Lydian's
/// pos-3 (interval 6) yields `"Subdominant♯"`.
///
/// The seventh degree (pos 6) is a special case: `"Leading tone"` (interval
/// 11) and `"Subtonic"` (interval 10) are qualitatively distinct roles, not
/// the same degree with a flat.  No ♭/♯ suffix is added; the name itself
/// encodes the distinction.
pub fn degree_name_heptatonic(pos: usize, interval: u8) -> String {
    const BASE_NAMES: [&str; 7] = [
        "Tonic",
        "Supertonic",
        "Mediant",
        "Subdominant",
        "Dominant",
        "Submediant",
        "Leading tone",
    ];

    if pos == 6 {
        return if interval == 11 {
            "Leading tone".to_string()
        } else {
            "Subtonic".to_string()
        };
    }

    let base = BASE_NAMES[pos];
    let ionian_int = IONIAN_INTERVALS[pos];
    let diff = interval as i16 - ionian_int as i16;
    let suffix = match diff {
        1 => "♯",
        -1 => "♭",
        2 => "𝄪",
        -2 => "𝄫",
        _ => "",
    };
    format!("{base}{suffix}")
}

/// Harmonic-function label: `"(T)"` tonic, `"(S)"` subdominant/pre-dominant,
/// `"(D)"` dominant, or `"   "` (no stable function).
///
/// `is_major_context` is `true` for Lydian, Ionian, Mixolydian
/// (`mode_fifth_offset ≥ −1`).  It determines whether the Submediant (pos 5)
/// is a tonic parallel (major context) or a subdominant preparation (minor).
///
/// Locrian's Vb (pos 4, interval 6) destroys dominant function → `"   "`.
pub fn function_label(pos: usize, interval: u8, is_major_context: bool) -> &'static str {
    match pos {
        0 => "(T)",
        1 => "(S)",
        2 => "(T)",
        3 => "(S)",
        4 => {
            if interval == 6 { "   " } else { "(D)" } // Locrian Vb: no dominant function
        }
        5 => {
            if is_major_context {
                "(T)"
            } else {
                "(S)"
            }
        }
        6 => "(D)",
        _ => "   ",
    }
}

/// Returns `true` when this scale degree is the **characteristic tone** of its
/// mode, the single interval that most distinguishes it from its neighbour on
/// the Circle of Fifths brightness spectrum:
///
/// | Mode       | Characteristic vs. neighbour   |
/// |------------|--------------------------------|
/// | Lydian     | IV# (pos 3, int 6) vs Ionian   |
/// | Ionian     | none (reference)               |
/// | Mixolydian | VIIb (pos 6, int 10) vs Ionian |
/// | Dorian     | VI=  (pos 5, int 9) vs Aeolian |
/// | Aeolian    | none (reference minor)         |
/// | Phrygian   | IIb  (pos 1, int 1) vs Aeolian |
/// | Locrian    | Vb   (pos 4, int 6) vs Phrygian|
///
/// Harmonic and Melodic Minor characteristic tones are handled separately via
/// [`comment_minor_variant`].
pub fn is_characteristic(pos: usize, interval: u8, mode_fifth_offset: i8) -> bool {
    match mode_fifth_offset {
        1 => pos == 3 && interval == 6,   // Lydian:      IV#
        0 => false,                       // Ionian:      reference
        -1 => pos == 6 && interval == 10, // Mixolydian:  VIIb
        -2 => pos == 5 && interval == 9,  // Dorian:      VI= (vs Aeolian's VIb)
        -3 => false,                      // Aeolian:     reference minor
        -4 => pos == 1 && interval == 1,  // Phrygian:    IIb
        -5 => pos == 4 && interval == 6,  // Locrian:     Vb
        _ => false,
    }
}

/// Comment for an in-scale note in a pure diatonic 7-note mode.
fn comment_diatonic_heptatonic(
    pos: usize,
    interval: u8,
    mode_fifth_offset: i8,
    is_major_context: bool,
) -> String {
    let func = function_label(pos, interval, is_major_context);
    let name = degree_name_heptatonic(pos, interval);
    let marker = if is_characteristic(pos, interval, mode_fifth_offset) {
        " [char.]"
    } else {
        ""
    };
    format!("{func} {name}{marker}")
}

/// Comment for an in-scale note in Harmonic or Melodic Minor.
///
/// These scales share the Aeolian key signature but raise one (harmonic) or
/// two (melodic) scale degrees.  The raised degrees are characteristic and
/// receive an explicit # annotation.  All other degrees fall through to the
/// standard Aeolian-context comment.
fn comment_minor_variant(pos: usize, interval: u8, scale_name: &str) -> String {
    match (scale_name, pos, interval) {
        ("Harmonic Minor", 4, 7) => "(D) Dominant".to_string(),
        ("Harmonic Minor", 6, 11) => "(D) Leading tone VII♯ [char.]".to_string(),
        ("Melodic Minor", 5, 9) => "(S) Submediant VI♯ [char.]".to_string(),
        ("Melodic Minor", 6, 11) => "(D) Leading tone VII♯ [char.]".to_string(),
        _ => comment_diatonic_heptatonic(pos, interval, -3, false),
    }
}

/// Comment for a note in a non-heptatonic scale.
fn comment_non_heptatonic(interval: u8, scale: &Scale) -> String {
    let pos = scale.intervals.iter().position(|&x| x == interval).unwrap();

    if scale.name == "Pentatonic" {
        // [0, 2, 4, 7, 9]: 5 degrees
        let (func, name) = match pos {
            0 => ("(T)", "Tonic"),
            1 => ("(S)", "Supertonic"),
            2 => ("(T)", "Mediant"),
            3 => ("(D)", "Dominant"),
            4 => ("(T)", "Submediant"),
            _ => ("   ", "?"),
        };
        return format!("{func} {name}");
    }

    if scale.name == "No Scale" {
        let sym = interval_to_degree_symbol(interval);
        let func = match interval {
            0 => "(T)",
            2 | 5 => "(S)",
            7 | 11 => "(D)",
            9 => "(T)", // Submediant: tonic parallel in major context
            _ => "   ",
        };
        return format!("{func} {sym}");
    }

    format!("    interval {interval}")
}

/// Brief description of a chromatic (out-of-scale) note.
///
/// Each interval has a standard theoretical role relative to the tonal centre;
/// the description is refined for major vs minor context where they differ.
fn comment_chromatic(interval: u8, mode_fifth_offset: i8) -> String {
    let sym_raw = interval_to_degree_symbol(interval);
    let sym = sym_raw
        .replace("bb", "𝄫")
        .replace("##", "𝄪")
        .replace('b', "♭")
        .replace('#', "♯");
    let is_minor = mode_fifth_offset <= -2;

    let desc = match interval {
        1 => {
            if is_minor {
                "Neapolitan or II♭ (borrowed from Phrygian)"
            } else {
                "Neapolitan chord (II♭, borrowed from Phrygian)"
            }
        }
        3 => {
            if is_minor {
                "III♭ (parallel major mediant)"
            } else {
                "III♭ (borrowed from parallel minor)"
            }
        }
        4 => {
            if is_minor {
                "III♮ (from harmonic or melodic minor)"
            } else {
                "III (Mediant)"
            }
        }
        6 => "Tritone (IV♯ or V♭)",
        8 => {
            if is_minor {
                "VI♭ (submediant of parallel major)"
            } else {
                "VI♭ (borrowed from parallel minor; chromatic mediant)"
            }
        }
        10 => {
            if is_minor {
                "VII♭ (Subtonic, Aeolian VII)"
            } else {
                "VII♭ (borrowed from parallel minor or Mixolydian)"
            }
        }
        11 => {
            if is_minor {
                "VII♯ leading tone (harmonic minor)"
            } else {
                "VII (Leading tone)"
            }
        }
        9 => {
            if is_minor {
                "VI♮ (Dorian or melodic minor)"
            } else {
                "VI (Submediant)"
            }
        }
        2 => "II (Supertonic)",
        5 => "IV (Subdominant)",
        7 => "V (Dominant)",
        0 => "I (Tonic)",
        _ => "",
    };
    format!("    {sym}, {desc}")
}

/// Full comment for a note at the given interval.
///
/// Dispatches to the appropriate helper depending on whether the note is
/// in-scale or chromatic, and on the scale type.
pub fn build_comment(interval: u8, in_scale: bool, scale: &Scale) -> String {
    if !in_scale {
        return comment_chromatic(interval, scale.mode_fifth_offset);
    }

    let is_major_context = scale.mode_fifth_offset >= -1;

    if scale.intervals.len() == 7 {
        let pos = scale.intervals.iter().position(|&x| x == interval).unwrap();
        match scale.name {
            "Harmonic Minor" | "Melodic Minor" => comment_minor_variant(pos, interval, scale.name),
            _ => comment_diatonic_heptatonic(
                pos,
                interval,
                scale.mode_fifth_offset,
                is_major_context,
            ),
        }
    } else {
        comment_non_heptatonic(interval, scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::{SCALES, find_scale};

    #[test]
    fn comment_format_major() {
        let scale = find_scale("Major").unwrap();
        assert_eq!(build_comment(0, true, scale), "(T) Tonic");
        assert_eq!(build_comment(2, true, scale), "(S) Supertonic");
        assert_eq!(build_comment(4, true, scale), "(T) Mediant");
        assert_eq!(build_comment(5, true, scale), "(S) Subdominant");
        assert_eq!(build_comment(7, true, scale), "(D) Dominant");
        assert_eq!(build_comment(9, true, scale), "(T) Submediant");
        assert_eq!(build_comment(11, true, scale), "(D) Leading tone");
    }

    #[test]
    fn comment_characteristic_marking() {
        let lydian = find_scale("Lydian").unwrap();
        let mixo = find_scale("Mixolydian").unwrap();
        let dorian = find_scale("Dorian").unwrap();
        let phrygian = find_scale("Phrygian").unwrap();
        let locrian = find_scale("Locrian").unwrap();

        assert!(
            build_comment(6, true, lydian).contains("[char.]"),
            "Lydian IV# should be [char.]"
        );
        assert!(
            !build_comment(0, true, lydian).contains("[char.]"),
            "Tonic should not be [char.]"
        );
        assert!(
            build_comment(10, true, mixo).contains("[char.]"),
            "Mixolydian VIIb should be [char.]"
        );
        assert!(
            build_comment(9, true, dorian).contains("[char.]"),
            "Dorian VI= should be [char.]"
        );
        assert!(
            build_comment(1, true, phrygian).contains("[char.]"),
            "Phrygian IIb should be [char.]"
        );
        assert!(
            build_comment(6, true, locrian).contains("[char.]"),
            "Locrian Vb should be [char.]"
        );
    }

    #[test]
    fn comment_locrian_no_dominant_function() {
        let locrian = find_scale("Locrian").unwrap();
        let comment = build_comment(6, true, locrian); // Vb
        assert!(
            comment.starts_with("   "),
            "Locrian Vb has no dominant function: {comment}"
        );
    }

    #[test]
    fn comment_harmonic_minor_raised_degrees() {
        let scale = find_scale("Harmonic Minor").unwrap();
        let lt = build_comment(11, true, scale);
        assert!(
            lt.contains("VII♯"),
            "Harmonic minor leading tone should be VII♯: {lt}"
        );
        assert!(
            lt.contains("[char.]"),
            "VII♯ should be characteristic: {lt}"
        );
        assert!(
            lt.starts_with("(D)"),
            "Leading tone is dominant function: {lt}"
        );
    }

    #[test]
    fn comment_melodic_minor_raised_degrees() {
        let scale = find_scale("Melodic Minor").unwrap();
        let vi = build_comment(9, true, scale);
        let vii = build_comment(11, true, scale);
        assert!(vi.contains("VI♯"), "Melodic minor raised VI: {vi}");
        assert!(vii.contains("VII♯"), "Melodic minor raised VII: {vii}");
    }

    #[test]
    fn comment_out_of_scale_chromatic() {
        let major = find_scale("Major").unwrap();
        let neap = build_comment(1, false, major);
        let flat7 = build_comment(10, false, major);

        assert!(
            neap.starts_with("   "),
            "Out-of-scale comment starts with spaces: {neap}"
        );
        assert!(neap.contains("II♭"), "Should label the degree: {neap}");
        assert!(flat7.contains("VII♭"), "Should label VII♭: {flat7}");
    }

    #[test]
    fn comment_aeolian_subtonic() {
        let scale = find_scale("Aeolian").unwrap();
        let comment = build_comment(10, true, scale);
        assert!(
            comment.contains("Subtonic"),
            "Aeolian VII is a subtonic: {comment}"
        );
        assert!(
            !comment.contains("Leading"),
            "Should NOT say 'Leading tone': {comment}"
        );
    }

    #[test]
    fn degree_name_quality_suffixes() {
        for (pos, &int) in IONIAN_INTERVALS.iter().enumerate() {
            let name = degree_name_heptatonic(pos, int);
            assert!(
                !name.contains('♭') && !name.contains('♯'),
                "Ionian pos {pos} (interval {int}) should have no suffix: {name}"
            );
        }
        assert_eq!(degree_name_heptatonic(3, 6), "Subdominant♯");
        assert_eq!(degree_name_heptatonic(1, 1), "Supertonic♭");
        assert_eq!(degree_name_heptatonic(4, 6), "Dominant♭");
        assert_eq!(degree_name_heptatonic(6, 10), "Subtonic");
        assert_eq!(degree_name_heptatonic(6, 11), "Leading tone");
    }

    #[test]
    fn function_label_major_submediant_is_tonic() {
        assert_eq!(function_label(5, 9, true), "(T)");
        assert_eq!(function_label(5, 8, false), "(S)");
    }

    #[test]
    fn function_label_locrian_flat_fifth_no_dominant() {
        assert_eq!(function_label(4, 6, false), "   ");
        assert_eq!(function_label(4, 7, false), "(D)");
    }

    #[test]
    fn build_comment_never_panics_for_any_scale_and_interval() {
        for scale in SCALES {
            for &interval in scale.intervals {
                let c = build_comment(interval, true, scale);
                assert!(
                    !c.is_empty(),
                    "Scale '{}', interval {interval}: comment is empty",
                    scale.name
                );
            }
            for interval in 0u8..12 {
                if !scale.intervals.contains(&interval) {
                    let c = build_comment(interval, false, scale);
                    assert!(
                        !c.is_empty(),
                        "Scale '{}', chromatic interval {interval}: out-of-scale comment empty",
                        scale.name
                    );
                }
            }
        }
    }
}
