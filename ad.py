import os
import sys
import subprocess

main_src = '''use std::time::Instant;

mod pt1;
mod pt2;
fn main() {
    let txt = include_str!("../text.txt");
    let now = Instant::now();
    pt1::solve(&txt);
    // pt2::solve(&txt);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
'''

src = '''
#[allow(dead_code)]
pub fn solve(txt: &str){
    for line in txt.lines(){
        println!("{line}");
    }
}
'''

def main():
    args = sys.argv
    if len(args) < 3:
        print("Not enough arguments")
        return
    year = args[1]
    day = args[2]
    if not os.path.isdir(year):
        os.mkdir(year)
    dir = f"d{day}/"
    if os.path.isdir(f"{year}/d{day}"):
        print("Project already exists")
        return
    os.chdir(year)
    print(os.getcwd())
    subprocess.run(f"cargo new d{day}".split(' '))
    with open(dir + '/src/main.rs', 'w') as f:
        f.write(main_src)
    with open(dir + '/src/pt1.rs', 'w') as f:
        f.write(src)
    with open(dir + '/src/pt2.rs', 'w') as f:
        f.write(src)
    with open(dir + '/text.txt', 'w') as f:
        f.write('')

if __name__ == "__main__":
    main()