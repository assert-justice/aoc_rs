
struct Sleigh{
    a: Vec<u64>,
    b: Vec<u64>,
    c: Vec<u64>,
}

impl Sleigh {
    fn get_legroom(&self) -> u64{
        self.a.iter().sum()
    }
    fn get_qe(&self) -> u64{
        self.a.iter().fold(1, |acc,x|acc*x)
    }
    fn is_balanced(&self) -> bool{
        let legroom = self.get_legroom();
        return  legroom == self.b.iter().sum() && legroom == self.c.iter().sum();
    }
}

fn parse(txt: &str) -> Vec<u64>{
    txt.lines().map(|l|l.parse().unwrap()).collect()
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    format!("{:?}", data.len())
}
