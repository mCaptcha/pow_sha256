# Changelog

## 0.2.1

### Changed
- performance improvement: internally `get_difficulty` was being called
  multiple times while generating `PoW`, now it calls only once.

### Fixed
- encoding and decoding configuration used internally was causing an
  error in `PoW` computation

## 0.2.0

### Changed
- Difficulty factor is now an unsigned 32 bit number

## 0.1

### Added:
- PoW constructor
- unique salt
