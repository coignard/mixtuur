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

use anyhow::{Result, anyhow};
use clap::Parser;
use colored::Colorize;

use mixtuur::cli::Cli;
use mixtuur::pitch::Pitch;
use mixtuur::render::print_colors;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let tonic = Pitch::parse(&cli.tonic).ok_or_else(|| {
        anyhow!(
            "Unknown note '{}'. \
             Valid: C C# Db D D# Eb E F F# Gb G G# Ab A A# Bb B",
            cli.tonic
        )
    })?;

    print_colors(tonic, cli.scale.as_scale(), cli.explain);

    if cli.push
        && let Err(e) = mixtuur::cubase::push_to_cubase(tonic, cli.scale.as_scale())
    {
        eprintln!("{} {:#}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    Ok(())
}
