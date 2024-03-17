use std::collections::HashMap;

use ndarray::prelude::*;
use rand::random;

const n: usize = 100;

fn gen_net(p: f64) -> Array2<u32> {
    let mut mat: Array2<u32> = ndarray::Array2::zeros((n, n));

    let f = |x: i32| {
        if x < 0 {
            (x + n as i32) as usize
        } else if x >= n as i32 {
            (x - n as i32) as usize
        } else {
            x as usize
        }
    };

    let mut edges = Vec::new();

    for i in 0..n {
        for d in [-2, -1, 1, 2] {
            let j = f(i as i32 + d);
            edges.push((i, j));
        }
    }

    let rand_int = || random::<usize>() % n;

    let edges = edges.into_iter().map(|(i, j)| {
        if p > random() {
            let i = rand_int();
            let j = rand_int();
            (i, j)
        } else {
            (i, j)
        }
    });

    for (i, j) in edges {
        mat[[i, j]] += 1;
        mat[[j, i]] += 1;
    }

    mat
}

fn path(hist: &mut HashMap<(usize, usize), usize>, mat: &Array2<u32>, i: usize, j: usize) -> usize {
    if i == j {
        0
    } else if let Some(len) = hist.get(&(i, j)) {
        *len
    } else {
        let mut min = 0;
        for k in 0..n {
            if mat[[i, k]] > 0 {
                let len = path(hist, mat, k, j);
                if len < min {
                    min = len;
                }
            }
        }
        hist.insert((i, j), min);
        min
    }
}

fn diameter(mut mat: Array2<u32>) -> usize {
    let mut max = 0;
    let mut hist = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            let len = path(&mut hist, &mut mat, i, j);
            if len > max {
                max = len;
            }
        }
    }
    max
}

pub fn ex6() {
    for p in [0., 0.001, 0.01, 0.1] {
        for x in 0..30 {
            let mat = gen_net(p);
            let d = diameter(mat);
            println!("{p} {x} {d}");
        }
    }
}
