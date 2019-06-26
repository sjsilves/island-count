use super::*;
use std::time::{Instant};

#[test]
fn it_merges() {
    let land1: Vec<Point> = [Point { x: 1, y: 4 }].to_vec();
    let land2: Vec<Point> = [Point { x: 4, y: 1 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }].to_vec();

    let new_island = merge_islands(Island{id:0, lands: land1}, Island{id:3, lands: land2});
    assert_eq!(new_island.lands.len(), 4);   
}

#[test]
fn it_merges_2_islands_into_1() {
    let island1 = Island { id: 1, lands: [Point { x: 1, y: 4 }, Point { x: 2, y: 3 }].to_vec() };
    let island2 = Island { id: 0, lands: [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 2 }].to_vec() };
    
    let new_island = merge_islands(island1, island2);
    assert_eq!(new_island.lands.len(), 6);   
}

#[test]
fn it_adds_land_to_existing_island () {
    let island = Island { id: 0, lands: [Point { x: 1, y: 4 }].iter().cloned().collect() };
    let land = Point { x: 2, y: 3 };

    let new_island = add_land_to_existing_island(island, land.clone());

    assert_eq!(new_island, Island { id: 0, lands: [Point { x: 1, y: 4 }, Point { x: 2, y: 3 }].to_vec() });
}

#[test]
fn it_gets_id_0() {
    let islands: HashSet<Island> = HashSet::new();;
    let id = get_next_island_id(islands);

    assert_eq!(id, 0);   
}

#[test]
fn it_gets_id_5_with_skipped_ids() {
    let mut islands: HashSet<Island> = HashSet::new();

    let mut land1: Vec<Point> = [Point { x: 1, y: 4 }, Point { x: 2, y: 3 }, Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 2 }].to_vec();
    islands.insert(Island { id: 0, lands: land1 });

    let mut land2: Vec<Point> = [Point { x: 3, y: 6 }].to_vec();
    islands.insert(Island { id: 4, lands: land2 }); 

    let mut land3: Vec<Point> = [Point { x: 4, y: 1 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }].to_vec();
    islands.insert(Island { id: 3, lands: land3 });
    let id = get_next_island_id(islands);

    assert_eq!(id, 5);   
}

#[test]
fn it_has_6_islands_in_9_by_9_ocean() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,t,t],
                  &[o,t,o,o,t,o,o,o,t],
                  &[o,o,o,t,o,o,o,o,t],
                  &[o,o,o,o,o,o,t,o,t],
                  &[o,t,o,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o],
                  &[o,o,o,o,o,t,o,o,o],
                  &[o,o,o,o,o,t,o,o,o]];

    let now = Instant::now();
    let count = count_islands(ocean);
    println!("count_islands run duration: {}ms", now.elapsed().as_millis());

    assert_eq!(count, 6);
}

#[test]
#[should_panic]
fn it_panics_with_asymmetrical_ocean() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,t,t],
                  &[o,t,o,o,t,o,o,o,t],
                  &[o,o,o,t,o,o,o,o,t,o],
                  &[o,o,o,o,o,o,t,o,t],
                  &[o,t,o,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o],
                  &[o,o,o,o,o,t,o,o,o],
                  &[o,o,o,o,o,t,o,o,o]];

    count_islands(ocean);
}

#[test]
fn it_validates_an_ocean() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,t,t],
                  &[o,t,o,o,t,o,o,o,t],
                  &[o,o,o,t,o,o,o,o,t],
                  &[o,o,o,o,o,o,t,o,t],
                  &[o,t,o,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o],
                  &[o,o,o,o,o,t,o,o,o],
                  &[o,o,o,o,o,t,o,o,o]];

    let result = validate_ocean(ocean);
    println!["Validated: {}",result];
    assert_eq!(result, true);
}

#[test]
fn it_has_2_islands() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,o,t],
                  &[o,o,t],
                  &[t,o,t]];

    let now = Instant::now();
    let count = count_islands(ocean);
    println!("count_islands run duration: {}ms", now.elapsed().as_millis());
    assert_eq!(count, 3);
}

#[test]
fn it_has_3_islands() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,t],
                  &[o,o,o,t],
                  &[o,t,o,t],
                  &[o,o,o,o]];

    let now = Instant::now();
    let count = count_islands(ocean);
    println!("count_islands run duration: {}ms", now.elapsed().as_millis());
    assert_eq!(count, 3);
}

#[test]
fn it_has_5_islands() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o],
                  &[o,o,o,t,o,o,o,o],
                  &[o,o,o,o,o,o,t,o],
                  &[o,t,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o],
                  &[o,t,o,o,o,t,o,o],
                  &[o,o,o,o,o,t,o,o]];

    let now = Instant::now();
    let count = count_islands(ocean);
    println!("count_islands run duration: {}ms", now.elapsed().as_millis());
    assert_eq!(count, 5);
}

#[test]
fn it_has_11_islands_in_an_20_x_20() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o,o,o,o,o,o,t,t,t,t,t,t,t],
                  &[o,t,o,o,t,o,o,o,o,o,t,t,o,o,o,o,o,o,t,t],
                  &[o,o,o,t,o,o,o,o,o,o,t,t,o,o,o,t,o,o,o,t],
                  &[o,o,o,o,o,o,t,o,o,o,o,o,o,o,t,o,o,o,o,o],
                  &[o,t,o,o,o,o,o,o,o,t,o,o,o,t,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o,o,o,t,o,t,o,o,t,t,t,t,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,t,t,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,t,t,t,t,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,t,t,t,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,t,t,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,t,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,o,o,o,t,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,o,t]];

    let now = Instant::now();
    let count = count_islands(ocean);
    println!("count_islands run duration: {}ms", now.elapsed().as_millis());
    assert_eq!(count, 11);
}

