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

use anyhow::{Context, Result, bail};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::fs;
use std::path::PathBuf;

use crate::color::{get_pitch_hsv, hsv_to_rgb};
use crate::pitch::Pitch;
use crate::scale::Scale;

/// Finds the highest installed version of Cubase's `UserPreferences.xml`.
///
/// Searches the platform-specific preferences directory:
/// - macOS: `~/Library/Preferences/Cubase */UserPreferences.xml`
/// - Windows: `%APPDATA%/Steinberg/Cubase *_64/UserPreferences.xml`
///
/// When multiple Cubase versions are found, returns the lexicographically
/// highest path (e.g. "Cubase 15" wins over "Cubase 14").
fn find_cubase_prefs() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(pref_dir) = dirs::preference_dir()
        && let Ok(entries) = std::fs::read_dir(&pref_dir)
    {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("Cubase ") {
                let path = entry.path().join("UserPreferences.xml");
                if path.exists() {
                    candidates.push(path);
                }
            }
        }
    }

    if let Some(config_dir) = dirs::config_dir() {
        let steinberg_dir = config_dir.join("Steinberg");
        if let Ok(entries) = std::fs::read_dir(&steinberg_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("Cubase ") {
                    let path = entry.path().join("UserPreferences.xml");
                    if path.exists() {
                        candidates.push(path);
                    }
                }
            }
        }
    }

    candidates.sort_by(|a, b| b.cmp(a));
    candidates.into_iter().next()
}

/// Injects the generated 12-pitch palette directly into Cubase XML settings.
///
/// Locates `UserPreferences.xml`, creates a `.bak` backup alongside it, then
/// rewrites every `<int name="Color" .../>` element inside the
/// `pitchColors → Set` list with the ARGB value derived from the given tonic
/// and scale.
///
/// # Errors
///
/// Returns an error if no Cubase installation is found, if the backup cannot
/// be written, or if the XML cannot be parsed or serialised.
pub fn push_to_cubase(tonic: Pitch, scale: &Scale) -> Result<()> {
    let prefs_path = find_cubase_prefs()
        .context("Could not find any Cubase UserPreferences.xml on this system.")?;

    let backup_path = prefs_path.with_extension("xml.bak");
    fs::copy(&prefs_path, &backup_path)
        .context("Failed to create backup of UserPreferences.xml")?;

    println!("Found Cubase preferences: {}", prefs_path.display());
    println!("Backup created:           {}", backup_path.display());

    let mut reader = Reader::from_file(&prefs_path)?;
    reader.config_mut().trim_text(false);

    let mut writer = Writer::new(Vec::new());
    let mut buf = Vec::new();

    let mut in_pitch_colors = false;
    let mut in_set_list = false;
    let mut current_semitone: Option<u8> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"member"
                    && let Some(attr) = e.try_get_attribute("name")?
                    && attr.value.as_ref() == b"pitchColors"
                {
                    in_pitch_colors = true;
                } else if in_pitch_colors
                    && e.name().as_ref() == b"list"
                    && let Some(attr) = e.try_get_attribute("name")?
                    && attr.value.as_ref() == b"Set"
                {
                    in_set_list = true;
                }
                writer.write_event(Event::Start(e.clone()))?;
            }
            Ok(Event::Empty(e)) => {
                let mut e_mut = e.clone();

                if in_set_list {
                    if e.name().as_ref() == b"string"
                        && let Some(attr) = e.try_get_attribute("name")?
                        && attr.value.as_ref() == b"Name"
                        && let Some(val) = e.try_get_attribute("value")?
                    {
                        let name_str = String::from_utf8_lossy(val.value.as_ref());
                        current_semitone = match name_str.as_ref() {
                            "C" => Some(0),
                            "C#/Db" => Some(1),
                            "D" => Some(2),
                            "D#/Eb" => Some(3),
                            "E" => Some(4),
                            "F" => Some(5),
                            "F#/Gb" => Some(6),
                            "G" => Some(7),
                            "G#/Ab" => Some(8),
                            "A" => Some(9),
                            "A#/Bb" => Some(10),
                            "B" => Some(11),
                            _ => None,
                        };
                    } else if e.name().as_ref() == b"int"
                        && let Some(attr) = e.try_get_attribute("name")?
                        && attr.value.as_ref() == b"Color"
                        && let Some(semi) = current_semitone
                    {
                        let (hsv, _) = get_pitch_hsv(semi, tonic, scale);
                        let (r, g, b) = hsv_to_rgb(hsv);
                        let cubase_color: u32 =
                            (255 << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                        let val_str = cubase_color.to_string();

                        let mut new_elem = BytesStart::new("int");
                        new_elem.push_attribute(("name", "Color"));
                        new_elem.push_attribute(("value", val_str.as_str()));
                        e_mut = new_elem;
                    }
                }
                writer.write_event(Event::Empty(e_mut))?;
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"list" && in_set_list {
                    in_set_list = false;
                } else if e.name().as_ref() == b"member" && in_pitch_colors && !in_set_list {
                    in_pitch_colors = false;
                } else if e.name().as_ref() == b"item" {
                    current_semitone = None;
                }
                writer.write_event(Event::End(e.clone()))?;
            }
            Ok(e) => writer.write_event(e)?,
            Err(e) => bail!(
                "XML parsing error at pos {}: {:?}",
                reader.buffer_position(),
                e
            ),
        }
        buf.clear();
    }

    let result_xml = writer.into_inner();
    fs::write(&prefs_path, result_xml).context("Failed to write updated UserPreferences.xml")?;

    println!("Done.");
    Ok(())
}
