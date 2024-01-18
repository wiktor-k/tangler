// SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use std::io::{BufRead, BufReader, Read, Result, Write};

pub fn extract(
    source: impl Read,
    mut sink: impl Write,
    expected: &[impl AsRef<str>],
) -> Result<()> {
    let mut outputting = false;
    for line in BufReader::new(source).lines() {
        let line = line?;
        if line == "```" && outputting {
            sink.write_all(b"\n")?;
            outputting = false;
        } else if line.starts_with("```") && expected.iter().any(|info| info.as_ref() == &line[3..])
        {
            outputting = true;
        } else if outputting {
            sink.write_all(line.as_bytes())?;
            sink.write_all(b"\n")?;
        }
    }

    Ok(())
}
