use crate::queue::Queue;

#[derive(Clone, Debug)]
pub struct WeightedAdjacencyMatrix {
    pub matrix: Vec<Vec<Option<usize>>>,
}

pub fn bfs(graph: WeightedAdjacencyMatrix, source: usize, needle: usize) -> Vec<usize> {
    if needle >= graph.matrix.len() {
        return vec![];
    }

    let mut prev = vec![None; graph.matrix.len()];
    let mut seen = vec![false; graph.matrix.len()];
    seen[source] = true;
    let mut queue = Queue::new();
    queue.enqueue(source);

    while queue.lenght() > 0 {
        let curr = queue.deque().unwrap();

        if curr == needle {
            break;
        }

        for i in 0..graph.matrix[curr].len() {
            if let Some(_) = graph.matrix[curr][i] {
                if !seen[i] {
                    seen[i] = true;
                    prev[i] = Some(curr);
                    queue.enqueue(i);
                }
            }
        }
    }

    if matches!(prev[needle], None) {
        return vec![];
    }

    let mut curr = needle;
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
    use super::*;

    #[test]
    fn test_bfs() {
        let graph = WeightedAdjacencyMatrix {
            matrix: vec![
                vec![None, Some(1), Some(2), None, None, None],
                vec![Some(1), None, None, Some(3), None, None],
                vec![Some(2), None, None, Some(4), Some(5), None],
                vec![None, Some(3), Some(4), None, Some(6), Some(7)],
                vec![None, None, Some(5), Some(6), None, Some(8)],
                vec![None, None, None, Some(7), Some(8), None],
            ],
        };
        assert_eq!(bfs(graph.clone(), 0, 4), vec![0, 2, 4]);
        assert_eq!(bfs(graph, 0, 9), vec![]);

        let graph = WeightedAdjacencyMatrix {
            matrix: vec![
                vec![None, Some(3), Some(1), None, None, None, None],
                vec![None, None, None, None, Some(1), None, None],
                vec![None, None, Some(7), None, None, None, None],
                vec![None, None, None, None, None, None, None],
                vec![None, Some(1), None, Some(5), None, Some(2), None],
                vec![None, None, Some(18), None, None, None, Some(1)],
                vec![None, None, None, Some(1), None, None, Some(1)],
            ],
        };

        assert_eq!(bfs(graph.clone(), 0, 6), vec![0, 1, 4, 5, 6]);
        assert_eq!(bfs(graph, 6, 0), vec![])
    }
}
