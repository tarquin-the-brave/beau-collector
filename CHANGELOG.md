# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Don't throw away error cause chain when formatting the errors into one error.
  Use anyhow's [inline representation][inlinerepr] instead.  Error message where some collected
  errors had a chain of causes will now look like:

```
first error without causes
there was an error processing x: something went wrong: second error
```

Where causes are separated by colons.

[inlinerepr]: https://docs.rs/anyhow/1.0.28/anyhow/struct.Error.html#display-representations

## [0.2.0] - 2020-05-04
### Changed
- Remove requirement for `T: std::fmt::Debug`, where `T` is the type parameter
  to the `BeasuCollector` trait.
- Move the type parameter for the type to be collected into `I`,
  `where I: std::iter::FromIterator<T>`, to be a type parameter of function
  `bcollect` rather than of trait `BeauCollector`.

This enables you to add a turbo fish to `bcollect` like so:

```rust
use beau_collector::BeauCollector as _;
let y = x.iter().map(f).bcollect::<HashMap<_, _>>();
```

where previously you may have had:

```rust
use beau_collector::BeauCollector as _;
let y: Result<HashMap<_, _> = x.iter().map(f).bcollect();
```

**BREAKING** - this does however break fully qualified syntax as a type parameter has move.

What was:

```rust
let y = beau_collector::BeauCollector::<Vec<String>, String>::bcollect(x.iter().map(f));
```

will need to change to:

```rust
let y = beau_collector::BeauCollector::<String>::bcollect::<Vec<String>>(x.iter().map(f));
```

which also makes it pointless to pass the type parameter to `BeauCollector`:

```rust
let y = beau_collector::BeauCollector::bcollect::<Vec<String>>(x.iter().map(f));
```

## [0.1.1] - 2020-05-03
### Added
- Add changelog

## [0.1.0] - 2020-05-03
### Added
- Initial release - [Documentation](https://docs.rs/beau_collector/0.1.0/beau_collector/).

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/0.2.0...HEAD
[0.2.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/0.1.0


