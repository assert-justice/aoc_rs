use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone)]
struct Puzzle{
    width: i32,
    height: i32,
    gx: i32,
    gy: i32,
    dir: u8,
    // solid: HashSet<(i32, i32)>,
    // visited: HashSet<(i32, i32, u8)>,
}

impl Puzzle{
    pub fn parse(txt: &str) -> (Self, HashSet<(i32, i32)>){
        let lines: Vec<&str> = txt.lines().collect();
        let height: i32 = lines.len().try_into().unwrap();
        let width: i32 = lines[0].chars().count().try_into().unwrap();
        let mut gx: i32 = 0;
        let mut gy: i32 = 0;
        let mut solid = HashSet::new();
        for y in 0..lines.len(){
            let line: Vec<char> = lines[y].chars().collect();
            for x in 0..line.len(){
                let c = line[x];
                match c{
                    '^' => {
                        gx = x.try_into().unwrap();
                        gy = y.try_into().unwrap();
                    },
                    '#' => {
                        solid.insert((x.try_into().unwrap(),y.try_into().unwrap()));
                    },
                    _ => {},
                }
            }
        }
        (Puzzle{
            gx,
            gy,
            width,
            height,
            dir: 0,
            // solid,
            // visited: HashSet::new(),
        },solid)
    }
    fn is_solid(solid: &HashSet<(i32, i32)>, x: i32, y: i32) -> bool{
        solid.contains(&(x,y))
    }
    fn does_guard_loop(mut puzzle: Puzzle, solid: &HashSet<(i32, i32)>, sx: i32, sy: i32) -> bool{
        let dirs:[(i32, i32);4] = [(0,-1), (1,0), (0, 1), (-1,0)];
        let mut visited = HashSet::new();
        loop {
            if puzzle.gx < 0 || puzzle.gx >= puzzle.width {return false;}
            if puzzle.gy < 0 || puzzle.gy >= puzzle.height {return false;}
            let pos = (puzzle.gx, puzzle.gy, puzzle.dir);
            if visited.contains(&pos) {return true;};
            visited.insert(pos);
            let (dx, dy) = dirs[usize::from(puzzle.dir)];
            let x = puzzle.gx + dx;
            let y = puzzle.gy + dy;
            if Puzzle::is_solid(&solid, x, y) || (x == sx && y == sy){
                puzzle.dir += 1;
                if puzzle.dir > 3{puzzle.dir = 0;}
                continue;
            }
            puzzle.gx = x;
            puzzle.gy = y;
        }
    }
    pub fn solve(txt: &str){
        let (puzzle, solid) = Puzzle::parse(txt);
        let mut tally = 0;
        for x in 0..puzzle.width{
            for y in 0..puzzle.height{
                if Puzzle::is_solid(&solid,x, y) {continue;}
                else if x == puzzle.gx && y == puzzle.gy{continue;}
                if Puzzle::does_guard_loop(puzzle.clone(), &solid, x, y){tally += 1;}
            }
        }
        println!("{tally}");
    }
}

fn main() {
    let input = include_str!("../text.txt");
    let now = Instant::now();
    Puzzle::solve(input);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
