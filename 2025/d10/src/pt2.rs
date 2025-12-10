
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Indicator{
    data: Vec<usize>,
}

enum IndicatorStatus{
    Under,
    Equal,
    Over,
}

#[allow(dead_code)]
impl Indicator {
    fn new(len: usize) -> Self{
        Self { data: vec![0; len] }
    }
    fn inc(&mut self, idx: usize){
        self.data[idx] += 1;
    }
    fn parse(txt: &str) -> Self{
        let mut me = Self::new(0);
        for c in txt.split(',') {
            me.data.push(c.parse().unwrap());
        }
        me
    }
    fn get_status(&self, target: &Indicator) -> IndicatorStatus{
        debug_assert!(self.data.len() == target.data.len());
        let mut is_equal = true;
        for idx in 0..self.data.len() {
            let a = self.data[idx];
            let b = target.data[idx];
            if a > b{return IndicatorStatus::Over;}
            if a < b{is_equal = false;}
        }
        if is_equal{
            IndicatorStatus::Equal
        } else {
            IndicatorStatus::Under
        }
    }
}

#[allow(dead_code)]
struct Machine{
    goal_indicator: Indicator,
    buttons: Vec<Vec<usize>>,
    power: Vec<usize>,
}

impl Machine {
    fn new(goal_indicator: Indicator, buttons: Vec<Vec<usize>>, power: Vec<usize>) -> Self{
        Self{
            goal_indicator,
            buttons,
            power,
        }
    }
    fn parse(txt: &str) -> Self{
        let mut strings: Vec<&str> = txt.split(' ').collect();
        strings.remove(0);
        let p_string = strings.pop().unwrap();
        let indicator = Indicator::parse(&p_string[1..(p_string.len()-1)]);
        let mut buttons = Vec::new();
        for s in strings {
            let s = &s[1..(s.len()-1)];
            let button = s.split(',').map(|num|{
                // println!("num: {}", num);
                num.parse().unwrap()
            }).collect();
            buttons.push(button);
        }
        let power = p_string[1..(p_string.len()-1)].split(',').map(|num|num.parse().unwrap()).collect();
        Self::new(indicator, buttons, power)
    }
    fn press(&self, mut indicator: Indicator, button: &Vec<usize>) -> Indicator{
        for idx in button {
            indicator.inc(*idx);
        }
        indicator
    }
    fn get_min_presses(&mut self) -> usize{
        // implement dfs
        let start_indicator = Indicator::new(self.goal_indicator.data.len());
        let mut open: Vec<Indicator> = vec![start_indicator.clone()];
        let mut presses = 0;
        loop {
            let mut next = Vec::new();
            for indicator in &open {
                // let indicator = *indicator;
                match indicator.get_status(&self.goal_indicator) {
                    IndicatorStatus::Equal => {return presses;},
                    IndicatorStatus::Over => {continue;}
                    IndicatorStatus::Under => {},
                }
                for b in &self.buttons {
                    let temp = self.press(indicator.clone(), b);
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
        println!("count: {count}");
        res += count;
    }
    format!("{}",res)
}
