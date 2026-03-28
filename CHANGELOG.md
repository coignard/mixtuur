# Changelog

## 0.1.1

### Added
- `--push` flag: injects the generated palette directly into Cubase's
  `UserPreferences.xml` (macOS and Windows)
- `cubase` module with XML rewriting via `quick-xml` and path discovery
  via `dirs`
- `color::get_pitch_hsv()`: extracted shared colour-computation logic
  used by both the terminal renderer and the Cubase exporter

## 0.1.0

### Added

- Initial commit
