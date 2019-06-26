use std::collections::HashSet;
use super::*;
//use std::time::{Instant};
//use rayon::prelude::*;
extern crate rand;
//use rand::Rng;

#[cfg(test)]
mod tests;


pub fn count_islands(ocean: &[&[bool]]) -> usize {

    if !validate_ocean(ocean) {
        panic!["Invalid ocean shape"];
    }

    let all_lands = flatten_all_land(&ocean);

    let mut lands: HashSet<Point> = HashSet::new();
    let max_size: i32 = ocean.len() as i32;
    let mut island_count = 0;

    for land in all_lands.clone() {
        //println!["Land: {:?}", land];
        if !lands.contains(&land){
            lands = union(&lands, &check_land(all_lands.clone(),lands.clone(), &land, &max_size));
            //println!["lands: {:?}", lands];
            island_count += 1;
        }
    }
    
    return island_count;

}

fn flatten_all_land(ocean: &[&[bool]]) -> Vec<Point> {
    let all = ocean.to_vec();
    return all.iter()
        .enumerate()
        .map(|(i, x)| (i, x.to_vec().iter()
            .enumerate()
            .filter(|f| f.1 == &true)
            .map(|(j, y)| Point{x: i as i32, y: j as i32}).collect::<Vec<Point>>()))
        .map(|(_, x)| x)
        .flatten()
        .collect::<Vec<Point>>();
}

fn check_land (all_land: Vec<Point>, lands: HashSet<Point>, land: &Point, max_size: &i32) -> HashSet<Point> {
    let mut new_lands = lands.clone();
    //println!["check_land.lands: {:?}", lands];
    if !lands.contains(&land.clone()) {
        new_lands.insert(land.clone());
        let adj_lands = check_adjacent_land(all_land.clone(), new_lands.clone(), &land, max_size);
        for l in adj_lands {
            new_lands = union(&new_lands.clone(), &check_land(all_land.clone(), new_lands.clone(), &l, max_size));
        }
    }
    
    return new_lands;
}

fn check_adjacent_land (all_land: Vec<Point>, lands: HashSet<Point>, land: &Point, max_size: &i32) -> HashSet<Point> {
    //println!["check_adjacent_land({:?}, {:?},{:?},{}", all_land, lands, land, max_size];
    let mut adj_lands:HashSet<Point> = HashSet::new();
    let test_lands: Vec<Point> = [Point{x:-1, y:-1}, Point{x:-1, y:0}, Point{x:-1, y:1}, Point{x:0, y:-1}, Point{x:0, y:1}, Point{x:1, y:-1}, Point{x:1, y:0}, Point{x:1, y:1}].to_vec();
    let mut counter = 0;

    for test_land in test_lands.clone() {
        let land_test = Point{x:land.x+test_land.x, y:land.y+test_land.y};
        if check_bounds(max_size, land, &land_test) 
                && all_land.clone().contains(&land_test) 
                //&& !lands.clone().contains(&land_test) 
                {
            adj_lands.insert(land_test);
            counter += 1;
            //println!["adj_lands_{} :: {:?} :: {:?} :: {:?}", counter, land, land_test, adj_lands];
            
        }
    }
    //println!["adj_lands: {:?}", adj_lands];

    return adj_lands;
}

fn union(lands1: &HashSet<Point>, lands2: &HashSet<Point>) -> HashSet<Point> {
    let new_lands = lands1.union(&lands2)
        .map(|x| x.clone())
        .collect::<HashSet<Point>>(); 
    return new_lands;
}

fn check_bounds (max_size: &i32, land: &Point, test_land: &Point) -> bool {
    if  test_land.x >= 0 
        && test_land.y >= 0
        && (test_land.x) < *max_size
        && (test_land.y) < *max_size {
        return true;
    }
    return false;
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
fn build_ocean(size: usize) -> Vec<Point> {
    let mut ocean: &[&[bool]] = &[&[]];

    for i in 0..size {
        let mut row: &[bool] = &[];
        for j in 0..size {
            let mut rng = rand::thread_rng();
            row[j] = rng.gen();
        }
        ocean[i] = row;
    }
    return flatten_all_land(&ocean);
}
*/
