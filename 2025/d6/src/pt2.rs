use std::collections::HashMap;

fn flush(op: char, xs: &mut Vec<usize>) -> usize{
    let val = if op == '+'{
        xs.iter().sum()
    }
    else {
        xs.iter().fold(1, |acc,x|acc*x)
    };
    // println!("{val}, {op}, {:?}", xs);
    xs.clear();
    val
}
struct Grid{
    data: HashMap<(usize, usize),char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> char{
        *self.data.get(&(x,y)).unwrap()
    }
    fn solve(&self) ->usize{
        let mut op = '+';
        let mut xs: Vec<usize> = Vec::new();
        let mut res = 0;
        for col in 0..self.width {
            let c = self.get(col, self.height-1);
            if c != ' '{
                let total = flush(op, &mut xs);
                // println!("op: {op}");
                // println!("total: {total}");
                res += total;
                op = c;
            }
            let mut s = String::new();
            for row in 0..(self.height - 1) {
                s.push(self.get(col, row));
            }
            let x = s.trim().parse().unwrap_or(0);
            if x == 0{continue;}
            // println!("intermediate: {x}");
            xs.push(x);
        }
        let total = flush(op, &mut xs);
        // println!("total: {total}");
        res += total;
        res
    }
}

fn parse(txt: &str) -> Grid{
    let mut data = HashMap::new();
    let mut width = 0;
    for (y,line) in txt.lines().enumerate() {
        width = line.len();
        for (x, c) in line.char_indices() {
            data.insert((x,y), c);
        }
    }
    Grid { data, width, height: txt.lines().count() }
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    format!("{}", data.solve())
}
