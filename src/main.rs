use std::io::Read;

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text)?;

    let arena = Arena::new();

    let root = parse_document(&arena, &text, &ComrakOptions::default());

    let expected_infos: Vec<_> = std::env::args().skip(1).collect();

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where
        F: Fn(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &|node| {
        if let &mut NodeValue::CodeBlock(ref block) = &mut node.data.borrow_mut().value {
            let info = String::from_utf8_lossy(&block.info);
            let code = String::from_utf8_lossy(&block.literal);
            if expected_infos.iter().any(|s| s == &*info) {
                println!("{}", code);
            }
        }
    });

    Ok(())
}
