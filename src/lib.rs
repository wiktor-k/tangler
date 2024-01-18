// SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io::{Read, Result, Write};

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};

pub fn extract(mut source: impl Read, mut sink: impl Write, expected: &[String]) -> Result<()> {
    let mut text = String::new();
    source.read_to_string(&mut text)?;

    let arena = Arena::new();

    let root = parse_document(&arena, &text, &ComrakOptions::default());

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F)
    where
        F: FnMut(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &mut |node| {
        if let &mut NodeValue::CodeBlock(ref block) = &mut node.data.borrow_mut().value {
            let info = String::from_utf8_lossy(&block.info);
            let code = String::from_utf8_lossy(&block.literal);
            if expected.iter().any(|s| *s == info) {
                writeln!(sink, "{}", code).expect("write to work");
            }
        }
    });

    Ok(())
}
