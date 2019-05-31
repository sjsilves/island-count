//use std::time::{Instant};
use crate::point_struct::Point;
use crate::island_struct::Island;
use rayon::prelude::*;
mod island_struct;
mod point_struct;
extern crate rand;
//use rand::Rng;

#[cfg(test)]
mod tests;

fn main() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o,t],
                  &[o,t,o,o,t,o,o,o,t],
                  &[o,o,t,t,o,o,o,o,t],
                  &[o,o,o,o,o,o,t,o,t],
                  &[o,t,o,o,o,o,o,o,t],
                  &[o,t,t,o,t,o,o,o,t],
                  &[o,t,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,t,o,o,t]];

    //const size: usize = 30;
    //let mut rng = rand::thread_rng();
    //let mut ocean: Vec<&[bool]> = Vec::new();

    //let mut row = Vec::new();
    //for j in 0usize..size {
    //    row.push(rng.gen());
    //}
    //for i in 0usize..size {
    //    ocean.push(&row[..]);
    //}
    
    println!("Number of islands in the ocean: {}", count_islands(&ocean[..]));
}

fn count_islands(ocean: &[&[bool]]) -> usize {

    if !validate_ocean(ocean) {
        panic!["Invalid ocean shape"];
    }
    let mut islands: Vec<Island> = Vec::new();
    let mut x = 0;

    for row in ocean {
        let mut y = 0;
        for column in *row {
            if *column == true {
                //println!["Land ho!: ({},{})", x, y];
                let land = Point { x: x, y: y };
                // look around for adjacent land
                let adjacent_land = get_adjacent_land(&ocean, land);
                //println!["adjacent_land: {:?}", adjacent_land];

                // see if any of the adjacent land is part of an island we already mapped
                let found_islands = find_islands(&islands[..], land, &adjacent_land[..]);
                //println!["found_islands: {:?}", found_islands];
                let mut new_island: Island;

                if found_islands.len() > 0 {
                    new_island = merge_islands(&found_islands[..]);
                    islands = remove_islands(&islands, &found_islands[..]);
                } else {
                    new_island = Island { id: get_next_island_id(&islands), lands: [land].to_vec() };
                }

                islands.push(new_island);

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

    for x in land.x-1..land.x+2 {
        for y in land.y-1..land.y+2 {
            let test_point: Point = Point{x: x, y: y};
            let current_point: Point = Point{x: land.x, y: land.y};
            if  test_point != current_point 
                    && x >=0 
                    && y >= 0
                    && x < ocean.len() as i32
                    && y < ocean[0].len() as i32 {
                if ocean[x as usize][y as usize] == true {
                    adjacent_land.push(Point{x: x, y: y});
                }
            }
        }
    }
    
    return adjacent_land;
}

fn find_islands (islands: &[Island], land: Point, adjacent_land: &[Point]) -> Vec<Island> {
    let mut found_islands: Vec<Island> = Vec::new();
    let mut land_added = false;

    for island in islands.iter() {
        for l in island.lands.iter() {
            if adjacent_land.par_iter().any(|x| x.x == l.x && x.y == l.y) {
                let mut new_island = island.clone();
                if !land_added {
                    new_island.lands.push(land);
                    land_added = true;
                }
                
                if !found_islands.par_iter().any(|x| x.id == new_island.id) {
                    found_islands.push(new_island);
                }
            }
        }
    }

    return found_islands;
}

fn merge_islands (islands: &[Island]) -> Island {
    islands.clone().to_vec().sort_by(|a1, b1| a1.id.cmp(&b1.id));
    
    let mut new_lands: Vec<Point> = Vec::new();

    for found in islands.iter() {
        new_lands = [&new_lands[..], &found.lands[..]].concat();
    }

    return Island{ id: islands[0].id, lands: new_lands };
} 

fn remove_islands (islands: &[Island], islands_to_remove: &[Island]) -> Vec<Island> {
    let mut new_islands = islands.to_vec();
    for island in islands_to_remove.iter() {
        let remove_index = find_index_by_island_id(&new_islands[..], island.id);
        new_islands.remove(remove_index);
    }
    new_islands.clone().to_vec().sort_by(|a1, b1| a1.id.cmp(&b1.id));

    return new_islands;
} 

fn find_index_by_island_id(islands: &[Island], id: usize) -> usize {
    let index = match islands.iter().position(|x| x.id == id){
        Some(n) => n,
        None => panic!["Island {:?} not found in: {:?}", id, islands]
    };

    return index;
}

fn get_next_island_id(islands: &[Island]) -> usize {
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
/*
fn build_ocean(size: usize) -> &[&[bool]] {
    let mut ocean: &[&[bool]] = &[&[]];

    for i in 0..size {
        let mut row: &[bool] = &[];
        for j in 0..size {
            let mut rng = rand::thread_rng();
            row[j] = rng.gen();
        }
        ocean[i] = row;
    }
    return ocean.clone();
}
*/