use std::{
    fmt::Debug,
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

use ndarray::{Array2, ArrayBase, Array};
use petgraph::{
    dot::{Config, Dot},
    visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable},
};

pub trait OutputGraph {
    fn output<'a>(self, config: &'a [Config], filename: &str);
}

impl<G> OutputGraph for G
where
    G: IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: Debug,
    G::NodeWeight: Debug,
{
    fn output<'a>(self, config: &'a [Config], filename: &str) {
        let dot = Dot::with_config(self, config);

        let file = File::create(format!("output/{filename}")).unwrap();

        let proc = Command::new(if self.is_directed() { "dot" } else { "neato" })
            .arg("-Tpng")
            .stdin(Stdio::piped())
            .stdout(file)
            .spawn()
            .unwrap();

        write!(proc.stdin.unwrap(), "{dot:?}").unwrap();
    }
}

struct EdgeList<'a, T>(&'a [(T, T)]);

impl<'a, T> From<Array2<T>> for EdgeList<'a, T>
{
    fn from(value: Array2<T>) -> Self {
        value.indexed_iter().flat_map(|((i, j), x)| (0..x));
    }
}
