
fn next_row(xs: &Vec<char>) -> Vec<char>{
    let mut res = Vec::new();
    for i in 0..xs.len(){
        let left = xs.get(i.wrapping_sub(1)).unwrap_or(&'.') == &'^';
        let right = xs.get(i+1).unwrap_or(&'.') == &'^';
        let center = xs[i] == '^';
        if left && center && !right{res.push('^');}
        else if !left && center && right{res.push('^');}
        else if left && !center && !right{res.push('^');}
        else if !left && !center && right{res.push('^');}
        else{res.push('.');}
    }
    res
}

fn count_row(xs: &Vec<char>) -> usize{
    xs.into_iter().filter(|c|**c == '.').count()
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let num_rows = 400000;
    let mut count = 0;
    for line in txt.lines(){
        let mut row = line.chars().collect::<Vec<char>>();
        for _ in 0..num_rows{
            count += count_row(&row);
            row = next_row(&row);
        }
    }
    println!("{count}");
}
