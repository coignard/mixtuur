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

use palette::{Hsv, IntoColor, Srgb};

// Malinowski maps the artist's colour wheel onto the circle of fifths.
// Adjacent fifths are adjacent hues; the tonic (I) is blue.
//
// Each entry is indexed by CoF position (0 = I, 1 = V, 2 = II ...).
// To convert a semitone interval to its CoF position: `(interval × 7) % 12`.
// This is the mathematical core: semitone × 7 ≡ fifths (mod 12).
//
// Hex values match the Cubase 12 "Harmonic Colors" pitch palette exactly.

/// RGB triples for the twelve Circle-of-Fifths positions (0 = tonic, 1 = P5 ...).
pub const COF_COLORS: [(u8, u8, u8); 12] = [
    (0x3D, 0x70, 0xE8), // I
    (0x7C, 0x4E, 0xED), // V
    (0xA7, 0x4C, 0xF0), // II
    (0xD2, 0x47, 0xF0), // VI
    (0xF9, 0x19, 0x2D), // III
    (0xF3, 0x8B, 0x1B), // VII
    (0xE3, 0xE3, 0x00), // IV#/Vb
    (0xEB, 0xB7, 0x00), // IV
    (0x00, 0xDF, 0x50), // I#/IIb
    (0x00, 0xDF, 0xB1), // V#/VIb
    (0x00, 0xDF, 0xDE), // II#/IIIb
    (0x1D, 0x94, 0xE4), // VIIb
];

/// Converts an 8-bit RGB triple to the HSV colour space.
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> Hsv {
    Srgb::new(r, g, b).into_format::<f32>().into_color()
}

/// Converts an HSV colour back to an 8-bit RGB triple.
pub fn hsv_to_rgb(hsv: Hsv) -> (u8, u8, u8) {
    let srgb: Srgb<f32> = hsv.into_color();
    srgb.into_format::<u8>().into_components()
}

/// Returns a desaturated copy of `hsv` for out-of-scale notes.
///
/// Saturation is reduced to 30 % of its original value; hue and value are
/// preserved so the note's harmonic colour remains recognisable but
/// visually subordinate to the in-scale pitches.
pub fn ghost(mut hsv: Hsv) -> Hsv {
    hsv.saturation *= 0.3;
    hsv
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The CoF mapping `(interval × 7) mod 12` is the mathematical core of
    /// Malinowski's Harmonic Coloring.  Verify key theoretical properties:
    ///   - Root and P5 are adjacent on the CoF (distance 1)
    ///   - M3 is 4 steps away; m3 is 9 steps (= 3 steps counter-clockwise)
    ///   - Tritone is exactly opposite (6 steps)
    #[test]
    fn cof_mapping_theory() {
        let cof = |i: u8| (u32::from(i) * 7 % 12) as usize;
        assert_eq!(cof(0), 0, "Unison: CoF position 0");
        assert_eq!(cof(7), 1, "P5: one step clockwise");
        assert_eq!(cof(2), 2, "M2: two steps");
        assert_eq!(cof(9), 3, "M6: three steps");
        assert_eq!(cof(4), 4, "M3: four steps");
        assert_eq!(cof(11), 5, "M7: five steps");
        assert_eq!(cof(6), 6, "TT: directly opposite");
        assert_eq!(cof(1), 7, "m2: seven steps (= 5 steps ccw)");
        assert_eq!(cof(8), 8, "m6");
        assert_eq!(cof(3), 9, "m3: nine steps (= 3 steps ccw)");
        assert_eq!(cof(10), 10, "m7");
        assert_eq!(cof(5), 11, "P4: eleven steps (= 1 step ccw)");
    }

    /// The tonic (I) must be assigned the blue colour (first COF_COLORS entry).
    #[test]
    fn cof_tonic_is_blue() {
        let (r, g, b) = COF_COLORS[0];
        assert!(r < 100, "tonic blue: low red");
        assert!(g < 150, "tonic blue: moderate green");
        assert!(b > 200, "tonic blue: high blue");
    }

    /// The tritone (IV#/Vb, CoF position 6) should be yellow as the
    /// complementary colour to blue, as in Malinowski's wheel.
    #[test]
    fn cof_tritone_is_yellow() {
        let (r, g, b) = COF_COLORS[6];
        assert!(r > 200, "tritone yellow: high red");
        assert!(g > 200, "tritone yellow: high green");
        assert!(b < 50, "tritone yellow: low blue");
    }

    #[test]
    fn ghost_reduces_saturation_preserves_hue_and_value() {
        let original = Hsv::new(180.0f32, 1.0f32, 0.9f32);
        let ghosted = ghost(original);
        assert_eq!(ghosted.hue, original.hue);
        assert!((ghosted.value - 0.9).abs() < f32::EPSILON);
        assert!((ghosted.saturation - 0.3).abs() < f32::EPSILON);
    }
}
