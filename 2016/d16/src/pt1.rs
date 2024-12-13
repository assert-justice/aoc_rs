
fn parse(txt: &str) -> Vec<u8>{
    txt.bytes().map(|v|v-48).collect()
}

fn dragon(data: Vec<u8>) -> Vec<u8>{
    let mut a = data;
    let mut b = a.clone();
    b.reverse();
    b = b.into_iter().map(|v|if v == 0{1} else{0}).collect();
    a.push(0);
    for v in b{
        a.push(v);
    }
    a
}

fn checksum(data: Vec<u8>) -> Vec<u8>{
    let mut res = Vec::new();
    for i in (0..data.len()).step_by(2){
        let a = data[i];
        let b = data[i+1];
        if a == b{
            res.push(1);
        }
        else{
            res.push(0);
        }
    }
    if res.len() % 2 == 0{
        return checksum(res);
    }
    res
}

fn print(data: Vec<u8>){
    for v in data{
        print!("{v}");
    }
    print!("\n");
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let len = 272;
    for line in txt.lines(){
        let mut data = parse(line);
        while data.len() < len {
            data = dragon(data);
        }
        while data.len() > len {
            data.pop();
        }
        let c = checksum(data);
        print(c);
    }
}
