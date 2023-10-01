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
        let mut queue = vec![vec![src]];

        while !queue.is_empty() {
            let path = queue.pop().expect("q is non-empty");
            let curr = *path.last().expect("path should be non-empty");
            if curr == tgt {
                return Some(path);
            }

            if visited[curr] {
                continue;
            }
            visited[curr] = true;

            for (i, e) in self.edges[curr].iter().enumerate() {
                if let Some(_) = e {
                    let mut new_path = path.clone();
                    new_path.push(i);
                    queue.push(new_path);
                }
            }
        }

        None
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
