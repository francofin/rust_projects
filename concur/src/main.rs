use std::thread;
use std::time::Duration; //Pause Thread
use std::sync::mpsc; //for channels
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use num::{BigUint, One};
use std::time::Instant; //measure time for nfunc to run. 


fn factorial(num: u32) -> BigUint{
    if num == 0 || num ==1{
        return BigUint::one()
    } else {
        (1..=num).map(BigUint::from).reduce(|acc, x| acc*x).unwrap() //mapped from u32 to big unit. Reduce returns an option.
    }
}

fn multithreaded_fact(num: u32) -> BigUint{
    if num == 0 || num ==1{
        return BigUint::one()
    } else {
        //into par iterator from rayon
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc*x) //mapped from u32 to big unit. Reduce returns an option.
        //This one is fropm Rayon is different and takes an idetiity in adition to reduce formula. 
    }
}

fn main() {

    println!("Factorial of number is {}", factorial(3));
    println!("Factorial of number is {}", multithreaded_fact(3));
    // let (transmitter, receiver) = mpsc::channel();

    let now = Instant::now();
    factorial(50000);
    println!("{:.2?}", now.elapsed());

    let now2 = Instant::now();
    multithreaded_fact(50000);
    println!("{:.2?}", now2.elapsed());

    let counter = Arc::new(Mutex::new(0));
    let mut arc_handles = vec![];

    for _ in 0..8{
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap(); //lock to gain access to counter and wunwrap val indside
            *num +=1;
            }
        ); //releases lock here

        arc_handles.push(handle);
    }

    for hand in arc_handles{
        hand.join().unwrap();
    }

    println!("{:?}", counter.lock().unwrap());


    let (transmitter, receiver) = mpsc::sync_channel(1000); //sepcify how many values the queue can hold. send is a blocoking op when queue is filled. 
    let val = String::from("Transmitting");

    let rc1 = Arc::new(String::from("Test"));
    let rc2 = rc1.clone();

//send takes ownership and recv also takes ownership at that point. 
    std::thread::spawn(move || {
        rc2;
        }
    );
    
    // let msg = receiver.recv().unwrap();

    // println!("Message {:?}", msg);


    let tx = transmitter.clone(); //A second producer
    std::thread::spawn(move || {
        let vec = vec![String::from("Transmittign Again"), String::from("Oh No Again"), String::from("What is this")];

        for val in vec{
            transmitter.send(val).unwrap(); //unwrap as it returns a result
            }
        }
    );

    std::thread::spawn(move || {
        let vec = vec![String::from("Transmittign from clone Again"), String::from("Oh No Clone Again"), String::from("What is this Clone")];

        for val in vec{
            tx.send(val).unwrap(); //unwrap as it returns a result
            }
        }
    );

    for rec in receiver{
        println!("{:?}", rec)
    }


    //spawn returns a join handle.
    let handle = std::thread::spawn(move || {
        println!("Hello, world!");
        }
    );



    handle.join().unwrap(); //Form of blocking. 

    println!("Hello, From Main!"); //told main to wait until threasd has completed task.

//Move fforces closure to take ownership of the values passed into the spawn thread. 
    let v = vec![1,2,23];
    // let handle = std::thread::spawn(move || {
    //     println!("{:?}", v);
    //     }
    // );


    let mut thread_handles = Vec::new();
    for e in v {
        thread_handles.push(std::thread::spawn(move || println!("Thread {:?}", e)));
    }

    println!("Hello, From Main!");

    for thread in thread_handles{
        thread.join().unwrap(); 
    };




//Sleep the thread.
    thread::sleep(Duration::from_secs(1));
    
}
