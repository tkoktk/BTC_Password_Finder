use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;
use crate::engine::PasswordIterator;
use crate::config::{read_private_key_file, write_found_password};
use bip38::Decrypt;
use std::time::Duration;

pub struct Coordinator {
    private_key: String,
    iterator: Option<PasswordIterator>,
    worker_count: usize,
}

impl Coordinator {
    pub fn new() -> Self {
        let private_key = read_private_key_file();
        println!("priv key {}", private_key);
        let iterator = Some(PasswordIterator::new()); // Wrap in Option
        
        let worker_count = std::thread::available_parallelism()
            .unwrap()
            .get() * 2;
        
        println!("Coordinator initialized with {} workers", worker_count);
        
        Self {
            private_key,
            iterator,
            worker_count
        }
    }
    
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        //State shared between workers
        let found = Arc::new(AtomicBool::new(false));
        let total_tested = Arc::new(AtomicUsize::new(0));
        
        //Take ownership of iterator
        let mut iterator = self.iterator.take()
            .expect("Iterator already consumed");
        
        //Create channels
        let mut worker_senders: Vec<Sender<Vec<String>>> = Vec::new();
        let (result_tx, result_rx) = channel::<String>();
        
        let mut handles = Vec::with_capacity(self.worker_count);
        
        //Spawn workers, num of CPU cores * 1.5
        for worker_id in 0..self.worker_count {
            let (work_tx, work_rx) = channel::<Vec<String>>();
            worker_senders.push(work_tx);
            
            let encrypted_key = self.private_key.clone();
            let result_tx = result_tx.clone();
            let found = Arc::clone(&found);
            let total_tested = Arc::clone(&total_tested);
            
            let handle = thread::spawn(move || {
                let mut local_tested = 0usize;
                
                while let Ok(batch) = work_rx.recv() {
                    if found.load(Ordering::Relaxed) {
                        break;
                    }
                    
                    let batch_size = batch.len();
                    
                    for password in batch {
                        if local_tested & 0x3F == 0 && found.load(Ordering::Relaxed) {
                            return;
                        }

                        println!("Worker {} testing: '{}'", worker_id, password);
                        
                        if encrypted_key.decrypt(password.as_str()).is_ok() {
                            found.store(true, Ordering::SeqCst);
                            let _ = result_tx.send(password);
                            return;
                        }
                        
                        local_tested += 1;
                    }
                    
                    total_tested.fetch_add(batch_size, Ordering::Relaxed);
                }
            });
            
            handles.push(handle);
        }
        
        //Generator thread
            let found_gen = Arc::clone(&found);
            let generator = thread::spawn(move || {
            let mut worker_idx = 0;
            let mut batch_size = 25_000;
            let num_workers = worker_senders.len();
            let mut total_sent = 0;
                
            while !found_gen.load(Ordering::Relaxed) {
                match iterator.next_batch(batch_size) {
                    Some(batch) => {
                        let batch_len = batch.len();
                        println!("Generator sending batch of {} to worker {}", batch_len, worker_idx);
                        
                        if worker_senders[worker_idx].send(batch).is_err() {
                            println!("Generator: Worker {} disconnected", worker_idx);
                            break;
                        }
                        
                        total_sent += batch_len;
                        worker_idx = (worker_idx + 1) % num_workers;
                    }
                    None => {
                        println!("Generator: Password space exhausted after {} passwords", total_sent);
                        break;
                    }
                }
            }
            
            println!("Generator shutting down. Total sent: {}", total_sent);
            
            //signal workers to stop
            drop(worker_senders);
        });

        //Monitor thread
        let found_monitor = Arc::clone(&found);
        let total_monitor = Arc::clone(&total_tested);
        let monitor = thread::spawn(move || {
        let mut last_count = 0;
        
        while !found_monitor.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_secs(5));
            
            let current_count = total_monitor.load(Ordering::Relaxed);
            if current_count > last_count {
                let elapsed = start.elapsed();
                let rate = current_count as f64 / elapsed.as_secs_f64();
                
                println!("⚡ {:>12} passwords | {:>10.0} p/s | {:>8.1?} elapsed", 
                        current_count, rate, elapsed);
                
                last_count = current_count;
                }
            }
        });
        
        // Drop result_tx after spawning all threads so workers can send results
        drop(result_tx);
        
        //Wait for result
        match result_rx.recv_timeout(Duration::from_secs(300)) {
            Ok(password) => {
                let elapsed = start.elapsed();
                let total = total_tested.load(Ordering::Relaxed);
                
                println!("\n ============= PASSWORD FOUND ============= ");
                println!("Password: {}", password);
                println!("Time: {:.2?}", elapsed);
                println!("Tested: {} passwords", total);
                println!("Speed: {:.0} passwords/second", total as f64 / elapsed.as_secs_f64());
                
                write_found_password(&password)?;
                
                found.store(true, Ordering::SeqCst);
                generator.join().ok();
                monitor.join().ok();
                for handle in handles {
                    handle.join().ok();
                }
                
                return Ok(());
            }
            Err(_) => {
                //No password found
                found.store(true, Ordering::SeqCst);
            }
        }
        
        //Cleanup
        generator.join().ok();
        monitor.join().ok();
        for handle in handles {
            handle.join().ok();
        }
        
        let total = total_tested.load(Ordering::Relaxed);
        println!("\nPassword not found");
        println!("Total tested: {} passwords", total);
        
        Ok(())
    }
}