use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Indicator{
    data: u64,
    len: usize,
}

#[allow(dead_code)]
impl Indicator {
    fn new(len: usize) -> Self{
        Self { data: 0, len}
    }
    fn push(&mut self, val: bool){
        if !val{
            self.len += 1;
            return;
        }
        let val: u64 = 1 << self.len;
        self.data |= val;
        self.len += 1;
    }
    fn set(&mut self, idx: usize, val: bool){
        let mask: u64 = 1 << idx;
        if val{
            self.data |= mask;
        }
        else {
            self.data &= !mask;
        }
    }
    fn get(&mut self, idx: usize) -> bool{
        let mask: u64 = 1 << idx;
        (self.data & mask) > 0
    }
    fn toggle(&mut self, idx: usize){
        let mask: u64 = 1 << idx;
        self.data ^= mask;
    }
    fn is_lit(&self) -> bool{
        let mask: u64 = (1 << self.len) - 1;
        mask == self.data
    }
    fn parse(txt: &str) -> Self{
        let mut me = Self::new(0);
        for c in txt.chars() {
            debug_assert!(c == '#' || c == '.');
            me.push(c == '#');
        }
        me
    }
}

#[allow(dead_code)]
struct Machine{
    goal_indicator: Indicator,
    buttons: Vec<Vec<usize>>,
    power: Vec<usize>,
    seen: HashSet<Indicator>,
}

impl Machine {
    fn new(goal_indicator: Indicator, buttons: Vec<Vec<usize>>, power: Vec<usize>) -> Self{
        Self{
            goal_indicator,
            buttons,
            power,
            seen: HashSet::new(),
        }
    }
    fn parse(txt: &str) -> Self{
        let mut strings: Vec<&str> = txt.split(' ').collect();
        let i_string = strings.remove(0);
        let p_string = strings.pop().unwrap();
        let indicator = Indicator::parse(&i_string[1..(i_string.len()-1)]);
        let mut buttons = Vec::new();
        for s in strings {
            let s = &s[1..(s.len()-1)];
            let button = s.split(',').map(|num|num.parse().unwrap()).collect();
            buttons.push(button);
        }
        let power = p_string[1..(p_string.len()-1)].split(',').map(|num|num.parse().unwrap()).collect();
        Self::new(indicator, buttons, power)
    }
    fn press(&self, mut indicator: Indicator, button: &Vec<usize>) -> Indicator{
        for idx in button {
            indicator.toggle(*idx as usize);
        }
        indicator
    }
    fn get_min_presses(&mut self) -> usize{
        // implement dfs
        let start_indicator = Indicator::new(self.goal_indicator.len);
        let mut open: Vec<Indicator> = vec![start_indicator];
        self.seen.insert(start_indicator);
        let mut presses = 0;
        loop {
            let mut next = Vec::new();
            for indicator in &open {
                let indicator = *indicator;
                if indicator == self.goal_indicator {return presses;}
                for b in &self.buttons {
                    let temp = self.press(indicator, b);
                    if self.seen.contains(&temp) {continue;}
                    self.seen.insert(temp);
                    next.push(temp);
                }
            }
            debug_assert!(next.len() > 0);
            presses += 1;
            open = next;
        }
    }
}

fn parse(txt: &str) -> Vec<Machine>{
    txt.lines().map(|l|Machine::parse(l)).collect()
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    // debug_assert!(Indicator::test());
    // let res = data[0].get_min_presses();
    let mut res = 0;
    for mut m in data {
        let count = m.get_min_presses();
        // println!("{count}");
        res += count;
    }
    format!("{}",res)
}
