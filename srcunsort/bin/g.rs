mod rw {
    pub struct Bar<T> {
        pub contents: T
    }

    impl<T> Bar<T> {
        pub fn new(contents: T) -> Bar<T> {
            Bar {
                contents
            }
        }

    }
}

fn main() {
    let t = rw::Bar::new("hello world");
    println!("{}", t.contents);
}
