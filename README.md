# spacetime-call-i

> [SpacetimeDB 2.0's spacetime.json](https://github.com/clockworklabs/SpacetimeDB/releases#:~:text=spacetime.json%20configuration) now allows you to declaratively configure the database. [`spacetime call`](https://spacetimedb.com/docs/cli-reference/#spacetime-call) uses the configuration by default and `spacetime-call-i` is feature-complete, therefor there will be no new releases and the repository is archived.

The `spacetime-call-i` utility is a tool that allows you to make `spacetime call`s without having to enter the server and database each time.

## Usage

`cargo run --`

```txt
Usage:
       cargo run -- [OPTIONS] --database <database>
OR
       spacetime-call-i.exe [OPTIONS] --database <database>

Options:
  -s, --server <server>      The name of the server which hosts the database. [default: local]
  -d, --database <database>  The name or identity of the database.
  -h, --help                 Print help
  -V, --version              Print version
```

Internal commands:

- `clear|clean|cls`
- `quit|exit`

Specifying any other text as first argument will be interpreted as reducer name, anything else will be interpreted as the arguments of the reducer.

## Licensing

spacetime-call-i is dual-licensed under both the [MIT License](https://choosealicense.com/licenses/mit/) and the [Apache License (Version 2.0)](https://choosealicense.com/licenses/apache-2.0/) and therefore Open Source.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
