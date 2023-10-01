use std::collections::VecDeque;

pub struct Graph {
    edges: Vec<Vec<Option<i32>>>,
}

impl Graph {
    pub fn new(num_verteces: usize) -> Graph {
        let mut edges = Vec::new();
        for _ in 0..num_verteces {
            edges.push(vec![None; num_verteces])
        }

        Graph { edges }
    }

    pub fn add(&mut self, src: usize, tgt: usize, weight: i32) {
        self.edges[src][tgt] = Some(weight);
    }

    pub fn add_multiple(&mut self, src: usize, conn: &[(usize, i32)]) {
        for c in conn {
            self.edges[src][c.0] = Some(c.1)
        }
    }

    pub fn breadth_first_search(&self, src: usize, tgt: usize) -> Option<Vec<usize>> {
        let mut visited = vec![false; self.edges.len()];
        visited[src] = true;
        let mut prev = vec![None; self.edges.len()];
        let mut queue = vec![src];

        while !queue.is_empty() {
            let curr = queue.pop().expect("q is non-empty");
            if curr == tgt {
                break;
            }

            for (i, e) in self.edges[curr].iter().enumerate() {
                walk_edge(curr, i, e, &mut queue, &mut prev, &mut visited)
            }
        }

        build_path(tgt, prev)
    }
}

fn build_path(tgt: usize, prev: Vec<Option<usize>>) -> Option<Vec<usize>> {
    prev[tgt].map(|mut curr| {
        let mut path = VecDeque::from([curr, tgt]);
        while let Some(prev) = prev[curr] {
            path.push_front(prev);
            curr = prev;
        }
        path.into()
    })
}

fn walk_edge(
    curr: usize,
    i: usize,
    e: &Option<i32>,
    queue: &mut Vec<usize>,
    prev: &mut Vec<Option<usize>>,
    visited: &mut Vec<bool>,
) {
    if let Some(_) = e {
        if !visited[i] {
            queue.push(i);
            prev[i] = Some(curr);
            visited[i] = true;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_graph() -> Graph {
        let mut g = Graph::new(5);
        g.add_multiple(0, &[(1, 0), (2, 0)]);
        g.add_multiple(2, &[(3, 0), (4, 0)]);
        g.add(1, 3, 0);

        g
    }

    #[test]
    fn bfs() {
        let g = create_graph();

        let p = g.breadth_first_search(0, 4).unwrap();
        assert_eq!(p, vec![0, 2, 4]);
    }
}
