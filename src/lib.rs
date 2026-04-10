// SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use std::io::{BufRead, BufReader, Read, Result, Write};

/// Extraction options.
///
/// # Examples
///
/// Enables [preserving newlines][`Options::set_preserve_newlines`] during export.
///
/// ```
/// # fn main() -> testresult::TestResult {
/// use tangler::Options;
///
/// let options = Options::default().set_preserve_newlines(true);
/// # Ok(()) }
/// ```
#[derive(Debug, Default)]
pub struct Options {
    preserve_newlines: bool,
}

impl Options {
    /// Enables or disables preserving newlines in the extracted output.
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
    /// use tangler::{extract, Options};
    ///
    /// let mut sink = Vec::new();
    ///
    /// extract(source.as_bytes(), &mut sink, &["sh"], &Options::default().set_preserve_newlines(true))?;
    ///
    /// assert_eq!(&b"\necho x\n\n"[..], &sink[..]);
    /// # Ok(()) }
    /// ```
    pub fn set_preserve_newlines(mut self, preserve_newlines: bool) -> Self {
        self.preserve_newlines = preserve_newlines;
        self
    }
}

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
/// tangler::extract(source.as_bytes(), &mut sink, &["sh"], &tangler::Options::default())?;
///
/// assert_eq!(&b"echo x\n\n"[..], &sink[..]);
/// # Ok(()) }
/// ```
pub fn extract(
    source: impl Read,
    mut sink: impl Write,
    expected: &[impl AsRef<str>],
    options: &Options,
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
            if options.preserve_newlines {
                sink.write_all(b"\n")?;
            }
        } else if outputting {
            sink.write_all(line.as_bytes())?;
            sink.write_all(b"\n")?;
        } else if options.preserve_newlines {
            sink.write_all(b"\n")?;
        }
    }

    Ok(())
}
