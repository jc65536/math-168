use std::{
    fmt::Debug,
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

use ndarray::Array2;
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

pub struct EdgeIter<'a>(Box<dyn Iterator<Item = (u32, u32)> + 'a>);

impl<'a> Iterator for EdgeIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, T: Into<usize> + Copy> From<&'a Array2<T>> for EdgeIter<'a> {
    fn from(adj_matrix: &'a Array2<T>) -> Self {
        Self(Box::new(
            adj_matrix
                .indexed_iter()
                .filter(|((i, j), _)| i <= j)
                .flat_map(|(edge, &x)| (0..x.into()).map(move |_| edge))
                .map(|(i, j)| (i as u32, j as u32)),
        ))
    }
}
