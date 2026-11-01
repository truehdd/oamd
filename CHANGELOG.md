# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added a `NOTICE` file with project attribution for downstream distributions.

### Fixed
- OAMD fields cannot read past their declared element boundary. Truncated elements are bounded by the absolute end of the available payload, so their remaining data is not mistaken for another element.
- `ObjectAudioMetadataPayload::read` rejects extended metadata whose block counts no longer match the final object element. This prevents `get_damf_pos` from panicking after an object element is replaced.

## [0.1.0] - 2026-09-17

### Added
- `ObjectAudioMetadataPayload::read`, parsing an OAMD payload from the bytes its carrier hands over, with the object, trim, extended object, headphone and object description elements
- `BedObjectElement` and `DownmixRoute`, the element with `oa_element_id` 6, which says how a bed folds down to fewer outputs
