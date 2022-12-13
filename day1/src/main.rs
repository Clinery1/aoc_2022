use std::{
    fs::read_to_string,
    cmp::Ordering,
};


#[derive(PartialEq,Eq,Ord,Debug)]
struct Elf {
    total:u64,
    items:Vec<u64>,
}
impl PartialOrd for Elf {
    fn partial_cmp(&self,other:&Self)->Option<Ordering> {
        return self.total.partial_cmp(&other.total);
    }
}


fn main() {
    let mut elves=vec![];
    let mut input=read_to_string("input").unwrap();
    input.push('\n');
    let mut sum=0;
    let mut items=vec![];
    for line in input.lines() {
        if line.len()==0&&items.len()>0 {
            elves.push(Elf {
                total:sum,
                items,
            });
            items=vec![];
            sum=0;
        } else {
            let num=line.parse::<u64>().unwrap();
            sum+=num;
            items.push(num);
        }
    }
    dbg!(sum,items);
    elves.sort();
    println!("Total calories of the elf with the most calories: {}",elves.last().unwrap().total);
    println!("Total calories of the top 3 elves with the most calories: {}",
        elves
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i,v)|if i<3 {Some(v.total)} else {None})
            .sum::<u64>()
    );
}
