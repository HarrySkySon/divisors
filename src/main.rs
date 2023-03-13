use std::io;
use std::thread;

fn main() {
    // Read input from user
    let mut input = String::new();
    println!("Enter a positive integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Split work across threads
    let num_threads = 4; // Change this to adjust the number of threads used
    let mut handles = vec![];
    let chunk_size = n / num_threads;
    let mut start = 1;
    for i in 0..num_threads {
        let end = if i == num_threads - 1 {
            n
        } else {
            start + chunk_size
        };
        let handle = thread::spawn(move || {
            let mut divisors = vec![];
            for j in start..=end {
                if n % j == 0 {
                    divisors.push(j);
                }
            }
            divisors
        });
        handles.push(handle);
        start = end + 1;
    }

    // Collect results from threads
    let mut divisors = vec![];
    for handle in handles {
        let thread_divisors = handle.join().unwrap();
        divisors.extend(thread_divisors);
    }

    // Print results
    divisors.sort();
    divisors.dedup();
    println!("Divisors of {}: {:?}", n, divisors);
}
