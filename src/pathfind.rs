use std::collections::{BTreeMap, HashMap, HashSet};

use failure::Error;

use grid::Grid;

pub fn find_path(
    start: (i32, i32),
    end: (i32, i32),
    grid: &Grid,
) -> Result<Option<Vec<(i32, i32)>>, Error> {
    fn neighbors<'a>(
        (x, y): (i32, i32),
        visited: &'a HashSet<(i32, i32)>,
        grid: &'a Grid,
    ) -> impl Iterator<Item = (i32, i32)> + 'a {
        [(0, 1), (1, 0), (-1, 0), (0, -1)]
            .into_iter()
            .filter_map(move |direction| {
                let adjacent = (x + direction.0, y + direction.1);
                if adjacent.0 >= 0 && adjacent.0 < grid.width() && adjacent.1 >= 0
                    && adjacent.1 < grid.height() && !visited.contains(&adjacent)
                    && !grid.occupied(&adjacent)
                {
                    Some(adjacent)
                } else {
                    None
                }
            })
    };
    let distance = |(from_x, from_y): (i32, i32), (to_x, to_y): (i32, i32)| {
        (to_x - from_x).abs() + (to_y - from_y).abs()
    };

    let mut visited = HashSet::new();
    let mut edges = HashMap::new();
    let mut to_visit = BTreeMap::new();
    {
        let e = to_visit
            .entry(distance(start, end))
            .or_insert_with(|| HashSet::new());
        e.insert(start);
    }

    let path = 'pathfind: loop {
        let k = *to_visit
            .keys()
            .next()
            .ok_or_else(|| format_err!("no more nodes to search"))?;
        let nodes = to_visit.remove(&k).unwrap();
        for n in nodes {
            visited.insert(n);

            for adjacent in neighbors(n, &visited, grid) {
                if adjacent == end {
                    // reached the end, collect the path
                    break 'pathfind {
                        let mut path = vec![adjacent];
                        let mut prev = Some(n);
                        while let Some(n) = prev {
                            path.push(n);
                            prev = edges.get(&n).cloned();
                        }
                        path
                    };
                }

                edges.insert(adjacent, n);
                let e = to_visit
                    .entry(distance(adjacent, end))
                    .or_insert_with(|| HashSet::new());
                e.insert(adjacent);
            }
        }
    };

    Ok(Some(path.into_iter().rev().collect()))
}
