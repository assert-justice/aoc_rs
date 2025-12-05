
fn parse(txt: &str) -> Vec<(usize, usize)>{
    let mut lines = txt.lines();
    let mut ranges = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {break;}
        let line: Vec<usize> = line.split('-').map(|n|n.parse().unwrap()).collect();
        ranges.push((line[0], line[1]));
    }
    ranges
}

pub fn in_range(val: usize, range: (usize, usize)) -> bool{
    let (min, max) = range;
    val >= min && val <= max
}

fn can_combine(a: (usize, usize), b: (usize, usize)) -> Option<(usize, usize)>{
    let (min_a, max_a) = a;
    let (min_b, max_b) = b;
    // Check if either range contains the other. If so, return the enclosing range.
    let c = in_range(min_a, b);
    let d = in_range(max_a, b);
    let e = in_range(min_b, a);
    let f = in_range(max_b, a);
    if c && d{return Some(b);}
    if e && f{return Some(a);}
    // Check if there is an overlap. If so return the maximum max and minimum min.
    if c || d || e || f{
        let min = if min_a < min_b{min_a} else{min_b};
        let max = if max_a > max_b{max_a} else{max_b};
        return Some((min, max))
    }
    None
}

fn find_combinable(ranges: &Vec<(usize, usize)>) -> Option<(usize, usize, (usize, usize))>{
    let len = ranges.len();
    for start in 0..(len-1) {
        for idx in (start+1)..len {
            if let Some(range) = can_combine(ranges[start], ranges[idx]) {
                // println!("{:?} {:?} {:?}", ranges[start], ranges[idx], range);
                return Some((start, idx, range));
            }
        }
    }
    None
}

fn weld_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // combine ranges
    // println!("{:?}", ranges);
    while let Some((start, idx, range)) = find_combinable(&ranges) {
        ranges.remove(idx);
        ranges.remove(start);
        ranges.push(range);
        // println!("{:?}", ranges);
    }
    ranges
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let ranges = parse(txt);
    let ranges= weld_ranges(ranges);
    let res = ranges.into_iter().fold(0, |acc,(min,max)|acc+max-min+1);
    format!("{:?}", res)
}
