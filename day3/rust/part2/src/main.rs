use std::{
    collections::HashSet,
    fs::read_to_string,
};


fn main() {
    let contents=read_to_string("../../input").unwrap();
    let mut chunks=Vec::new();
    let mut array=["";3];
    for (i,line) in contents.lines().enumerate() {
        if i>0&&i%3==0 {
            chunks.push(array);
            array=["";3];
        }
        array[i%3]=line;
    }
    chunks.push(array);
    let mut sum=0;
    for chunk in chunks.into_iter() {
        let mut a=HashSet::new();
        for c in chunk[0].chars() {
            a.insert(c);
        }
        let mut b=HashSet::new();
        for c in chunk[1].chars() {
            b.insert(c);
        }
        'c:for c in chunk[2].chars() {
            if a.contains(&c)&&b.contains(&c) {
                sum+=get_prio(c);
                break 'c;
            }
        }
    }
    println!("Sum: {}",sum);
}
fn get_prio(c:char)->usize {
    match c {
        'a'..='z'=>(c as u8-('a' as u8-1)) as usize,
        'A'..='Z'=>((c as u8-('A' as u8-1))+26) as usize,
        _=>unreachable!(),
    }
}
