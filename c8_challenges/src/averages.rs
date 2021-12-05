// Given a list of integers, use a vector and return the mean (the average value), 
// median (when sorted, the value in the middle position), and mode (the value that 
// occurs most often; a hash map will be helpful here) of the list.
// let mut numbers: Vec<i32> = vec![0, 4, 3, 7, 7, 8];

use std::collections::HashMap;

fn avg(nums_vec: &Vec<i32>) -> f32 {
    let sum: i32 = nums_vec.iter().sum();
    // for x in numbers.iter(){
    //     sum += x;
    // }
    sum as f32 / nums_vec.len() as f32
}
fn median(nums_vec: &mut Vec<i32>) -> f32 {
    nums_vec.sort();

    if (nums_vec.len() % 2)==0 {
        let i_left = nums_vec.len()/2-1; 
        let i_right = nums_vec.len()/2 ;
        (nums_vec[i_left]+nums_vec[i_right]) as f32 / 2.0

    } else {
        nums_vec[(nums_vec.len()/2)] as f32
    }
}
fn mode(nums_vec: &Vec<i32>) -> Vec<i32> {
    let mut val_counts = HashMap::new();
    for x in nums_vec.iter() {
        *val_counts.entry(x).or_insert(0) += 1;
    };
    let max_value = val_counts.values().cloned().max().unwrap_or(0);
    
    val_counts
        .into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,5,3,7,7,8];
    println!("Mean {}", avg(&numbers));
    println!("Median {}", median(&mut numbers));
    println!("Mode {:?}", mode(&numbers));
}