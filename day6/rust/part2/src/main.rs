use std::fs::read_to_string;


#[derive(Debug)]
struct Transmission {
    buf_offset:usize,
    remainder:String,
}
impl Transmission {
    fn from_buffer(buf:String)->Self {
        let mut tmp=[' ';14];
        let mut iter=buf.chars().enumerate();
        let mut offset=0;
        for (i,c) in &mut iter {
            for i in 0..(tmp.len()-1) {
                tmp[i]=tmp[i+1];
            }
            tmp[tmp.len()-1]=c;
            let mut good=true;
            for (i,a) in tmp.iter().enumerate() {
                for (j,b) in tmp.iter().enumerate() {
                    if i!=j {
                        good&=a!=b;
                    }
                    good&=*b!=' ';
                }
            }
            if good {
                offset=i;
                break;
            }
        }
        let remainder=iter.map(|(_,c)|c).collect::<String>();
        return Self {
            buf_offset:offset,
            remainder,
        };
    }
}


fn main() {
    let input=read_to_string("../../input").unwrap();
    let t=Transmission::from_buffer(input);
    dbg!(&t);
}
