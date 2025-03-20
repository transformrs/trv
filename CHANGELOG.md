# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2025-03-20

### Added

- A `notes` command to extract the speaker notes from a presentation ([#42](https://github.com/transformrs/trv/pull/42)).
- Pass previous and next text to ElevenLabs for better audio quality ([#41](https://github.com/transformrs/trv/pull/41)).

### Fixed

- Fixed audio cache misses in some cases ([#45](https://github.com/transformrs/trv/pull/45)).

## [0.6.0] - 2025-03-16

### Added

- Support ElevenLabs text to speech ([#40](https://github.com/transformrs/trv/pull/40)).

### Changed

- Concat images and audio in single ffmpeg command ([#35](https://github.com/transformrs/trv/pull/35)).

### Fixed

- Add a pause between transitions ([#38](https://github.com/transformrs/trv/pull/38)).
- Trim the speaker notes for fewer random sounds like "uh"

## [0.5.0] - 2025-03-11

### Added

- Add a `--pre-typst` flag to run arbitrary commands before Typst ([#30](https://github.com/transformrs/trv/pull/30)).

### Changed

- Show only image and audio (instead of video) during `watch` ([#32](https://github.com/transformrs/trv/pull/32)).
- Reduce binary size by changing some compiler settings.

### Fixed

- Video order being wrong when having more than 10 slides.
- `watch` watches full directory instead of just the Typst file ([#30](https://github.com/transformrs/trv/pull/30)).
- Avoid copying Typst input file to fix includes and images ([#31](https://github.com/transformrs/trv/pull/31)).

## [0.4.1] - 2025-03-08

### Fixed

- Fix video and audio going out of sync ([#25](https://github.com/transformrs/trv/pull/25)).

## [0.4.0] - 2025-03-06

### Changed

- Remove the `--release` flag as it is now based on `build` or `watch` ([#22](https://github.com/transformrs/trv/pull/22)).
- Change the command to `trv build` and `trv watch` ([#21](https://github.com/transformrs/trv/pull/21)).
- Specify audio config in file instead of command line ([#20](https://github.com/transformrs/trv/pull/20)).

## [0.3.1] - 2025-03-05

### Added

- Cache video generation ([#19](https://github.com/transformrs/trv/pull/19)).

## [0.3.0] - 2025-02-27

### Added

- Support Google Text to Speech.
- Add multiple args to configure the voice including `speed`, and `audio_codec`.
- Support HD portrait videos.
- Support Zyphra Zonos model.

## [0.2.0] - 2025-02-22

### Added

- Support OpenAI Compatible text to speech ([#3](https://github.com/transformrs/trv/pull/3)).

## [0.1.0] - 2025-02-22

Initial release.

[0.7.0]: https://github.com/transformrs/trv/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/transformrs/trv/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/transformrs/trv/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/transformrs/trv/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/transformrs/trv/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/transformrs/trv/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/transformrs/trv/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/transformrs/trv/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/transformrs/trv/releases/tag/v0.1.0
