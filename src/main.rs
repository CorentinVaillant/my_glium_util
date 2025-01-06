mod test;

fn main(){
    println!("starting tests");
    let now = std::time::Instant::now();

    let elapsed_time = now.elapsed();
    println!("Matrix test OK ✅, took {}", elapsed_time.as_secs_f64());
}