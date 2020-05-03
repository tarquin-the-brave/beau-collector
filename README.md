# beau\_collector

[![Build Status](https://api.travis-ci.org/tarquin-the-brave/beau-collector.svg?branch=master)][travis]
[![Latest Version](https://img.shields.io/crates/v/beau_collector.svg)][crates.io]
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)][docs.rs]

[travis]: https://travis-ci.org/tarquin-the-brave/beau-collector
[crates.io]: https://crates.io/crates/beau_collector
[docs.rs]: https://docs.rs/beau_collector

Collect _all_ the errors from an iterator of `Result`s into a single `Error`.

The default behaviour of running `collect::<Result<_, _>>()` on an iterator of
`Result`s which have some `Err` variants in is to collect to the first `Err` found
and throw away the other errors.

`beau_collector` is a simple solution for when you want to collect _all_ the errors from
an iterator of `Result`s and are happy for them to be put into a single simple `Error`,
where each error is on a newline within the string representation of that error.

A use case for this could be in a CLI tool where you want to collect all the errors and
print them to `stderr` for the user to see them all at once.

## Contributing

Please do.  See [issues][issues].  Pick one to fix, or raise a new one.

[issues]: https://github.com/tarquin-the-brave/beau-collector/issues
