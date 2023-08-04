#[derive(Clone, Debug)]
pub struct GraphEdge {
    pub to: usize,
    pub weight: usize,
}

#[derive(Clone, Debug)]
pub struct WeightedAdjacencyList {
    pub list: Vec<Vec<GraphEdge>>,
}

fn walk(
    graph: &WeightedAdjacencyList,
    curr: usize,
    needle: usize,
    seen: &mut Vec<bool>,
    path: &mut Vec<usize>,
) -> bool {
    if seen[curr] {
        return false;
    }

    seen[curr] = true;

    // recurse
    // pre
    path.push(curr);
    if curr == needle {
        return true;
    }

    // recurse
    for edge in graph.list[curr].iter() {
        if walk(graph, edge.to, needle, seen, path) {
            return true;
        }
    }

    // post
    path.pop();

    false
}

pub fn dfs(graph: WeightedAdjacencyList, source: usize, needle: usize) -> Vec<usize> {
    let mut seen = vec![false; graph.list.len()];
    let mut path = vec![];

    walk(&graph, source, needle, &mut seen, &mut path);

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tets_dfs() {
        //     >(1)<--->(4) ---->(5)
        //    /          |       /|
        // (0)     ------|------- |
        //    \   v      v        v
        //     >(2) --> (3) <----(6)
        let list = WeightedAdjacencyList {
            list: vec![
                vec![
                    GraphEdge { to: 1, weight: 3 },
                    GraphEdge { to: 2, weight: 1 },
                ],
                vec![GraphEdge { to: 4, weight: 1 }],
                vec![GraphEdge { to: 3, weight: 7 }],
                vec![],
                vec![
                    GraphEdge { to: 1, weight: 1 },
                    GraphEdge { to: 3, weight: 5 },
                    GraphEdge { to: 5, weight: 2 },
                ],
                vec![
                    GraphEdge { to: 2, weight: 18 },
                    GraphEdge { to: 6, weight: 1 },
                ],
                vec![GraphEdge { to: 3, weight: 1 }],
            ],
        };

        assert_eq!(dfs(list.clone(), 0, 6), vec![0, 1, 4, 5, 6]);
        assert_eq!(dfs(list, 6, 0), vec![]);
    }
}
