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

use clap::{Parser, ValueEnum};

/// All scales available for colour generation.
#[derive(Clone, Debug, ValueEnum)]
pub enum ScaleChoice {
    /// Major scale.
    #[value(name = "major", alias = "ionian")]
    Major,

    /// Natural minor scale.
    #[value(name = "aeolian", aliases = ["minor", "naturalminor"])]
    Aeolian,

    /// Harmonic minor scale (raised VII).
    #[value(name = "harmonic-minor", aliases = ["harmonic", "harmonicminor"])]
    HarmonicMinor,

    /// Melodic minor scale (raised VI and VII).
    #[value(name = "melodic-minor", aliases = ["melodic", "melodicminor"])]
    MelodicMinor,

    /// Dorian mode.
    #[value(name = "dorian")]
    Dorian,

    /// Phrygian mode.
    #[value(name = "phrygian")]
    Phrygian,

    /// Lydian mode (raised IV).
    #[value(name = "lydian")]
    Lydian,

    /// Mixolydian mode (lowered VII).
    #[value(name = "mixolydian", alias = "myxolydian")]
    Mixolydian,

    /// Locrian mode (lowered II and V).
    #[value(name = "locrian")]
    Locrian,

    /// Major pentatonic scale.
    #[value(name = "pentatonic", aliases = ["majorpentatonic", "major-pentatonic"])]
    Pentatonic,

    /// Full chromatic palette.
    #[value(name = "chromatic", aliases = ["all", "no-scale", "noscale"])]
    NoScale,
}

impl ScaleChoice {
    /// Resolves this choice to a reference to the corresponding built-in [`Scale`].
    pub fn as_scale(&self) -> &'static crate::scale::Scale {
        let name = match self {
            Self::Major => "Major",
            Self::Aeolian => "Aeolian",
            Self::HarmonicMinor => "Harmonic Minor",
            Self::MelodicMinor => "Melodic Minor",
            Self::Dorian => "Dorian",
            Self::Phrygian => "Phrygian",
            Self::Lydian => "Lydian",
            Self::Mixolydian => "Mixolydian",
            Self::Locrian => "Locrian",
            Self::Pentatonic => "Pentatonic",
            Self::NoScale => "No Scale",
        };
        crate::scale::find_scale(name).expect("built-in scale name is always valid")
    }
}

/// Command-line arguments for mixtuur.
///
/// Run `mixtuur --help` to see all accepted tonic names and scale variants.
#[derive(Parser, Debug)]
#[command(author, version, about = "Harmonic Pitch Color Generator for Cubase")]
pub struct Cli {
    /// Root note of the key.
    pub tonic: String,

    /// Scale or mode to apply.
    pub scale: ScaleChoice,

    /// Show harmonic function labels for every pitch.
    #[arg(long)]
    pub explain: bool,

    /// Push the generated palette directly to Cubase's UserPreferences.xml
    #[arg(long)]
    pub push: bool,
}
