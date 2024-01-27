use ndarray::{array, Array2, Axis};
use ndarray_linalg::{Eig, Scalar};
use petgraph::{dot::Config, graph::UnGraph};

use crate::util::{EdgeIter, OutputGraph};

fn interpolate(t: f64, p1: (f64, f64, f64), p2: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        t * p1.0 + (1. - t) * p2.0,
        t * p1.1 + (1. - t) * p2.1,
        t * p1.2 + (1. - t) * p2.2,
    )
}

fn interpolate_color(min: f64, val: f64, max: f64) -> (u32, u32, u32) {
    let t = (val - min) / (max - min);
    let p0 = (128., 0., 255.);
    let p1 = (0., 0., 255.);
    let p2 = (0., 255., 255.);
    let p3 = (0., 255., 0.);
    let p4 = (255., 255., 0.);
    let t0 = interpolate(t, p0, p1);
    let t1 = interpolate(t, p1, p2);
    let t2 = interpolate(t, p2, p3);
    let t3 = interpolate(t, p3, p4);
    let u0 = interpolate(t, t0, t1);
    let u1 = interpolate(t, t1, t2);
    let u2 = interpolate(t, t2, t3);
    let v0 = interpolate(t, u0, u1);
    let v1 = interpolate(t, u1, u2);
    let (r, g, b) = interpolate(t, v0, v1);
    (r as u32, g as u32, b as u32)
}

fn calc_centralities<'a>(centralities: impl Iterator<Item = &'a f64> + Clone) {
    let &cmin = centralities.clone().min_by(|x, y| x.total_cmp(y)).unwrap();
    let &cmax = centralities.clone().max_by(|x, y| x.total_cmp(y)).unwrap();
    let colors = centralities
        .clone()
        .map(|&c| interpolate_color(cmin, c, cmax));

    colors.clone().enumerate().for_each(|(i, (r, g, b))| {
        let x = format!("{r:02x}{g:02x}{b:02x}");
        println!("{i} [ color=\"#{x}\", fillcolor=\"#{x}2a\" ]");
    });

    let mut centcolors: Vec<(&f64, (u32, u32, u32))> = centralities.clone().zip(colors).collect();

    centcolors.sort_by(|(c1, _), (c2, _)| c1.total_cmp(c2));

    centcolors.iter().for_each(|(c, (r, g, b))| {
        let x = format!("{r:02x}{g:02x}{b:02x}");
        println!("\\colorbox[HTML]{{{x}}}{{\\color[HTML]{{{x}}} 000}} {c} \\\\");
    });
}

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

    let (eigvals, eigvecs) = a.eig().unwrap();

    let (_, eigvec) = eigvals
        .iter()
        .zip(eigvecs.axis_iter(Axis(1)))
        .max_by(|(&e1, _), (&e2, _)| e1.re().total_cmp(&e2.re()))
        .unwrap();

    calc_centralities(eigvec.map(|x| x.re()).iter());
    calc_centralities(a.map_axis(Axis(0), |r| r.sum()).iter());

    let a = a.map(|&f| f as usize);
    let g: UnGraph<(), ()> = UnGraph::from_edges(EdgeIter::from(&a));

    g.output_dot(&[Config::EdgeNoLabel, Config::NodeNoLabel], "ex6.dot");
}
