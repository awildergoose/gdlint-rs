use std::{fmt::Debug, sync::Arc};

pub type GDNodeChildren = Vec<Arc<GDNode>>;

#[derive(Debug, Clone)]
pub struct GDNode {
    pub children: GDNodeChildren,
    pub kind: GDNodeKind,
}

#[derive(Debug, Clone)]
pub enum GDNodeKind {
    Document {
        extends: Option<String>,
        class_name: Option<String>,
    },
    VariableDeclaration {
        name: String,
        // setget
        is_static: bool,
        var_type: Option<String>,
        value: Option<String>,
    },
}

impl GDNode {
    #[must_use]
    pub const fn new(kind: GDNodeKind, children: GDNodeChildren) -> Self {
        Self { children, kind }
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(Arc::new(child));
    }
}
