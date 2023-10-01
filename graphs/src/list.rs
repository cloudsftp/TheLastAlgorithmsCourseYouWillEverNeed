use anyhow::{anyhow, Error};

pub struct Edge {
    weight: i32,
    tgt: usize,
}

pub struct Graph {
    num_verteces: usize,
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(num_verteces: usize) -> Self {
        let mut edges = Vec::with_capacity(num_verteces);
        for _ in 0..num_verteces {
            edges.push(Vec::new())
        }

        Self {
            num_verteces,
            edges,
        }
    }

    pub fn add(&mut self, src: usize, tgt: usize, weight: i32) {
        self.edges[src].push(Edge { weight, tgt })
    }

    pub fn add_multiple(&mut self, src: usize, conn: &[(usize, i32)]) {
        let edges = &mut self.edges[src];

        for c in conn {
            edges.push(Edge {
                weight: c.1,
                tgt: c.0,
            })
        }
    }

    fn dfs(&self, curr: usize, tgt: usize, path: &mut Vec<usize>, visited: &mut Vec<bool>) -> bool {
        path.push(curr);

        if curr == tgt {
            return true;
        } else if visited[curr] {
            path.pop();
            return false;
        }

        visited[curr] = true;
        for e in &self.edges[curr] {
            if self.dfs(e.tgt, tgt, path, visited) {
                return true;
            }
        }
        path.pop();
        false
    }

    pub fn depth_first_search(&self, src: usize, tgt: usize) -> Option<Vec<usize>> {
        let mut visited = vec![false; self.num_verteces];
        let mut path = Vec::new();
        if self.dfs(src, tgt, &mut path, &mut visited) {
            Some(path)
        } else {
            None
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
    fn dfs() {
        let g = create_graph();

        let p = g.depth_first_search(0, 4).unwrap();
        assert_eq!(p, vec![0, 2, 4]);
    }
}
