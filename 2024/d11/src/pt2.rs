use std::collections::HashMap;

fn blink(x: u64) -> Vec<u64>{
    let mut out = Vec::new();
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
    out
}

fn count(mem: &mut HashMap<(u64, usize), usize>, val: u64, step: usize) -> usize{
    if step == 0{
        return 1;
    }
    if let Some(v) = mem.get(&(val,step)) {
        return *v;
    }
    let mut total = 0;
    for x in blink(val){
        total += count(mem, x, step-1);
    }
    mem.insert((val,step), total);
    total
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    for line in txt.lines(){
        let xs = line.split(' ')
            .map(|s|s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let mut mem: HashMap<(u64, usize), usize> = HashMap::new();
        let mut total = 0;
        for x in xs{
            total += count(&mut mem, x, 75);
        }
        println!("{total}");
        // println!("{:?}", mem);
        break;
    }
}
