use ndarray::array;
use ndarray_linalg::Eig;
use petgraph::{dot::Config, graph::UnGraph};

use crate::util::{EdgeIter, OutputGraph};

pub fn ex7() {
    let a = array![
        [0., 1., 1., 2., 1.],
        [1., 0., 1., 0., 1.],
        [1., 1., 0., 1., 0.],
        [2., 0., 1., 0., 1.],
        [1., 1., 0., 1., 0.],
    ];

    let (eigvals, _) = a.eig().unwrap();
    eigvals.iter().for_each(|e| println!("{e}"));

    let a = a.map(|&f| f as usize);
    let g: UnGraph<(), ()> = UnGraph::from_edges(EdgeIter::from(&a));

    g.output_png(&[Config::EdgeNoLabel, Config::NodeNoLabel], "ex7.png");
}