#[test]
fn it_has_1_islands_in_an_20_x_20() {
    let t = true;
    //let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t],
                  &[t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t,t]];

    let now = Instant::now();
    let count = count_islands(ocean);
    println!("count_islands run duration: {}ms", now.elapsed().as_millis());
    assert_eq!(count, 1);
}

#[test]
fn it_has_0_islands_in_an_20_x_20() {
    //let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o]];

    let now = Instant::now();
    let count = count_islands(ocean);
    println!("count_islands run duration: {}ms", now.elapsed().as_millis());
    assert_eq!(count, 0);
}

#[test]
fn it_has_4_islands() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o],
                  &[o,o,t,t,o,o,o,o],
                  &[o,o,o,o,o,o,t,o],
                  &[o,t,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o],
                  &[o,t,o,o,o,t,o,o],
                  &[o,o,o,o,o,t,o,o]];

    let count = count_islands(ocean);
    assert_eq!(count, 4);

    
}

#[test]
fn it_has_just_1_island() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,o,t],
                  &[o,t,o],
                  &[o,o,o]];

    let count = count_islands(ocean);
    assert_eq!(count, 1);

    
}

#[test]
fn it_has_1_island_really() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,o,t,o,t,o,t,o],
                  &[t,o,t,o,t,o,t,o],
                  &[t,o,t,o,t,o,t,o],
                  &[t,o,t,o,t,o,t,o],
                  &[t,o,t,o,t,o,t,o],
                  &[t,o,t,o,t,o,t,o],
                  &[t,o,t,o,t,o,t,o],
                  &[o,t,o,t,o,t,o,t]];

    let count = count_islands(ocean);
    assert_eq!(count, 1);  
}

#[test]
fn it_has_an_island_within_an_island() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,t,t,t,t,t,t],
                  &[t,o,o,o,o,o,o,t],
                  &[t,o,t,t,t,t,o,t],
                  &[t,o,t,t,t,t,o,t],
                  &[t,o,t,t,t,t,o,t],
                  &[t,o,t,t,t,t,o,t],
                  &[t,o,o,o,o,o,o,t],
                  &[t,t,t,t,t,t,t,t]];

    let count = count_islands(ocean);
    assert_eq!(count, 2);  
}

#[test]
fn it_has_0_islands() {
    //let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o]];

    let count = count_islands(ocean);
    assert_eq!(count, 0);  
}

#[test]
fn it_finds_2_islands() {
    let mut islands: HashSet<Island> = HashSet::new();
    let land1: Vec<Point> = [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 3, y: 0 },
    Point { x: 4, y: 0 }, Point { x: 5, y: 0 }, Point { x: 6, y: 0 }, Point { x: 7, y: 1 }, Point { x: 0, y: 2 },
    Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 3, y: 2 }, Point { x: 4, y: 2 }, Point { x: 5, y: 2 },
    Point { x: 6, y: 2 }].to_vec();
    islands.insert(Island { id: 0, lands: land1 }); 

    let land2: Vec<Point> = [Point { x: 0, y: 4 }, Point { x: 1, y: 4 }, Point { x: 2, y: 4 }, Point { x: 3, y: 4 },
    Point { x: 4, y: 4 }, Point { x: 5, y: 4 }, Point { x: 6, y: 4 }].to_vec();
    islands.insert(Island { id: 2, lands: land2 }); 

    let land3: Vec<Point> = [Point { x: 0, y: 6 }, Point { x: 1, y: 6 }, Point { x: 2, y: 6 }, Point { x: 3, y: 6 },
    Point { x: 4, y: 6 }, Point { x: 5, y: 6 }, Point { x: 6, y: 6 }].to_vec();
    islands.insert(Island { id: 3, lands: land3 });
     
    let adjacent_land = &[Point { x: 6, y: 2 }, Point { x: 6, y: 4 }];
    
    let found_islands = find_existing_islands(islands, adjacent_land);
    
    assert_eq!(found_islands.len(), 2);  
}
 
#[test]
fn it_might_work() {
    let x: &[usize] = &[0,1,2,3,4,5,6,7,8,9];
    let mut new_x: Vec<usize>;
    new_x = x[..1].to_vec();

    println!["{:?}", new_x];

} 

#[test]
fn test_iter() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,o,t],
                  &[o,t,o],
                  &[o,o,o]];
    
    let all = ocean.to_vec();
    let new_ocean = all.iter()
        .enumerate()
        .map(|(i, x)| (i, x.to_vec().iter()
            .enumerate()
            .filter(|f| f.1 == &true)
            .map(|(j, y)| (i, j, *y)).collect::<Vec<(usize, usize, bool)>>()))
        .map(|(_, x)| x)
        .flatten()
        .collect::<Vec<(usize, usize, bool)>>();
    println!("new_ocean {:?}", new_ocean); 
}

#[test]
fn test_fold() {
    let numbers_iterator = [0,2,3,4,5].iter();
    let sum = numbers_iterator
    .fold(0, |total, next| total + next);
    //let squared = (1..10).iter()
    //.map(|&x| x * x).collect();
    println!["{}", sum];
}