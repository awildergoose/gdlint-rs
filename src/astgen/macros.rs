#[macro_export]
macro_rules! wrap_err {
    ($v:expr, $($text:expr),+) => {
        $v.map_err(|_| anyhow::anyhow!($($text),+))
    };
}

#[macro_export]
macro_rules! str_of_node {
    ($self:expr, $node:expr) => {
        &$self.source[$node.byte_range()]
    };
}

#[macro_export]
macro_rules! string_of_node {
    ($self:expr, $node:expr) => {
        $self.source[$node.byte_range()].to_string()
    };
}

#[macro_export]
macro_rules! str_of_node_res {
    ($self:expr, $node:expr) => {
        &$self.source[wrap_err!($node, "failed to get node var")?.byte_range()]
    };
}

#[macro_export]
macro_rules! string_of_node_res {
    ($self:expr, $node:expr) => {
        $self.source[wrap_err!($node, "failed to get node var")?.byte_range()].to_string()
    };
}
