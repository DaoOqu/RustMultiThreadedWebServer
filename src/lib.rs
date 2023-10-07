use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        /// Create a new ThreadPool.
        ///
        /// The size is the number of threads in the pool.
        ///
        /// # Panics
        ///
        /// The `new` function will panic if the size is zero.
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create threads and store them in Vec
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}