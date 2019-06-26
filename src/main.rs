use crate::point_struct::Point;
use crate::island_struct::Island;
mod point_struct;
mod island_struct;
mod iterate;
mod iterate_2;
mod recurse;
use std::time::{Instant};

fn main() {
  test_1();
  test_2();
  test_3();
  test_4();
  test_5();
  test_6();
}

fn iterate_a(ocean: &[&[bool]], test_num: usize) {
    let mut sum_dur: u128 = 0;
    let mut count = 0;
    for _i in 1..10 {
      let now = Instant::now();
      iterate::count_islands(&ocean[..]);
      sum_dur += now.elapsed().as_micros();
      count += 1;
    }
    //println!("Number of islands in the ocean: {}", count);
    println!["Iterate_a_{} average duration: {}", test_num, sum_dur/count];
}

fn iterate_b(ocean: &[&[bool]], test_num: usize) {
    let mut sum_dur: u128 = 0;
    let mut count = 0;
    for _i in 1..10 {
      let now = Instant::now();
      iterate_2::count_islands(&ocean[..]);
      sum_dur += now.elapsed().as_micros();
      count += 1;
    }
    //println!("Number of islands in the ocean: {}", count);
    println!["Iterate_b_{} average duration: {}", test_num, sum_dur/count];
}

fn recurse(ocean: &[&[bool]], test_num: usize) {
    let mut sum_dur: u128 = 0;
    let mut count = 0;
    for _i in 1..10 {
      let now = Instant::now();
      recurse::count_islands(&ocean[..]);
      sum_dur += now.elapsed().as_micros();
      count += 1;
    }
    //println!("Number of islands in the ocean: {}", count);
    println!["recurse_{} average duration: {}", test_num, sum_dur/count];
}

fn test_1() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o,t],
                  &[o,t,o,o,t,o,o,o,t],
                  &[o,o,t,t,o,o,o,o,t],
                  &[o,o,o,o,o,o,t,o,t],
                  &[o,t,o,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o,t],
                  &[o,t,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,t,o,o,t]];

    iterate_a(&ocean, 1);
    //iterate_b(&ocean, 1);
    recurse(&ocean, 1);
}

fn test_2() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,t,o],
                  &[o,t,o,o,t,o,o,o,t,o,o,o,t,t,o,o,o,o,t,t],
                  &[o,o,t,t,o,o,o,o,t,t,o,t,t,t,o,t,o,o,o,o],
                  &[o,o,o,o,o,o,t,o,t,t,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,o,o,o,o,o,o,t,t,t,t,o,o,o,o,o,o,o,o],
                  &[t,t,t,o,t,o,o,o,t,t,t,t,o,t,o,o,o,o,o,o],
                  &[t,t,o,o,o,t,o,o,t,o,o,o,t,o,o,o,o,o,o,o],
                  &[t,t,t,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o],
                  &[t,t,o,o,o,t,o,o,t,o,o,o,t,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,t,t,t,o,o],
                  &[o,o,o,o,o,o,o,t,o,o,o,o,o,o,t,t,t,t,o,t],
                  &[o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,t,t,t,o,t],
                  &[o,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,t,o,t],
                  &[o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,t,t,t,o,t],
                  &[o,o,t,o,o,o,o,o,t,t,o,o,o,o,t,t,t,t,o,t],
                  &[t,t,t,t,t,o,o,t,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,t,t,t,o,o,t,t,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,t,t,t,o,o,o,t,o,o,t,o,t,o,o,o,o,o,o],
                  &[t,o,o,o,o,o,o,t,t,o,o,t,t,t,o,o,o,o,o,o]];

    iterate_a(&ocean, 2);
    //iterate_b(&ocean, 2);
    recurse(&ocean, 2);
}

fn test_3() {
    let t = true;
    let o = false;

    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,t,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
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

    iterate_a(&ocean, 3);
    //iterate_b(&ocean, 3);
    recurse(&ocean, 3);
}

fn test_4() {
    let t = true;
    let o = false;
    let ocean: &[&[bool]] = 
                &[&[t,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,t,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o],
                  &[o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o],
                  &[o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o],
                  &[o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o],
                  &[o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,t,t,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o],
                  &[o,o,o,o,o,o,t,o,t,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,t,o,t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,t,t,o,t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,o,t,o,o,t,o,o,o,t,o,o,o,o,o,o,o,o,t,o,o,o,t,o,o,t,o,o,o,t,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,o,o,t,o,o,o,t,t,o,o,o,o,o,o,o,o,o,o,o,t,o,o,t,o,o,o,t,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,t,o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,t,o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,t,o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,t,o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,o,t,o,o,t],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,o,t,o,o,t],
                  &[o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,t,t,t,o,t,o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,t,t,t,o,t],
                  &[o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,t,t,t,o,t,o,o,o,o,o,t,t,t,t,t,o,o,o,o,o,t,t,t,o,t],
                  &[o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,t,t,t,o,t,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,t,t,t,o,t],
                  &[t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o,t,o,o,o,t,o,o,o,o,o,o,o,o,o,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,o,t,o,o,t,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o],
                  &[o,t,t,t,t,o,o,o,o,o,o,t,o,t,o,o,o,o,o,o,o,t,t,t,t,o,o,o,o,o,o,t,o,t,o,o,o,o,o,o],
                  &[t,o,o,o,o,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o,t,o,o,o,o,o,o,o,o,o,o,t,t,t,o,o,o,o,o,o]];

    iterate_a(&ocean, 2);
    //iterate_b(&ocean, 2);
    recurse(&ocean, 2);
}

fn test_5() {
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

    iterate_a(&ocean, 3);
    //iterate_b(&ocean, 3);
    recurse(&ocean, 3);
}

fn test_6() {
    let t = true;
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

    iterate_a(&ocean, 3);
    //iterate_b(&ocean, 3);
    recurse(&ocean, 3);
}