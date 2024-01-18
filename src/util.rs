use std::{
    fmt::Debug,
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

use petgraph::{
    dot::{Config, Dot},
    visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable},
};

pub fn output_graph<'a, G>(graph: G, config: &'a [Config], filename: &str)
where
    G: IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: Debug,
    G::NodeWeight: Debug,
{
    let dot = Dot::with_config(graph, config);

    let file = File::create(format!("output/{filename}")).unwrap();

    let proc = Command::new(if graph.is_directed() { "dot" } else { "neato" })
        .arg("-Tpng")
        .stdin(Stdio::piped())
        .stdout(file)
        .spawn()
        .unwrap();

    write!(proc.stdin.unwrap(), "{dot:?}").unwrap();
}
