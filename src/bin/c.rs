use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use std::collections::VecDeque;
#[allow(unused_imports)]
use petgraph::graph::{Graph, UnGraph, node_index, NodeIndex};
#[allow(unused_imports)]
use petgraph::{Undirected, Directed};
#[allow(unused_imports)]
use petgraph::visit::{Dfs, Bfs, depth_first_search, Control, DfsEvent};

fn main() {
    // let source = AutoSource::from("6 5
    // 8 6 9 1 2 1
    // 1 3
    // 4 2
    // 4 3
    // 4 6
    // 4 6");
    input!{
        // from source,
        n: usize,
        m: usize,
        mut h: [usize; n],
        ab: [(usize, usize); m]
    };

    h.insert(0, 0);

    let graph = UnGraph::<(),(), usize>::from_edges(&ab);

    let mut v = vec![-1; n+1];
    v[0] = 0;

    for idx in 1..=n{
        if v[idx] != -1 {
            continue
        }
        let mut is_good = true;
        for node in graph.neighbors(node_index(idx)) {
            let next_idx = node.index();
            if h[idx]  <= h[next_idx] {
                is_good = false;
                break;
            }
        }
        if is_good{
            v[idx] = 1;
            for node in graph.neighbors(node_index(idx)) {
                let next_idx = node.index();
                v[next_idx] = 0;
            }
        }
        else {
            v[idx] = 0;
        }
    }

    let ans = v.iter().sum::<i32>();
    println!("{}", ans);
}
