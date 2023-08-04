use crate::adjacency_list::WeightedAdjacencyList;

fn has_unvisited(seen: &mut Vec<bool>, dists: &mut Vec<usize>) -> bool {
    for (i, s) in seen.iter().enumerate() {
        if !s && dists[i] < usize::MAX {
            return true;
        }
    }

    false
}

fn get_lowest_unvisited(seen: &mut Vec<bool>, dists: &mut Vec<usize>) -> Option<usize> {
    let mut lowest_distance = usize::MAX;
    let mut lowest_index = None;

    for i in 0..seen.len() {
        if seen[i] {
            continue;
        }

        if lowest_distance > dists[i] {
            lowest_distance = dists[i];
            lowest_index = Some(i);
        }
    }

    lowest_index
}

pub fn dijkstra_list(
    source: usize,
    destination: usize,
    graph: WeightedAdjacencyList,
) -> Vec<usize> {
    if destination >= graph.list.len() {
        return vec![];
    }

    let mut seen = vec![false; graph.list.len()];
    let mut prev = vec![None; graph.list.len()];
    let mut dists = vec![usize::MAX; graph.list.len()];

    dists[source] = 0;

    while has_unvisited(&mut seen, &mut dists) {
        let curr = get_lowest_unvisited(&mut seen, &mut dists).unwrap();

        seen[curr] = true;

        for edge in graph.list[curr].iter() {
            if seen[edge.to] {
                continue;
            }

            let new_dist = dists[curr] + edge.weight;
            if new_dist < dists[edge.to] {
                dists[edge.to] = new_dist;
                prev[edge.to] = Some(curr);
            }
        }
    }

    if matches!(prev[destination], None) {
        return vec![];
    }

    let mut curr = destination;
    let mut out = vec![];

    while !matches!(prev[curr], None) {
        out.push(curr);
        curr = prev[curr].unwrap();
    }

    out.extend(vec![source]);
    out.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::adjacency_list::GraphEdge;

    use super::*;

    #[test]
    fn dijkstra_list_test() {
        //      (1) --- (4) ---- (5)
        //    /  |       |       /|
        // (0)   | ------|------- |
        //    \  |/      |        |
        //      (2) --- (3) ---- (6)
        let list = WeightedAdjacencyList {
            list: vec![
                vec![
                    GraphEdge { to: 1, weight: 3 },
                    GraphEdge { to: 2, weight: 1 },
                ],
                vec![
                    GraphEdge { to: 0, weight: 3 },
                    GraphEdge { to: 2, weight: 4 },
                    GraphEdge { to: 4, weight: 1 },
                ],
                vec![
                    GraphEdge { to: 1, weight: 4 },
                    GraphEdge { to: 3, weight: 7 },
                    GraphEdge { to: 0, weight: 1 },
                ],
                vec![
                    GraphEdge { to: 2, weight: 7 },
                    GraphEdge { to: 4, weight: 5 },
                    GraphEdge { to: 6, weight: 1 },
                ],
                vec![
                    GraphEdge { to: 1, weight: 1 },
                    GraphEdge { to: 3, weight: 5 },
                    GraphEdge { to: 5, weight: 2 },
                ],
                vec![
                    GraphEdge { to: 6, weight: 1 },
                    GraphEdge { to: 4, weight: 2 },
                    GraphEdge { to: 2, weight: 18 },
                ],
                vec![
                    GraphEdge { to: 3, weight: 1 },
                    GraphEdge { to: 5, weight: 1 },
                ],
            ],
        };

        assert_eq!(dijkstra_list(0, 6, list), vec![0, 1, 4, 5, 6])
    }
}
