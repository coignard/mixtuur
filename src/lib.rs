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

//! # mixtuur
//!
//! `mixtuur` is a harmonic pitch colour generator for Cubase.
//!
//! Generates colour palettes for musical scales based on Stephen Malinowski's
//! Harmonic Coloring method, mapping the Circle of Fifths to HSV colour space.
//!
//! # Architecture
//!
//! ```text
//! CLI input (tonic + scale choice)
//!     → pitch::Pitch::parse()        → Pitch
//!     → cli::ScaleChoice::as_scale() → &Scale
//!     → note::build_note_names()     → [String; 12]
//!     → render::print_colors()       → terminal output
//! ```

#![warn(missing_docs)]

/// Command-line interface definition, including the [`cli::ScaleChoice`] enum.
pub mod cli;

/// Circle of Fifths palette and HSV colour helpers.
pub mod color;

/// Degree symbols, degree names, harmonic function labels, and comment
/// strings for in-scale and chromatic notes.
pub mod degree;

/// Diatonic, non-heptatonic, and chromatic note naming.
pub mod note;

/// [`Pitch`](pitch::Pitch) type with accidental spelling and Circle of Fifths
/// key-signature logic.
pub mod pitch;

/// Terminal rendering: the twelve-pitch colour table.
pub mod render;

/// [`Scale`](scale::Scale) catalogue and lookup by name.
pub mod scale;
