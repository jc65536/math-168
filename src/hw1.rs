use ndarray::array;
use ndarray_linalg::Eig;
use petgraph::{
    dot::{Config, Dot},
    graph::UnGraph,
};

pub fn exercise7() {
    let a = array![
        [0., 1., 1., 2., 1.],
        [1., 0., 1., 0., 1.],
        [1., 1., 0., 1., 0.],
        [2., 0., 1., 0., 1.],
        [1., 1., 0., 1., 0.],
    ];

    let (eigvals, _) = a.eig().unwrap();
    eigvals.iter().for_each(|e| println!("{e}"));

    let g: UnGraph<(), ()> = UnGraph::from_edges(&[
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 3),
        (0, 4),
        (1, 2),
        (1, 4),
        (2, 3),
        (3, 4),
    ]);

    let dot = Dot::with_config(&g, &[Config::EdgeNoLabel, Config::NodeNoLabel]);

    println!("{:?}", dot);
}
