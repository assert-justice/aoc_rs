use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Indicator{
    data: u64,
    len: usize,
}

#[allow(dead_code)]
impl Indicator {
    fn new() -> Self{
        Self { data: 0, len: 0 }
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
        let mut me = Self::new();
        for c in txt.chars() {
            me.push(c == '#');
        }
        me
    }
    fn test() -> bool{
        // let txt = ".###.#";
        // let i = Indicator::parse(txt);
        // println!("{}", i.data);
        // println!("{}", i.len);
        // println!("{}", i.is_lit());
        // let i = Indicator::parse("###");
        // println!("{}", i.data);
        // println!("{}", i.len);
        // println!("{}", i.is_lit());
        // let mut i = Indicator::parse("...");
        // i.set(1, true);
        // println!("data: {}", i.data);
        // println!("len: {}", i.len);
        // println!("should be true: {}", i.get(1));
        // // let mut i = Indicator::parse("###");
        // i.set(1, false);
        // println!("data: {}", i.data);
        // println!("len: {}", i.len);
        // println!("should be false: {}", i.get(1));
        // i.toggle(1);
        // println!("data: {}", i.data);
        // println!("len: {}", i.len);
        // println!("should be true: {}", i.get(1));
        // i.toggle(1);
        // println!("data: {}", i.data);
        // println!("len: {}", i.len);
        // println!("should be false: {}", i.get(1));
        // i.set(1, false);
        // i.toggle(1);
        // println!("data: {}", i.data);
        // println!("len: {}", i.len);
        // println!("should be false: {}", i.get(1));
        // i.toggle(0);
        // println!("data: {}", i.data);
        // println!("len: {}", i.len);
        // let mut i = Indicator::parse("...");
        // i.toggle(1);
        // println!("{}", i.data);
        // println!("{}", i.len);
        // println!("{}", i.is_lit());
        true
    }
}

#[allow(dead_code)]
struct Machine{
    start_indicator: Indicator,
    buttons: Vec<Vec<u8>>,
    power: Vec<u8>,
    seen: HashSet<Indicator>,
}

impl Machine {
    fn new(indicator: Indicator, buttons: Vec<Vec<u8>>, power: Vec<u8>) -> Self{
        Self{
            start_indicator: indicator,
            buttons,
            power,
            seen: HashSet::new(),
        }
    }
    fn parse(txt: &str) -> Self{
        let mut strings: Vec<&str> = txt.split(' ').collect();
        let i_string = strings.remove(0);
        let p_string = strings.pop().unwrap();
        let mut indicator = Indicator::new();
        let mut buttons = Vec::new();
        for idx in 0..(i_string.len()-1) {
            indicator.push(i_string.chars().nth(idx+1).unwrap() == '#');
        }
        for s in strings {
            let s = &s[1..(s.len()-1)];
            let button = s.split(',').map(|num|num.parse().unwrap()).collect();
            buttons.push(button);
        }
        let power = p_string[1..(p_string.len()-1)].split(',').map(|num|num.parse().unwrap()).collect();
        Self::new(indicator, buttons, power)
    }
    fn press(&self, mut indicator: Indicator, button: &Vec<u8>) -> Indicator{
        for idx in button {
            // indicator[*idx as usize] = !indicator[*idx as usize];
            indicator.toggle(*idx as usize);
        }
        indicator
    }
    fn get_min_presses(&mut self) -> usize{
        // implement dfs. try pushing each button and repeat. gonna need to memoize this bitch.
        let mut open: Vec<Indicator> = vec![self.start_indicator];
        let mut presses = 0;
        loop {
            let mut next = Vec::new();
            for indicator in &open {
                if indicator.is_lit() {return presses;}
                for b in &self.buttons {
                    // println!("fuck {}", indicator.data);
                    let temp = self.press(*indicator, b);
                    // println!("you {}", temp.data);
                    if self.seen.contains(&temp) {continue;}
                    self.seen.insert(temp);
                    next.push(temp);
                }
            }
            println!("len: {}", next.len());
            debug_assert!(next.len() > 0);
            presses += 1;
            open = next;
            break 0;
        }
    // println!("{:?}", open);
    // 0
    }
}

fn parse(txt: &str) -> Vec<Machine>{
    txt.lines().map(|l|Machine::parse(l)).collect()
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    debug_assert!(Indicator::test());
    let mut res = 0;
    for mut m in data {
        let count = m.get_min_presses();
        println!("{count}");
        res += count;
        break;
    }
    format!("{}",res)
}
