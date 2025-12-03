use std::collections::HashMap;


struct Puzzle{
    mem: HashMap<(usize,usize), usize>,
    xs: Vec<char>,
}

impl Puzzle {
    fn solve(txt: &str) -> usize{
        let mut puzzle = Self{
            mem: HashMap::new(),
            xs: Vec::new(),
        };
        let batteries = Puzzle::parse(txt);
        let mut res = 0;
        for b in batteries {
            puzzle.mem.clear();
            puzzle.xs = b;
            let val = puzzle.get_max_charge(12);
            // println!("{val}");
            res += val;
            // println!("{:?}", puzzle.mem);
            // break;
        }
        res
    }

    fn parse(txt: &str) -> Vec<Vec<char>>{
        let mut res = Vec::new();
        for line in txt.lines() {
            res.push(line.chars().collect());
        }
        res
    }

    fn get_max_charge(&mut self, num: usize) -> usize{
        self.spam(num, 0)
    }

    fn join(a: char, b: usize) -> usize{
        let mut s = String::new();
        s.push(a);
        s.push_str(b.to_string().as_str());
        s.parse().unwrap()
    }

    fn spam(&mut self, num: usize, start: usize) -> usize{
        // println!("{} {}",num, start);
        if let Some(v) = self.mem.get(&(num, start)) {
            *v
        }
        else {
            let v = self.eggs(num, start);
            self.mem.insert((num,start), v);
            v
        }
    }
    
    fn eggs(&mut self, num: usize, start: usize) -> usize{
        debug_assert!(start <= self.xs.len());
        debug_assert!(num <= self.xs.len());
        if num == 0{return 0;}
        if start == self.xs.len(){return 0;}
        let xs = &self.xs[start..];
        if num == 1{
            return xs.iter().map(|x|x.to_string().parse().unwrap()).max().unwrap();
        }
        else if num == xs.len(){
            let res = xs.iter().collect::<String>().parse().unwrap();
            return res;
        }
        let a = Self::join(xs[0], self.spam(num - 1, start + 1));
        let b = self.spam(num, start + 1);
        let res = if a > b {a} else{b};
        res
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    println!("pt 2: {}", Puzzle::solve(txt));
}
