use std::{fmt::Debug, sync::Arc};

// Base

pub type GDNodeChildren = Vec<Arc<GDNode>>;

#[derive(Debug)]
pub struct GDNode {
    pub children: GDNodeChildren,
    pub kind: GDNodeKind,
}

#[derive(Debug)]
pub enum GDNodeKind {
    Document {
        extends: Option<String>,
        class_name: Option<String>,
    },
    VariableDeclaration {},
}

impl GDNode {
    #[must_use]
    pub const fn new(kind: GDNodeKind, children: GDNodeChildren) -> Self {
        Self { children, kind }
    }
}
