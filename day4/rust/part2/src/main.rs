use std::{
    fs::read_to_string,
};


struct Assignment {
    start:usize,
    end:usize,
}
impl Assignment {
    fn overlaps(&self,other:&Self)->bool {
        let start_contained=self.start<=other.start&&
            self.end>=other.start;
        let end_contained=self.start<=other.end&&
            self.end>=other.end;
        start_contained||end_contained
    }
}


fn main() {
    let input=read_to_string("../../input").unwrap();
    let mut sum=0;
    for line in input.lines() {
        let pairs=line.split(',')
            .map(|s|{
                let nums=s.split('-').collect::<Vec<_>>();
                let start=nums[0].parse::<usize>().unwrap();
                let end=nums[1].parse::<usize>().unwrap();
                Assignment {start,end}
            })
            .collect::<Vec<_>>();
        if pairs[0].overlaps(&pairs[1])||pairs[1].overlaps(&pairs[0]) {
            sum+=1;
        }
    }
    println!("{} assignments overlap",sum);
}
