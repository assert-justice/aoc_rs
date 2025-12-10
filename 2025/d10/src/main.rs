use std::time::Instant;

mod pt1;
mod pt2;
fn main() {
    let example = include_str!("../example.txt");
    let txt = include_str!("../text.txt");
    let now = Instant::now();
    println!("pt 1 example: {}", pt1::solve(&example));
    println!("pt 1 result: {}", pt1::solve(&txt));
    // println!("pt 2 example: {}", pt2::solve(&example));
    // println!("pt 2 result: {}", pt2::solve(&txt));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
