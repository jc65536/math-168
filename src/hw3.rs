use ndarray::{array, Array2};
use petgraph::{dot::Config, graph::UnGraph};
use rand::random;

use crate::util::{EdgeIter, OutputGraph};

fn clustering_coef(a: &Array2<f64>) -> f64 {
    let squared = a.dot(a);
    let two_paths = squared.sum() - squared.diag().sum();
    let cubed = &squared.dot(a);
    let triangles6 = cubed.diag().sum();
    if two_paths == 0. {
        0.
    } else {
        triangles6 / two_paths
    }
}

pub fn ex7() {
    let mut a = array![
        [0., 1., 1., 1., 1.],
        [1., 0., 1., 0., 1.],
        [1., 1., 0., 1., 0.],
        [1., 0., 1., 2., 1.],
        [1., 1., 0., 1., 0.]
    ];

    a.diag_mut().iter_mut().for_each(|n| *n = 0.);

    let clustering = clustering_coef(&a);

    println!("Global clustering coefficient: {clustering}");

    let a = a.map(|&f| f as usize);
    let g: UnGraph<(), ()> = UnGraph::from_edges(EdgeIter::from(&a));

    g.output_png(&[Config::EdgeNoLabel, Config::NodeNoLabel], "ex7.png");
}

fn random_er_net(n: usize, p: f64) -> Array2<f64> {
    let mut a = Array2::zeros((n, n));
    for i in 0..n {
        for j in (i + 1)..n {
            if random::<f64>() < p {
                a[[i, j]] = 1.;
                a[[j, i]] = 1.;
            }
        }
    }
    a
}

pub fn ex8() {
    println!("n C p");
    for p in [0.05, 0.1, 0.2] {
        for n in (5..=40).step_by(5) {
            let a = random_er_net(n, p);
            let c = clustering_coef(&a);
            println!("{n} {c} {p}");
        }
    }
}
