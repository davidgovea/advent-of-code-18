use std::io::{self, Read, Write};
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day X --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

type Coord = (i32, i32);
type CoordId = usize;
type CoordEntry = (CoordId, Coord);
fn parse_coords(input: &str) -> Vec<CoordEntry> {
    input.lines().enumerate().map(|(index, line)| {
        let parsed = line.split(", ").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        (index, (parsed[0], parsed[1]))
    }).collect::<Vec<CoordEntry>>()
}

fn get_bounding_rect(coords: &Vec<CoordEntry>) -> (Coord, Coord) {
    let mut sort_coords = coords.clone();
    sort_coords.sort_by_key(|c| (c.1).0);
    let min_x = (sort_coords.first().unwrap().1).0;
    let max_x = (sort_coords.last().unwrap().1).0;

    sort_coords.sort_by_key(|c| (c.1).1);
    let min_y = (sort_coords.first().unwrap().1).1;
    let max_y = (sort_coords.last().unwrap().1).1;

    ((min_x, min_y), (max_x, max_y))
}

#[derive(Debug)]
enum CellState {
    Shared,
    Claimed(CellClaim)
}

#[derive(Debug)]
struct CellClaim {
    dist: u32,
    coord_id: CoordId,
    pos: Coord,
}

fn enumerate_points(coord: &Coord, distance: i32) -> Vec<Coord> {
    let mut points = Vec::new();
    for n in 0..=distance {
        let dx = distance - n;
        let dy = n;
        points.push((coord.0 + dx, coord.1 + dy));
        if n != distance && n != 0 {
            points.push((coord.0 - dx, coord.1 + dy));
            points.push((coord.0 + dx, coord.1 - dy));
        }
        if (distance != 0) {
            points.push((coord.0 - dx, coord.1 - dy));
        }
    }
    points
}

fn part1(input: &str) -> Result<(), Box<std::error::Error>> {

    let coords_list = parse_coords(input);
    let ((min_x, min_y), (max_x, max_y)) = get_bounding_rect(&coords_list);

    let search_area = (max_x - min_x + 1) * (max_y - min_y + 1);

    let mut grid: HashMap<(i32, i32), CellState> = HashMap::new();
    let mut search_distance = 0;
    while grid.len() < search_area as usize {
        for (coord_id, coord) in &coords_list {
            let points_to_visit = enumerate_points(coord, search_distance);

            for point in points_to_visit {
                if point.0 < min_x || point.0 > max_x ||
                   point.1 < min_y || point.1 > max_y { continue; }
                                
                match grid.get(&point) {
                    None => {
                        grid.insert(point, CellState::Claimed(CellClaim { dist: search_distance as u32, coord_id: *coord_id, pos: point }));
                    },
                    Some(CellState::Claimed(c)) if c.dist == search_distance as u32 => {
                        grid.insert(point, CellState::Shared);
                    },
                    _ => ()
                }
            }

        }
        search_distance += 1;
    }

    let mut cells_by_id: HashMap<CoordId, Vec<&CellClaim>> = HashMap::new();
    let mut current_id: CoordId = Default::default();
    for (cell_coord, state) in grid.iter() {
        let is_boundary = cell_coord.0 == min_x || cell_coord.0 == max_y ||
                          cell_coord.1 == min_y || cell_coord.1 == max_y;
        match state {
            CellState::Claimed(c) => {
                cells_by_id.entry(c.coord_id).or_default().push(c);
            }
            _ => ()
        }
    }

    let mut interior_area_totals = cells_by_id.iter()
        .filter(|(id, claims)| {
            for claim in claims.iter() {
                let is_boundary = claim.pos.0 == min_x || claim.pos.0 == max_y ||
                                  claim.pos.1 == min_y || claim.pos.1 == max_y;
                if is_boundary { return false; }
            }
            return true;
        })
        .map(|(id, list)| (id, list.len()))
        .collect::<Vec<_>>();

    interior_area_totals.sort_by_key(|a| a.1);

    writeln!(io::stdout(), "looks like the largest interior area is {} cells", interior_area_totals.last().unwrap().1);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<std::error::Error>> {

    let coords_list = parse_coords(input);
    let coord_count = coords_list.len();
    let ((min_x, min_y), (max_x, max_y)) = get_bounding_rect(&coords_list);

    let search_area = (max_x - min_x + 1) * (max_y - min_y + 1);

    let mut grid: HashMap<(i32, i32), (u32, u32)> = HashMap::new();
    let mut search_distance = 0;
    let mut cells_reached = 0;
    let mut found_area = 0;

    while cells_reached < search_area {
        for (coord_id, coord) in &coords_list {
            let points_to_visit = enumerate_points(coord, search_distance);

            for point in points_to_visit {
                if point.0 < min_x || point.0 > max_x ||
                   point.1 < min_y || point.1 > max_y { continue; }

                match grid.get(&point) {
                    Some((count, total)) => {
                        let new_total = total + &(search_distance as u32);

                        if count == &((coord_count - 1) as u32) {
                            cells_reached += 1;
                            if (new_total) < 10000 {
                                found_area += 1;
                            }
                        } else {
                            grid.insert(point, (count + 1, new_total));
                        }
                    },
                    None => {
                        grid.insert(point, (1, search_distance as u32));
                    }
                }
            }

        }
        search_distance += 1;
    }

    writeln!(io::stdout(), "there are {} cells within a total distance of 10000 from all points", found_area)?;
    Ok(())
}

