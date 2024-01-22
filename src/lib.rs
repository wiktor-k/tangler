// SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use std::io::{BufRead, BufReader, Read, Result, Write};

/// Extracts expected code-blocks from Markdown source and writes them into sink.
///
/// Multiple expected code-block infos can be passed. In this case it's
/// sufficient for any one of them to match for the code-block to be printed.
///
/// This function will not lint the source and assumes that the input is a valid
/// Markdown document.
///
/// # Examples
///
/// ```
/// use std::io::Write;
///
/// # fn main() -> testresult::TestResult {
/// let source = r#"```sh
/// echo x
/// ```"#;
/// let mut sink = Vec::new();
///
/// tangler::extract(source.as_bytes(), &mut sink, &["sh"])?;
///
/// assert_eq!(&b"echo x\n\n"[..], &sink[..]);
/// # Ok(()) }
/// ```
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
