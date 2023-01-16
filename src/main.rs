use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let text = std::fs::read_to_string("/home/wiktor/Downloads/readme.md")?;
    let text = std::io::stdin().lines().collect::<Result<String, _>>()?;
    let arena = Arena::new();
    println!("..{}..", text.len());
    let root = parse_document(&arena, &text, &ComrakOptions::default());

    let mut expected_infos: Vec<_> = std::env::args().collect();
    expected_infos.pop();

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where
        F: Fn(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        &mut NodeValue::CodeBlock(ref block) => {
            let info = String::from_utf8_lossy(&block.info);
            let code = String::from_utf8_lossy(&block.literal);
            println!("found {} with content {}.", info, code);
        }
        _ => (),
    });

    Ok(())
}
