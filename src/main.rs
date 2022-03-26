use std::thread;
use std::time::Instant;


mod foobar;
use foobar::{Foobar, Incasement};


fn main() {
    let incasement = Incasement::new(Foobar::new(1,1));
    let now = Instant::now();    

    let mut incase = incasement.clone();
    let test_push = thread::spawn(move || {
        for i in 0..2 {
            incase.push(Foobar::new(i, i));
        }
    });

    let mut debug = incasement.clone();
    let test_update = thread::spawn(move || {
        for i in 0..2 {
            debug.update(i, i + 2);
        }
    });


    // initialized test
    let start_time = now.elapsed();
    
    test_push.join().unwrap();
    let push_time = now.elapsed();
    // after multiple pushes
    println!("Test Push Time    : {:?}", push_time - start_time);
    
    test_update.join().unwrap();

    // after multiple updates
    println!("Test Update Time  : {:?}", now.elapsed() - push_time);
}
