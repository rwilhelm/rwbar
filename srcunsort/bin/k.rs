#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod rw {
    use anyhow::{anyhow, Context, Result};
    use xcb;
    //use xcb::base::Error;

    #[derive(Debug)]
    pub struct Bar {
        pub a: u32
    }

    impl Bar {
        pub fn new(a: u32) -> Result<Bar> {
            let bar = Bar {
                a
            };

            Ok(bar)
        }
    }
}

fn main() {
    let bar = rw::Bar::new(1982).unwrap();
    //dbg!(bar);
    println!("{}", &bar.a);
    
    //let a = match rw::Bar::new(1982) {
    //    Some(bar) => {
    //        println!("{}", bar.a);
    //    },
    //    None => {
    //    }
    //};
}
