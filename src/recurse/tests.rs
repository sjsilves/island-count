use super::*;
use std::time::{Instant};

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
fn it_has_55_islands() {
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
fn it_has_44_islands() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o],
                  &[o,t,o,t,o],
                  &[o,o,t,o,o],
                  &[o,o,o,o,o],
                  &[o,t,o,o,o]];
    /*let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o],
                  &[o,o,t,t,o,o,o,o],
                  &[o,o,o,o,o,o,t,o],
                  &[o,t,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o],
                  &[o,t,o,o,o,t,o,o],
                  &[o,o,o,o,o,t,o,o]];*/

    let count = count_islands(ocean);
    assert_eq!(count, 2);
}

#[test]
fn it_has_just_1_island_x() {
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
fn it_finds_adjacent_land(){
  let all_lands: Vec<Point> = [Point{x:0, y:0}, Point{x:0, y:2}, Point{x:1, y:1}].to_vec();
  let mut lands: HashSet<Point> = HashSet::new();
  lands.insert(Point{x:0, y:0});
  lands.insert(Point{x:1, y:1});
  let land = Point{x:1, y:1};
  let max_size = 3;
  let adj_land = check_adjacent_land(all_lands, lands, &land, &max_size);

  println!["adj_land: {:?}", adj_land];
  assert_eq!(adj_land.len(), 2);
}
 
#[test]
fn it_might_work() {
    let x: &[usize] = &[0,1,2,3,4,5,6,7,8,9];
    let mut new_x: Vec<usize>;
    new_x = x[..1].to_vec();

    println!["{:?}", new_x];

} 

#[test]
fn it_flattens_to_3_lands(){
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,o,t],
                  &[o,t,o],
                  &[o,o,o]];

    let all_lands = flatten_all_land(&ocean);

    println!["ISLANDS {:?}", all_lands];  

    assert_eq!(all_lands.len(), 3);
    //let this_thing = all_lands.iter()
    //    .map(|l| check_adjacent_land2(all_lands, l))
    //    .collect::<Vec<(usize, usize)>>();
    //let island_count = all_lands.iter()
    //   .map(|l| check_adjacent_land2(all_lands.clone(), l))
    //    .flatten()
    //    .collect::<Vec<(usize, usize)>>(); 
}

#[test]
fn it_ABC_123(){
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,o,t],
                  &[o,t,o],
                  &[o,o,o]];

    let all_lands = flatten_all_land(&ocean);

    println!["Number of lands: {:?}", all_lands];  

    assert_eq!(all_lands.len(), 3);
}

#[test]
fn it_unions(){
    let mut hash1: HashSet<Point> = HashSet::new();
    let mut hash2: HashSet<Point> = HashSet::new();
    hash1.insert(Point{x:1,y:1});
    hash1.insert(Point{x:1,y:2});
    hash1.insert(Point{x:2,y:4});
    hash2.insert(Point{x:0,y:3});
    hash2.insert(Point{x:2,y:0});

    let hash3 = union(&hash1, &hash2);

    println!["Number of lands: {:?}", hash3];  

    assert_eq!(hash3.len(), 5);
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