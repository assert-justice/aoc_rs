fn compact(line: &str) ->String{
    let mut res = String::new();
    let chars: Vec<char> = line.trim().chars().collect();
    for idx in 0..chars.len() {
        if chars[idx] == ' ' && chars[idx-1] == ' '{continue;}
        res.push(chars[idx]);
    }
    res
}

struct Problem{
    op: char,
    xs: Vec<usize>,
}

impl Problem {
    fn calc(&self) -> usize{
        debug_assert!(self.op == '+' || self.op == '*');
        if self.op == '+'{
            return  self.xs.iter().sum();
        }
        return self.xs.iter().fold(1, |acc, x|acc*x);
    }
}

fn parse(txt: &str) -> Vec<Problem>{
    let mut lines: Vec<Vec<String>> = txt.lines().map(|s|compact(s).split(' ').map(|s|s.to_string()).collect()).collect();
    let ops = lines.pop().unwrap();
    let mut res = Vec::new();
    for idx in 0..ops.len() {
        // println!("{}",ops[idx].len());
        let op = ops[idx].chars().nth(0).unwrap();
        let mut xs = Vec::new();
        for f in 0..lines.len(){
            xs.push(lines[f][idx].parse().unwrap());
        }
        res.push(Problem { op, xs });
    }
    res
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    let data = data.iter().map(|p|p.calc());
    format!("{}", data.sum::<usize>())
}
