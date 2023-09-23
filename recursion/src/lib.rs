use anyhow::{anyhow, Error};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Wall,
    Empty,
}

#[allow(dead_code)]
struct Field {
    height: usize,
    width: usize,
    tiles: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point(i32, i32);

type Path = Vec<Point>;

fn parse_line(line: &str) -> Result<Vec<Tile>, Error> {
    let mut row = Vec::with_capacity(line.len());
    for c in line.chars() {
        row.push(match c {
            '#' => Tile::Wall,
            ' ' => Tile::Empty,
            _ => return Err(anyhow!("unknown symbol: {c}")),
        })
    }
    Ok(row)
}

#[allow(dead_code)]
impl Field {
    fn try_new(input: Vec<&str>) -> Result<Self, Error> {
        let height = input.len();
        if height == 0 {
            return Err(anyhow!("input has height 0"));
        }

        let width = input[0].len();

        let mut tiles = Vec::with_capacity(height);
        for line in input {
            if line.len() != width {
                return Err(anyhow!("line {line} has incorrect length"));
            }

            tiles.push(parse_line(line)?);
        }

        Ok(Self {
            height,
            width,
            tiles,
        })
    }

    fn at(&self, p: Point) -> Result<&Tile, Error> {
        if p.0 < 0 || p.1 < 0 || p.0 as usize >= self.height || p.1 as usize >= self.width {
            return Err(anyhow!("out of bounds: {p:?}"));
        }
        Ok(&self.tiles[p.0 as usize][p.1 as usize])
    }

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn walk(
        &self,
        curr: Point,
        end: Point,
        seen: &mut Vec<Vec<bool>>,
        path: &mut Vec<Point>,
    ) -> bool {
        // happy base case
        if curr == end {
            path.push(curr);
            return true;
        }
        // sad base cases
        if self.at(curr).is_err()
            || self.at(curr).is_ok_and(|t| t == &Tile::Wall)
            || seen[curr.0 as usize][curr.1 as usize]
        {
            return false;
        }

        // pre
        path.push(curr);
        seen[curr.0 as usize][curr.1 as usize] = true;

        // recurse
        for dir in Self::DIRECTIONS {
            if self.walk(Point(curr.0 + dir.0, curr.1 + dir.1), end, seen, path) {
                return true;
            }
        }

        // post
        path.pop();
        false
    }

    fn find(&self, start: Point, end: Point) -> Path {
        let mut seen = Vec::with_capacity(self.height);
        for _ in 0..self.height {
            seen.push(vec![false; self.width]);
        }

        let mut path = Vec::new();
        self.walk(start, end, &mut seen, &mut path);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let simple_input = vec!["######### #", "#         #", "# #########"];
        let f = Field::try_new(simple_input).unwrap();

        assert_eq!(f.height, 3);
        assert_eq!(f.width, 11);
        assert_eq!(f.tiles[0][0], Tile::Wall);
        assert_eq!(f.tiles[1][0], Tile::Wall);
        assert_eq!(f.tiles[2][0], Tile::Wall);
        assert_eq!(f.tiles[1][1], Tile::Empty);
    }

    #[test]
    fn simple_maze() {
        let simple_input = vec!["######### #", "#         #", "# #########"];
        let f = Field::try_new(simple_input).unwrap();

        let path = f.find(Point(2, 1), Point(0, 9));
        assert_eq!(
            path,
            vec![
                Point(2, 1),
                Point(1, 1),
                Point(1, 2),
                Point(1, 3),
                Point(1, 4),
                Point(1, 5),
                Point(1, 6),
                Point(1, 7),
                Point(1, 8),
                Point(1, 9),
                Point(0, 9)
            ]
        )
    }
}
