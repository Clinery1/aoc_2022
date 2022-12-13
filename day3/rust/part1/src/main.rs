use std::{
    collections::HashSet,
    fs::read_to_string,
};


fn main() {
    let contents=read_to_string("../../input").unwrap();
    let mut prio_dup_list=Vec::new();
    for line in contents.lines() {
        let size=line.len()/2;
        let mut left=HashSet::new();
        let mut right=HashSet::new();
        for c in line.chars().enumerate().filter_map(|(i,c)|if i<size {Some(c)} else {None}) {
            left.insert(c);
        }
        for c in line.chars().skip(size) {
            right.insert(c);
        }
        'test:for l in left {
            for r in right.iter() {
                if l==*r {
                    let num=match l {
                        'a'..='z'=>l as u8-('a' as u8-1),
                        'A'..='Z'=>(l as u8-('A' as u8-1))+26,
                        _=>unreachable!(),
                    };
                    prio_dup_list.push(num as usize);
                    break 'test;
                }
            }
        }
    }
    let sum:usize=prio_dup_list.into_iter().sum();
    println!("Sum: {}",sum);
}
