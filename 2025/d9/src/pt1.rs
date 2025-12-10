
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vec2I{
    x: i64,
    y: i64,
}

impl Vec2I {
    fn new(x: i64, y: i64) -> Self{
        Self { x, y }
    }
    fn parse(txt: &str) -> Self{
        let args: Vec<i64> = txt.split(',').map(|s|s.parse().unwrap()).collect();
        Vec2I::new(args[0], args[1])
    }
    fn sub(&self, v: Vec2I) -> Vec2I{
        Vec2I { x: self.x-v.x, y: self.y-v.y }
    }
}

fn parse(txt: &str) -> Vec<Vec2I>{
    txt.lines().map(|l|Vec2I::parse(l)).collect()
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    let mut area = 0;
    for i in 0..(data.len()-1) {
        for f in (i+1)..data.len() {
            let a = data[i];
            let b = data[f];
            let c = a.sub(b);
            let a = ((c.x+1) * (c.y+1)).abs();
            if a > area{area = a;}
        }
    }
    format!("{}", area)
}
