# beau\_collector

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
