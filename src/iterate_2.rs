use super::*;
use std::collections::HashSet;
//use std::time::{Instant};
use rayon::prelude::*;
extern crate rand;
//use rand::Rng;

#[cfg(test)]
mod tests;

pub fn count_islands(ocean: &[&[bool]]) -> usize {

    if !validate_ocean(ocean) {
        panic!["Invalid ocean shape"];
    }

    let all_lands = flatten_all_land(&ocean);

    let mut islands: Vec<Vec<Point>> = Vec::new();
    let mut lands: HashSet<Point> = HashSet::new();
    let mut island_count = 0;
    //let mut x = 0;

    for land in all_lands.clone() {
    //for row in ocean.iter() {
    //    let mut y = 0;
    //    for column in row.iter() {
    //        if *column {
                //println!["Land ho!: ({},{})", x, y];
                let land = Point { x: land.x, y: land.y };
                lands.insert(land.clone());
                // look around for adjacent land
                let adjacent_land = get_adjacent_land(ocean, &land.clone());
                if adjacent_land.len() == 1 {
                    island_count += 1;
                }
                lands = lands.union(&adjacent_land.clone())
                    .map(|x| x.clone())
                    .collect::<HashSet<Point>>(); 
                //println!["adjacent_land: {:?}", adjacent_land];

                // see if any of the adjacent land is part of an island we already mapped
                let found_islands = find_islands(islands.clone(), &land.clone(), adjacent_land);
                if found_islands.len() > 1 {
                    island_count -= 1;
                }
                //println!["found_islands: {:?}", found_islands];
                let mut new_island: Vec<Point>;

                //if found_islands.len() > 0 {
                    new_island = merge_islands(found_islands);
                    //#islands = remove_islands(&islands, &found_islands[..]);
                //} else {
                //    new_island = [land].to_vec();
                //}

                //islands.push(new_island);

                //println!["There are now {} islands on our map: {:?}", islands.len(), islands];
                
    //        } 
     //       y += 1;
     //   }
    //    x += 1;
    }

    //return islands.len();
    return island_count;
}

fn flatten_all_land(ocean: &[&[bool]]) -> Vec<Point> {
    let all = ocean.to_vec();
    return all.iter()
        .enumerate()
        .map(|(i, x)| (i, x.to_vec().iter()
            .enumerate()
            .filter(|f| f.1 == &true)
            .map(|(j, y)| Point{x:i as i32, y:j as i32}).collect::<Vec<Point>>()))
        .map(|(_, x)| x)
        .flatten()
        .collect::<Vec<Point>>();
}

fn get_adjacent_land (ocean: &[&[bool]], land: &Point) -> HashSet<Point> {
    let mut adjacent_land: HashSet<Point> = HashSet::new();
    
    if land.x-1 >= 0 && ocean[(land.x-1) as usize][land.y as usize] {
        adjacent_land.insert(Point{x: land.x-1, y: land.y});
    }
    if land.x-1 >= 0 && land.y-1 >= 0 && ocean[(land.x-1) as usize][(land.y-1) as usize] {
        adjacent_land.insert(Point{x: land.x-1, y: land.y-1});
    }
    if land.y-1 >= 0 && ocean[land.x as usize][(land.y-1) as usize] {
        adjacent_land.insert(Point{x: land.x, y: land.y-1});
    }
    if land.x+1 < ocean.len() as i32 && land.y-1 >= 0 && ocean[(land.x+1) as usize][(land.y-1) as usize] {
        adjacent_land.insert(Point{x: land.x+1, y: land.y-1});
    }
    
    return adjacent_land;
}

fn find_islands (islands: Vec<Vec<Point>>, land: &Point, adjacent_land: HashSet<Point>) -> Vec<Vec<Point>> {
    let mut found_islands: Vec<Vec<Point>> = Vec::new();
    let mut land_added = false;

    for island in islands.iter() {
        for l in island.iter() {
            if adjacent_land.par_iter().any(|x| x.x == l.x && x.y == l.y) {
                let mut new_island = island.clone();
                if !land_added {
                    new_island.push(land.clone());
                    land_added = true;
                }
                
                //#if !found_islands.par_iter().any(|x| x.id == new_island.id) {
                //#    found_islands.push(new_island);
                //#}
            }
        }
    }

    return found_islands;
}

fn merge_islands (islands: Vec<Vec<Point>>) -> Vec<Point> {
    //#islands.clone().to_vec().sort_by(|a1, b1| a1.id.cmp(&b1.id));
    
    let mut new_lands: Vec<Point> = Vec::new();

    //#for found in islands.iter() {
    //#    new_lands = [&new_lands[..], &found.lands[..]].concat();
    //#}

    return new_lands;
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

/*#
fn get_point_vector_index(vec: Vec<Point>, val: Point) -> usize {
    let res1 = vec.iter().position(|&v| v == val);

    println!("{:?}", res1);    // outputs: Some(2)
    return 1;
}
#*/
fn validate_ocean(ocean: &[&[bool]]) -> bool {
    for row in ocean.iter() {
        if row.len() != ocean.len() {
            return false;
        }
    }
    return true;
}