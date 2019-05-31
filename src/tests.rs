use super::*;
use std::time::{Instant};

#[test]
fn it_merges() {
    let found_islands: &[_] = &[Island{id:0, lands:[Point{x:0,y:0}, Point{x:0,y:1}, Point{x:1,y:1}].to_vec()}
                                ,Island{id:2, lands:[Point{x:2,y:2}].to_vec()}
                                ];
    let new_island = merge_islands (found_islands);

    assert_eq!(new_island.id, 0);
    assert_eq!(new_island.lands.len(), 4);   
}

#[test]
fn it_gets_id_0() {
    let islands: &[_] = &[];
    let id = get_next_island_id(islands);

    assert_eq!(id, 0);   
}

#[test]
fn it_gets_id_5_with_skipped_ids() {
    let islands: &[_] = &[
        Island { id: 0, lands: [Point { x: 1, y: 4 }, Point { x: 2, y: 3 }, Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 2 }].to_vec() }, 
        Island { id: 4, lands: [Point { x: 3, y: 6 }].to_vec() }, 
        Island { id: 3, lands: [Point { x: 4, y: 1 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }].to_vec() }].to_vec();
    let id = get_next_island_id(islands);

    assert_eq!(id, 5);   
}

#[test]
#[should_panic]
fn it_panics_with_empty_islands() {
    let islands: &[_] = &[].to_vec();
    let island = Island { id: 0, lands: [Point { x: 0, y: 2 }].to_vec() };
    find_index_by_island_id(islands, island.id);
}

#[test]
fn it_finds_index_1() {
    let islands: &[_] = &[
        Island { id: 0, lands: [Point { x: 1, y: 4 }, Point { x: 2, y: 3 }, Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 2 }].to_vec() }, 
        Island { id: 4, lands: [Point { x: 3, y: 6 }].to_vec() }, 
        Island { id: 3, lands: [Point { x: 4, y: 1 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }].to_vec() }].to_vec();
    
    let island = Island { id: 4, lands: [Point { x: 3, y: 6 }].to_vec() };
    let index = find_index_by_island_id(islands, island.id);

    assert_eq!(index, 1);   
}

#[test]
fn it_removes_island_leaving_0() {
    let islands: &[_] = &[
        Island { id: 0, lands: [Point { x: 3, y: 6 }].to_vec() }].to_vec();
    
    let island = &[ Island { id: 0, lands: [Point { x: 3, y: 6 }].to_vec() } ];
    let new_islands = remove_islands(islands, island);

    assert_eq!(new_islands.len(), 0);   
}

#[test]
fn it_removes_2_islands() {
    let islands: &[_] = &[
        Island { id: 0, lands: [Point { x: 1, y: 4 }].to_vec() }, 
        Island { id: 4, lands: [Point { x: 3, y: 6 }].to_vec() }, 
        Island { id: 3, lands: [Point { x: 4, y: 1 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }].to_vec() }].to_vec();
    
    
    let island = &[ Island { id: 0, lands: [Point { x: 1, y: 4 }].to_vec() }, Island { id: 4, lands: [Point { x: 3, y: 6 }].to_vec() } ];
    let new_islands = remove_islands(islands, island);

    assert_eq!(new_islands.len(), 1);   
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
    let islands = &[
        Island { id: 0, lands: [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 3, y: 0 }, Point { x: 4, y: 0 }, Point { x: 5, y: 0 }, Point { x: 6, y: 0 }, Point { x: 7, y: 1 }, Point { x: 0, y: 2 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 3, y: 2 }, Point { x: 4, y: 2 }, Point { x: 5, y: 2 }, Point { x: 6, y: 2 }].to_vec() }, 
        Island { id: 2, lands: [Point { x: 0, y: 4 }, Point { x: 1, y: 4 }, Point { x: 2, y: 4 }, Point { x: 3, y: 4 }, Point { x: 4, y: 4 }, Point { x: 5, y: 4 }, Point { x: 6, y: 4 }].to_vec() }, 
        Island { id: 3, lands: [Point { x: 0, y: 6 }, Point { x: 1, y: 6 }, Point { x: 2, y: 6 }, Point { x: 3, y: 6 }, Point { x: 4, y: 6 }, Point { x: 5, y: 6 }, Point { x: 6, y: 6 }].to_vec() }];
    let land = Point { x: 7, y: 3 }; 
    let adjacent_land = &[Point { x: 6, y: 2 }, Point { x: 6, y: 4 }];
    
    let found_islands = find_islands(islands, land, adjacent_land);
    
    assert_eq!(found_islands.len(), 2);  
}
 
#[test]
fn it_might_work() {
    let x: &[usize] = &[0,1,2,3,4,5,6,7,8,9];
    let mut new_x: Vec<usize>;
    new_x = x[..1].to_vec();

    println!["{:?}", new_x];

} 