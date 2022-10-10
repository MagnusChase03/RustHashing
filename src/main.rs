use std::{env, thread, fs};
use std::sync::{Arc, Mutex};

// Hash function
fn hash(file_name: &String) -> u32 {

    let contents = fs::read(file_name).expect("File not found");

    // Add up all the bytes in file
    let mut sum: u32 = 0;
    for byte in &contents {

        sum += *byte as u32;

    }

    sum

}

fn main() {

    // Grab command line args
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {

        println!("Usage: ./rust-hashing [file-1] [file-2] ...")

    } else {

        args.remove(0);
        
        let size = args.len();

        // Keep handles for threads and create an Arc for threads to access args
        let files = Arc::new(Mutex::new(args));
        let mut handles = vec![];
        
        for _ in 0..size {

            // Create another refrence to args for thread
            let files = Arc::clone(&files);
            let handle = thread::spawn(move || {

                // Grab the next arg
                let file = files.lock().unwrap().remove(0);
                println!("{} is {}", &file, hash(&file));

            });

            handles.push(handle);

        }

        // Wait for all threads to complete
        for handle in handles {

            handle.join().unwrap();

        }

    }
}
