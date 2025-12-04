use std::time::Instant;

mod pt1;
mod pt2;
fn main() {
    let txt = include_str!("../text.txt");
    let now = Instant::now();
    println!("pt 1: {}", pt1::solve(&txt));
    println!("pt 2: {}", pt2::solve(&txt));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
