use std::collections::{BinaryHeap, VecDeque};

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

    pub fn dijkstra(&self, src: usize, tgt: usize) -> Option<(Vec<usize>, i32)> {
        let mut visited = vec![false; self.num_verteces];
        visited[src] = true;
        let mut dist = vec![None; self.num_verteces];
        dist[src] = Some(0);
        let mut heap = BinaryHeap::with_capacity(self.num_verteces);
        heap.push(to_heap_elem(src, &dist));
        let mut prev = vec![None; self.num_verteces];

        while let Some(MinHeapElement { vertex, dist: cost }) = heap.pop() {
            for Edge { weight, tgt } in &self.edges[vertex] {
                let tgt = *tgt;
                if visited[tgt] {
                    continue;
                }
                visited[tgt] = true;

                let new_dist = cost + weight;
                if let Some(curr_dist) = dist[tgt] {
                    if curr_dist < new_dist {
                        continue;
                    }
                }

                dist[tgt] = Some(new_dist);
                heap.push(to_heap_elem(tgt, &dist));
                prev[tgt] = Some(vertex);
            }
        }

        prev[tgt].map(|mut curr| {
            let mut path = VecDeque::from([curr, tgt]);
            while let Some(prev) = prev[curr] {
                path.push_front(prev);
                curr = prev;
            }
            (path.into(), dist[tgt].expect("this has to be some"))
        })
    }
}

fn to_heap_elem(vertex: usize, dist: &Vec<Option<i32>>) -> MinHeapElement {
    MinHeapElement {
        vertex,
        dist: dist[vertex].expect("only add visited"),
    }
}

#[derive(Eq, Ord)]
struct MinHeapElement {
    vertex: usize,
    dist: i32,
}

impl PartialEq for MinHeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for MinHeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
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

    #[test]
    fn dijkstra() {
        let mut g = Graph::new(5);
        g.add_multiple(0, &[(1, 1), (2, 5)]);
        g.add_multiple(1, &[(2, 7), (3, 3)]);
        g.add_multiple(2, &[(4, 1)]);
        g.add_multiple(3, &[(1, 1), (2, 2)]);

        let (path, cost) = g.dijkstra(0, 4).unwrap();
        assert_eq!(path, vec![0, 2, 4]);
        assert_eq!(cost, 6);
    }
}
