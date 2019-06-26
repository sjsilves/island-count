use std::collections::HashSet;
use super::*;
use rayon::prelude::*;
extern crate rand;

#[cfg(test)]
mod tests;


pub fn count_islands(ocean: &[&[bool]]) -> usize {

    if !validate_ocean(ocean) {
        panic!["Invalid ocean shape"];
    }

    let mut islands: HashSet<Island> = HashSet::new();

    let mut x = 0;

    for row in ocean {
        let mut y = 0;
        for column in *row {
            if *column == true {
                //println!["Land ho!: ({},{})", x, y];
                let land = Point { x: x, y: y };
                // look around for adjacent land
                let adjacent_land = get_adjacent_land(&ocean, land.clone());

                // see if any of the adjacent land is part of an island we already mapped
                let found_existing_islands = find_existing_islands(islands.clone(), &adjacent_land[..]);
                //println!["found_existing_islands: {:?}", found_existing_islands];
                let mut new_island: Island;

                if found_existing_islands.len() > 0 {
                    let this_island = add_land_to_existing_island(found_existing_islands[0].clone(), land);
                    if found_existing_islands.len() > 1 {
                        let merged_island = merge_islands(this_island, found_existing_islands[1].clone());
                        islands.remove(&found_existing_islands[0].clone());
                        islands.remove(&found_existing_islands[1].clone());
                        islands.insert(merged_island);
                    } else {
                        islands.remove(&found_existing_islands[0].clone());
                        islands.insert(this_island); 
                    }
                } else {
                    let mut lands: Vec<Point> = Vec::new();
                    lands.push(land.clone());
                    new_island = Island { id: get_next_island_id(islands.clone()), lands: lands };
                    islands.insert(new_island);
                }

                //println!["There are now {} islands on our map: {:?}", islands.len(), islands];
            } 
            y += 1;
        }
        x += 1;
    }

    return islands.len();
}

fn get_adjacent_land (ocean: &[&[bool]], land: Point) -> Vec<Point> {
    let mut adjacent_land: Vec<Point> = Vec::new();
    //println!["get_adjacent_land for ({},{})", land.x, land.y];
    /*
    for x in land.x-1..land.x+2 {
        for y in land.y-1..land.y+2 {
            let test_point: Point = Point{x: x, y: y};

            //println!["check for adjacent_land at ({},{})", test_point.x, test_point.y];
            let current_point: Point = Point{x: land.x, y: land.y};
            if  test_point != current_point 
                    && x >= 0 
                    && y >= 0
                    && x < ocean.len() as i32
                    && y < ocean[0].len() as i32 {
                if ocean[x as usize][y as usize] == true {
                    adjacent_land.push(Point{x: x, y: y});
                }
            }
        }
    }
    */

    
    if land.x-1 >= 0 && ocean[(land.x-1) as usize][land.y as usize] {
        adjacent_land.push(Point{x: land.x-1, y: land.y});
    }
    if land.x-1 >= 0 && land.y-1 >= 0 && ocean[(land.x-1) as usize][(land.y-1) as usize] {
        adjacent_land.push(Point{x: land.x-1, y: land.y-1});
    }
    if land.y-1 >= 0 && ocean[land.x as usize][(land.y-1) as usize] {
        adjacent_land.push(Point{x: land.x, y: land.y-1});
    }
    if land.x+1 < ocean.len() as i32 && land.y-1 >= 0 && ocean[(land.x+1) as usize][(land.y-1) as usize] {
        adjacent_land.push(Point{x: land.x+1, y: land.y-1});
    }
    
    return adjacent_land;
}

fn find_existing_islands (islands: HashSet<Island>, adjacent_land: &[Point]) -> Vec<Island> {
    //println!["islands before find_existing_islands: {:?}", islands];
    let mut found_islands: Vec<Island> = Vec::new();
    if islands.len() > 0 {
        for island in islands.iter() {
            for land in island.lands.iter() {
                if adjacent_land.par_iter().any(|x| x.x == land.x && x.y == land.y) {
                    //println!["found existing island: {:?}", island];
                    let new_island = island.clone();
                    
                    if !found_islands.par_iter().any(|x| x.id == new_island.id) {
                        found_islands.push(new_island);
                    }
                }
            }
        }
    }

    return found_islands;
}

fn merge_islands (island_to_merge1: Island, island_to_merge2: Island) -> Island {
    let merged_id = island_to_merge1.id;
    
    let mut new_lands: Vec<Point> = Vec::new();
    
    new_lands = [&new_lands[..], &island_to_merge1.lands[..]].concat();
    new_lands = [&new_lands[..], &island_to_merge2.lands[..]].concat();

    return Island{ id: merged_id, lands: new_lands };
} 

fn add_land_to_existing_island (island: Island, land: Point) -> Island {
    let mut new_lands: Vec<Point> = island.lands.clone();
    new_lands.push(land);

    return Island{id: island.id, lands: new_lands};
}

fn get_next_island_id(islands: HashSet<Island>) -> usize {
    let id = match islands.iter().max_by_key(|x| x.id ){
        Some(i) => i.id+1,
        None => 0
    };
    
    return id;
}

fn validate_ocean(ocean: &[&[bool]]) -> bool {
    for row in ocean {
        if row.len() != ocean.len() {
            return false;
        }
    }

    return true;
}