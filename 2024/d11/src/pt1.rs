
fn blink(data: Vec<u64>) -> Vec<u64>{
    let mut out = Vec::new();
    for x in data{
        let str = x.to_string();
        if x == 0{
            out.push(1);
        }
        else if str.len() % 2 == 0{
            let a = &str[0..(str.len()/2)];
            let b = &str[(str.len()/2)..];
            out.push(a.parse().unwrap());
            out.push(b.parse().unwrap());
        }
        else{
            out.push(x * 2024);
        }
    }
    out
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    for line in txt.lines(){
        let mut xs = line.split(' ')
            .map(|s|s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        // println!("{:?}", xs);
        // let xs = blink(xs);
        // println!("{:?}", xs);
        for _ in 0..25{
            xs = blink(xs);
        }
        println!("{}", xs.len());
        break;
    }
}
