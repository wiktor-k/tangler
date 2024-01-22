// SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use tangler::extract;

fn main() -> std::io::Result<()> {
    extract(
        std::io::stdin(),
        std::io::stdout(),
        &std::env::args().skip(1).collect::<Vec<_>>(),
    )?;
    Ok(())
}
