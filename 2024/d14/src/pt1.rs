
fn modulo(a: i64, b: i64) -> i64{
    ((a % b) + b) % b
}
struct Robot{
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

struct Puzzle{
    width: i64,
    height: i64,
    robots: Vec<Robot>,
}

impl Puzzle {
    pub fn step(&mut self){
        for r in &mut self.robots{
            r.x += r.dx;
            r.y += r.dy;
            if r.x < 0 || r.x >= self.width{
                r.x = modulo(r.x, self.width);
            }
            if r.y < 0 || r.y >= self.height{
                r.y = modulo(r.y, self.height);
            }
        }
    }
    pub fn get_factor(&self) -> usize{
        let mut quads = [0, 0, 0, 0];
        let mid_x = self.width/2;
        let mid_y = self.height/2;
        for r in &self.robots{
            let mut idx = 0;
            if r.x == mid_x{continue;}
            if r.y == mid_y{continue;}
            if r.x > mid_x {idx += 1};
            if r.y > mid_y {idx += 2};
            quads[idx] += 1;
        }
        quads[0]*quads[1]*quads[2]*quads[3]
    }
    pub fn print(&self){
        for y in 0..self.height{
            for x in 0..self.width{
                let mut c = 0;
                for r in &self.robots{
                    if r.x == x && r.y == y{
                        c += 1;
                    }
                }
                if c == 0{
                    print!(".");
                }
                else{
                    print!("{c}");
                }
            }
            print!("\n");
        }
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let mut puzzle = Puzzle{
        width: 101,
        height: 103,
        robots: Vec::new(),
    };
    for line in txt.lines(){
        let mut line = line.split(' ');
        let mut pos = line.next().unwrap()[2..].split(',');
        let x = pos.next().unwrap().parse::<i64>().unwrap();
        let y = pos.next().unwrap().parse::<i64>().unwrap();
        let mut vel = line.next().unwrap()[2..].split(',');
        let dx = vel.next().unwrap().parse::<i64>().unwrap();
        let dy = vel.next().unwrap().parse::<i64>().unwrap();
        puzzle.robots.push(Robot{x,y,dx,dy});
    }
    puzzle.print();
    for _ in 0..100{
        // println!("step: {i}");
        // puzzle.print();
        puzzle.step();
    }
    println!("finally:");
    puzzle.print();
    let factor = puzzle.get_factor();
    println!("{factor}");
}
