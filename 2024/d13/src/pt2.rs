
type Vec2 = (i64, i64);
#[derive(Debug)]
struct Machine{
    a_button: Vec2,
    b_button: Vec2,
    goal: Vec2,
}

/*
gx = ax * a + bx * b
gy = ay * a + by * b

// solve both for a
a = (gx - bx * b) / ax = (gy - by * b) / ay
// rearrange
gx * ay - bx * b * ay = gy * ax - by * b * ax
// isolate b
gx * ay - gy * ax = bx * b * ay - by * b * ax = b * (bx * ay - by * ay)
// solve for b
(gx * ay - gy * ax)/(bx * ay - by * ay) = b
 */

impl Machine{
    // pub fn get_cost(&self) -> i64{
    //     let (ax, ay) = self.a_button;
    //     let (bx, by) = self.b_button;
    //     let (gx, gy) = self.goal;
    //     let b = (gx * ay - gy * ax)/(bx * ay - by * ay);
    //     let a = (gx - bx * b) / ax;
    //     println!("a: {a}, b: {b}");
    //     a * 3 + b
    // }
    pub fn get_cost2(&self) -> i64{
        // Shamelessly ~~stolen~~ adapted from https://github.com/happyhacks/aoc2024-rs/blob/main/src/bin/13.rs
        // Solve the system of equations:
        // a.x * x + b.x * y = prize.x
        // a.y * x + b.y * y = prize.y
        let (ax,ay) = self.a_button;
        let (bx,by) = self.b_button;
        let (prize_x, prize_y) = self.goal;
        let det = ax * by - ay * bx;
        if det == 0 {
            return 0; // No unique solution
        }
    
        let x_num = prize_x * by - prize_y * bx;
        let y_num = ax * prize_y - ay * prize_x;
    
        if x_num % det != 0 || y_num % det != 0 {
            return 0; // No integer solution
        }
    
        let x = x_num / det;
        let y = y_num / det;
        x * 3 + y
    }
}

fn parse_line(line: &str) -> Vec2{
    let mut line = line.split(": ");
    line.next();
    let line = line.next().unwrap().split(", ").collect::<Vec<&str>>();
    let a = &line[0][2..].parse::<i64>().unwrap();
    let b = &line[1][2..].parse::<i64>().unwrap();
    (*a,*b)
}

fn parse(txt: &str) -> Vec<Machine>{
    let mut out = Vec::new();
    let mut groups = Vec::new();
    groups.push(Vec::new());
    for line in txt.lines(){
        if line == ""{
            groups.push(Vec::new());
            continue;
        }
        groups.last_mut().unwrap().push(line);
    }
    for group in groups{
        let a_button = parse_line(group[0]);
        let b_button = parse_line(group[1]);
        let (gx, gy) = parse_line(group[2]);
        out.push(Machine{
            a_button,
            b_button,
            goal: (gx + 10000000000000, gy + 10000000000000),
            // goal: (gx, gy),
        });
    }
    out
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let machines = parse(txt);
    let mut total = 0;
    for m in machines{
        let cost = m.get_cost2();
        println!("cost: {}, {:?}", cost, m);
        total += cost;
    }
    println!("{total}");
}
