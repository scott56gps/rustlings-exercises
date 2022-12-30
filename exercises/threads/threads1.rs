// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Arc is needed to use a variable that could be modified by a thread, in the main thread
    // Mutex is used because we want to mutate the variable inside another thread
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    let handle = thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(250));

            // Since we want to mutate the variable, we must:
            //  1. Obtain a lock on the variable
            //  2. Mutate the underlying data the lock gives us (as a Result type)
            if let Ok(mut unwrapped_job_status) = status_shared.lock() {
                unwrapped_job_status.jobs_completed += 1;
                println!("Complted job {}!", i);
            }
        }
    });

    // While we {wait}...
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }

    // We perform work
    handle.join().unwrap();
}
