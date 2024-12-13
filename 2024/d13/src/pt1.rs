
type Vec2 = (i64, i64);
#[derive(Debug)]
struct Machine{
    a_button: Vec2,
    b_button: Vec2,
    goal: Vec2,
}

impl Machine{
    pub fn get_cost(&self) -> i64{
        let mut res = i64::MAX;
        let (ax, ay) = self.a_button;
        let (bx, by) = self.b_button;
        let (gx, gy) = self.goal;
        for a in 0..101{
            for b in 0..101{
                let x = a * ax + b * bx;
                let y = a * ay + b * by;
                if x != gx || y != gy{continue;}
                let cost = a * 3 + b;
                if cost < res{
                    res = cost;
                }
            }
        }
        if res == i64::MAX{
            return 0;
        }
        res
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
        let goal = parse_line(group[2]);
        out.push(Machine{
            a_button,
            b_button,
            goal
        });
    }
    out
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let machines = parse(txt);
    let mut total = 0;
    for m in machines{
        let cost = m.get_cost();
        // println!("cost: {}, {:?}", cost, m);
        total += cost;
    }
    println!("{total}");
}
