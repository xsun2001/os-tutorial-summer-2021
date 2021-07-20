// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU32, Ordering};

struct JobStatus {
    jobs_completed: AtomicU32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: AtomicU32::new(0) });
    for _ in 0..10 {
        let status_shared = status.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst);
        });
    }
    while status.jobs_completed.load(Ordering::Relaxed) < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
