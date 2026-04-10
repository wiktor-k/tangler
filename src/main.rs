// SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use tangler::{extract, Options};

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut options = Options::default();
    if !args.is_empty() && args[0] == "--preserve-newlines" {
        options = options.set_preserve_newlines(true);
        args.remove(0);
    }
    extract(std::io::stdin(), std::io::stdout(), &args, &options)?;
    Ok(())
}
