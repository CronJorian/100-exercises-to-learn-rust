// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return 0;
    }

    let num_threads = 8;
    let chunk_size = v.len().div_ceil(num_threads);

    let mut handles = Vec::new();

    for (i, chunk) in v.chunks(chunk_size).enumerate() {
        println!("Spawning thread {} with size {}", i + 1, chunk.len());
        let v_chunk = chunk.to_vec();
        let handle = thread::spawn(move || v_chunk.into_iter().sum::<i32>());
        handles.push(handle);
    }

    handles
        .into_iter()
        .map(|handle| {
            let subsum = handle.join().unwrap_or_default();
            println!("Subsum is {}", subsum);
            subsum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
