# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added a `NOTICE` file with project attribution for downstream distributions.

### Fixed
- `depth_factor` of a screen-anchored object follows the table of ETSI TS 103 420 (0.25, 0.5, 1.0, 2.0) instead of 0.25 times the index plus one
- `object_gain_idx` 3 takes the previous object's gain as the block ended up with it, so it is -inf after an inactive object and no longer a stale gain from an earlier one
- The gain is bit 1 and the priority bit 0 of `obj_basic_info_bits`, not the other way round, so a partial update of one no longer reads the other

## [0.1.0] - 2026-09-17

### Added
- `ObjectAudioMetadataPayload::read`, parsing an OAMD payload from the bytes its carrier hands over, with the object, trim, extended object, headphone and object description elements
- `BedObjectElement` and `DownmixRoute`, the element with `oa_element_id` 6, which says how a bed folds down to fewer outputs
