<!--
SPDX-FileCopyrightText: 2021-2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Test fixtures

This directory contains files that are used for testing `tangler`.

Each code snippet is extracted and compared with a known-good output:

```sh
OUTPUT=$(echo test)

[ "$OUTPUT" = "test" ]
```
