use std::{cell::RefCell, rc::Rc};

use crate::{
    astgen::{
        ast::{GDNode, GDNodeKind},
        traverser::Traverser,
    },
    wrap_err,
};

pub fn parse_code(code: String) -> anyhow::Result<GDNode> {
    let mut parser = type_sitter::Parser::<crate::astgen::gdscript::Source>::new(
        &tree_sitter_gdscript::LANGUAGE.into(),
    )?;

    let root = Rc::new(RefCell::new(GDNode::new(
        GDNodeKind::Document {
            extends: None,
            class_name: None,
        },
        vec![],
    )));
    let mut traverser = Traverser {
        source: code.clone(),
        root: root.clone(),
        scope: root.clone(),
    };
    let tree = wrap_err!(parser.parse(code, None), "Failed to parse with tree-sitter")?;
    if tree.root_node().is_err() {
        anyhow::bail!("Root node is Err: {:#?}", tree.root_node())
    }

    let tree = Box::leak(Box::new(tree));
    let root_node = tree.root_node()?;
    traverser.traverse(&root_node)?;

    Ok(root.borrow().clone())
}
