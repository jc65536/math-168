use ndarray::{array, Array2};
use ndarray_linalg::Eig;
use petgraph::{dot::Config, graph::UnGraph};

use crate::util::{EdgeIter, OutputGraph};

pub fn ex6() {
    let a: Array2<f64> = array![
        [0., 1., 0., 1., 0., 1., 1., 1., 0., 1., 1., 1.],
        [1., 0., 0., 1., 1., 1., 1., 1., 0., 0., 0., 1.],
        [0., 0., 0., 1., 1., 0., 1., 0., 1., 0., 0., 0.],
        [1., 1., 1., 0., 0., 1., 0., 1., 0., 1., 1., 1.],
        [0., 1., 1., 0., 0., 0., 1., 1., 1., 0., 1., 0.],
        [1., 1., 0., 1., 0., 0., 0., 0., 0., 0., 1., 1.],
        [1., 1., 1., 0., 1., 0., 0., 0., 1., 0., 1., 0.],
        [1., 1., 0., 1., 1., 0., 0., 0., 0., 1., 0., 1.],
        [0., 0., 1., 0., 1., 0., 1., 0., 0., 1., 1., 0.],
        [1., 0., 0., 1., 0., 0., 0., 1., 1., 0., 1., 1.],
        [1., 0., 0., 1., 1., 1., 1., 0., 1., 1., 0., 1.],
        [1., 1., 0., 1., 0., 1., 0., 1., 0., 1., 1., 0.],
    ];

    let (eigvals, _) = a.eig().unwrap();
    eigvals.iter().for_each(|e| println!("{e}"));

    let a = a.map(|&f| f as usize);
    let g: UnGraph<(), ()> = UnGraph::from_edges(EdgeIter::from(&a));

    g.output(&[Config::EdgeNoLabel, Config::NodeNoLabel], "ex6.png");
}
