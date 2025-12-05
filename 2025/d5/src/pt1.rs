
fn parse(txt: &str) -> (Vec<(usize, usize)>, Vec<usize>){
    let mut lines = txt.lines();
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {break;}
        let line: Vec<usize> = line.split('-').map(|n|n.parse().unwrap()).collect();
        ranges.push((line[0], line[1]));
    }
    while let Some(line) = lines.next() {
        ingredients.push(line.parse().unwrap());
    }
    (ranges, ingredients)
}

pub fn in_range(val: usize, ranges: &Vec<(usize, usize)>) -> bool{
    for (min, max) in ranges {
        if val >= *min && val <= *max{
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let (ranges, ingredients) = parse(txt);
    let res = ingredients.iter().filter(|i| in_range(**i, &ranges)).count();
    format!("{}", res)
}
