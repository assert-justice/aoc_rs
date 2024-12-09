
struct Entry{
    value: u64,
    xs: Vec<u64>
}

impl Entry {
    fn eval(&self, acc: u64, mut stack: Vec<u64>) -> u64{
        let v = stack.pop();
        if let Some(value) = v {
            return self.eval(acc + value, stack.clone()) + self.eval(acc * value, stack);
        }
        else{
            if acc == self.value {return 1;}
            else{return 0;}
        }
    }
    pub fn check(&self) -> u64{
        let mut stack = self.xs.clone();
        stack.reverse();
        let v = stack.pop().unwrap();
        self.eval(v, stack)
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let mut total = 0;
    for line in txt.lines(){
        let temp: Vec<&str> = line.split(": ").collect();
        let value = temp[0].parse::<u64>().unwrap();
        let xs: Vec<u64> = temp[1].split(' ').map(|v|v.parse::<u64>().unwrap()).collect();
        let entry = Entry{value, xs};
        // println!("{}", entry.check());
        if entry.check() > 0{total += value;}
    }
    println!("{total}");
}
