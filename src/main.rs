fn main() {
    let cpu_cores = std::thread::available_parallelism().unwrap().get();
    println!("Hello, world!, {}", cpu_cores);
}
