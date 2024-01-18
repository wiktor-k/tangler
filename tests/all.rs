// SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fs::read_to_string;
use std::path::PathBuf;

use rstest::rstest;
use tangler::extract;
use testresult::TestResult;

#[rstest]
fn main(
    #[files("fixtures/*")]
    #[exclude("md")]
    mut path: PathBuf,
) -> TestResult {
    let extension = path.extension().expect("test file to have an extension");
    let extension: String = extension.to_string_lossy().into();
    let output = read_to_string(&path)?;
    // construct input file name
    path.set_extension("md");
    let input = read_to_string(&path)?;
    let mut actual_output = vec![];
    extract(input.as_bytes(), &mut actual_output, &[extension])?;
    assert_eq!(
        output,
        String::from_utf8(actual_output)?,
        "Actual output should be the same as expected output"
    );

    Ok(())
}
