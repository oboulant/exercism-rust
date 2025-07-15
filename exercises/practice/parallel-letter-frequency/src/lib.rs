use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// Test runner does not accept `crayon` crate, so cannot
// implement a par_chunks().map().reduce()

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Flatten all input into a single string
    let input_chars: Vec<char> = input.concat().chars().collect();

    if input_chars.is_empty() {
        return HashMap::new();
    }

    // make sure no char at the end of the vector is left
    let chunk_size = input_chars.len().div_ceil(worker_count);

    // Each thread will deverse (in a thread safe way) it result in it
    let global_res: Arc<Mutex<HashMap<char, usize>>> =
        Arc::new(Mutex::new(HashMap::<char, usize>::new()));

    // For joining the thread once they are spawn
    let mut handles = Vec::new();

    for chunk in input_chars.chunks(chunk_size) {
        // BEcause it is moved into spawn thread
        let chunk = chunk.to_owned();

        // Each thread access the global HashMap protected by a Mutex, via a thread safe reference
        let global_res_ar = Arc::clone(&global_res);

        let handle = thread::spawn(move || {
            let mut local_res: HashMap<char, usize> = HashMap::new();

            // local count for chunk
            for c in chunk {
                if c.is_alphabetic() {
                    *local_res.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                }
            }

            // sync with global `global_res`
            let mut global_map = global_res_ar.lock().unwrap();
            for (ch, count) in local_res {
                *global_map.entry(ch).or_insert(0) += count;
            }
        });
        handles.push(handle);
    }

    // Join threads
    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(global_res)
        .expect("Arc has more than one strong reference")
        .into_inner() // get what is inside the Mutex
        .expect("Mutex was poisoned")
}

// To run the benches
// rustup run nightly cargo bench
