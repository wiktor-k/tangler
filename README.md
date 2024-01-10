<!--
SPDX-FileCopyrightText: 2021-2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Tangler

[![CI](https://github.com/wiktor-k/tangler/actions/workflows/ci.yml/badge.svg)](https://github.com/wiktor-k/tangler/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tangler)](https://crates.io/crates/tangler)

Extracts (tangles) code fragments from Markdown documents.

This can be used to test examples in README documents to make sure they are always up to date.

The most common use of this crate is installing it as part of the CI script, extracting selected code fragments and then executing them.

```bash
cargo install tangler
tangler bash < README.md | bash -euxo pipefail -
```

The first argument is a selector of the blocks that should be extracted. The tool takes the input markdown document from stdin and outputs only matching blocks to stdout.

## License

This project is licensed under either of:

  - [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0),
  - [MIT license](https://opensource.org/licenses/MIT).

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
