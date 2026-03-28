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

use colored::Colorize;

use crate::color::{get_pitch_hsv, hsv_to_rgb};
use crate::degree::{build_comment, interval_to_degree_symbol};
use crate::note::build_note_names;
use crate::pitch::Pitch;
use crate::scale::Scale;

/// Prints the twelve-pitch colour table for the given tonic and scale to stdout.
///
/// Each row shows the pitch name, degree symbol, HSV hue/saturation/value, and
/// (when `explain` is `true`) a harmonic function comment.  In-scale notes are
/// printed at full saturation; out-of-scale notes are desaturated ("ghost")
/// and dimmed.  The tonic is marked with `▶`, other in-scale pitches with `●`.
///
/// A separator line is drawn beneath the column headers; its width is computed
/// automatically from the header string so it always matches exactly.
pub fn print_colors(tonic: Pitch, scale: &Scale, explain: bool) {
    let note_names = build_note_names(tonic, scale);

    let comment_hdr = if explain { "  Comment" } else { "" };
    let header = format!(
        "{:<5}  {:<9}  {:<3}  {:<3}  {:<3}{}",
        "Pitch", "Degree", "H", "S", "V", comment_hdr
    );
    let mut max_width = 30;
    if explain {
        max_width += 9;
        for i in 0u8..12 {
            let interval = (i + 12 - tonic.semitone) % 12;
            let in_scale = scale.intervals.contains(&interval);
            let comment = build_comment(interval, in_scale, scale);
            let row_width = 31 + 2 + comment.chars().count();
            if row_width > max_width {
                max_width = row_width;
            }
        }
    }
    let separator = "-".repeat(max_width);

    println!();
    println!("     {}", header);
    println!("     {}", separator);

    for i in 0u8..12 {
        let interval = (i + 12 - tonic.semitone) % 12;
        let name = &note_names[i as usize];

        let (render_hsv, in_scale) = get_pitch_hsv(i, tonic, scale);
        let (dot_r, dot_g, dot_b) = hsv_to_rgb(render_hsv);

        let base_h = render_hsv.hue.into_positive_degrees().round() as u16;
        let render_s = (render_hsv.saturation * 100.0).round() as u8;
        let render_v = (render_hsv.value * 100.0).round() as u8;

        let dot_char = if interval == 0 {
            "▶"
        } else if in_scale {
            "●"
        } else {
            " "
        };

        let deg_sym = interval_to_degree_symbol(interval);
        let padded_name = if in_scale {
            format!("{:<5}", name)
        } else {
            format!("{:>5}", name)
        };
        let padded_deg = format!("{:<9}", deg_sym);
        let padded_h = format!("{:<3}", base_h);
        let padded_s = format!("{:<3}", render_s);
        let padded_v = format!("{:<3}", render_v);

        if in_scale {
            let comment = if explain {
                format!("  {}", build_comment(interval, true, scale))
            } else {
                String::new()
            };
            println!(
                "  {}  {}  {}  {}  {}  {}{}",
                dot_char.truecolor(dot_r, dot_g, dot_b),
                padded_name,
                padded_deg,
                padded_h,
                padded_s,
                padded_v,
                comment,
            );
        } else {
            let comment = if explain {
                format!("  {}", build_comment(interval, false, scale))
            } else {
                String::new()
            };
            println!(
                "     {}  {}  {}  {}  {}{}",
                padded_name.as_str().dimmed(),
                padded_deg.as_str().dimmed(),
                padded_h.as_str().dimmed(),
                padded_s.as_str().dimmed(),
                padded_v.as_str().dimmed(),
                comment.dimmed(),
            );
        }
    }
    println!();
}
