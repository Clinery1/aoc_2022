use std::{
    fs::read_to_string,
    collections::HashMap,
};


enum Node<'a> {
    Dir(HashMap<&'a str,Self>,usize),
    File(usize),
}
impl<'a> Node<'a> {
    fn add_node(&mut self,path:&[&'a str],node:Self) {
        match self {
            Self::Dir(nodes,size)=>{
                match node {
                    Self::File(s)=>*size+=s,
                    _=>{},
                }
                if path.len()==1 {
                    nodes.insert(path[0],node);
                } else {
                    nodes
                        .get_mut(path[0])
                        .unwrap()
                        .add_node(&path[1..],node);
                }
            },
            _=>panic!(),
        }
    }
    fn inspect<A,F:FnMut(&mut A,&Self)+Clone>(&self,acc:&mut A,mut f:F) {
        f(acc,self);
        match self {
            Self::Dir(files,_)=>{
                for file in files.values() {
                    file.inspect(acc,f.clone());
                }
            },
            _=>{},
        }
    }
}


fn main() {
    let input=read_to_string("../../input").unwrap();
    let mut root=Node::Dir(HashMap::new(),0);
    let mut path=Vec::new();
    for line in input.lines() {
        let items=line.split(' ').collect::<Vec<_>>();
        if items[0].starts_with('$') {
            // Command
            if items[1]=="cd" {
                match items[2] {
                    ".."=>{
                        path.pop();
                    },
                    "/"=>{},
                    path_section=>{
                        path.push(path_section);
                        root.add_node(&path,Node::Dir(HashMap::new(),0));
                    },
                }
            }
        } else if items[0].starts_with("dir") {
        } else {
            let size=items[0].parse::<usize>().unwrap();
            let name=items[1];
            path.push(name);
            root.add_node(&path,Node::File(size));
            path.pop();
        }
    }
    let mut sum=0;
    root.inspect(&mut sum,|sum,node|match node {
        Node::Dir(_,size)=>if *size<=100000 {*sum+=size},
        _=>{},
    });
    println!("Sum of dirs: {}",sum);
}
