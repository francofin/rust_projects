use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    println!("Hello, world!");

    let mut numbers: Vec<i32> = vec![];

    numbers.push(5);
    numbers.push(6);
    numbers.push(7);
    numbers.push(8);


    let top_value = numbers.pop(); //returns an option of type T, return None or Some(T) 
    println!("{:?}", top_value);

    let first = numbers.first(); //Returns an option of type T, None or Some. Can also do .last, .first_mut and .last_mut which borrows mutable references.
    println!("{:?}", first);
    let length_of_vec = numbers.len(); // .is_empty() returns a bool if empty. 
    println!("{:?}", length_of_vec);

    //We can add via insert and pass an index to insert and numbver to insert. 

    numbers.insert(0, 10);
    numbers.insert(3, 14);
    numbers.insert(5, 15);


    // numbers.remove(3); 

    numbers.sort();

    println!("{:?}", numbers);

    numbers.reverse();

    println!("{:?}", numbers);

    //We can shuffle a vector, 

    numbers.shuffle(&mut thread_rng());
    println!("{:?}", numbers);

    let mut bheap = BinaryHeap::new();
    bheap.push(5);
    bheap.push(20);
    bheap.push(4);

    println!("{:?}", bheap);

    let mut hash_map = HashMap::new();
    //has len and isempty methods. 
    hash_map.insert(1,1);
    hash_map.insert(5,2);
    hash_map.insert(30,3);
    hash_map.insert(40,4);
    hash_map.insert(70,8);

    println!("{:?}", hash_map);

    println!("{:?}", hash_map.contains_key(&40));
    println!("{:?}", hash_map.get(&40));

    // let removed_value = hash_map.remove(&40)
    // let removed_pair = hash_map.remove_entry(&40)
    // hash_map.clear() clears map.


    let mut hash_set = HashSet::new();
    //has len and isempty methods. 
    hash_set.insert(1);
    hash_set.insert(5);
    hash_set.insert(30);
    hash_set.insert(32);
    hash_set.insert(32);


    println!("{:?}", hash_set);

    for i in hash_set.iter(){
        println!("Iter: {:?}", i);
    }

    // hash_set.remove(&30);


    let mut hash_set_two = HashSet::new();
    //has len and isempty methods. 
    hash_set_two.insert(1);
    hash_set_two.insert(5);
    hash_set_two.insert(38);
    hash_set_two.insert(52);


    for i in hash_set.intersection(&hash_set_two){
        println!("Iter: {:?}", i);
    }

    //Alt method using & 
    let intersection = &hash_set & &hash_set_two;
    for i in intersection{
        println!("Iter Short Way: {:?}", i);
    }

    let union_set = &hash_set | &hash_set_two;
    for i in union_set{
        println!("Union Short Way: {:?}", i);
    }

    let diff = &hash_set - &hash_set_two;
    for i in diff{
        println!("Diff Short Way: {:?}", i);
    }


}
