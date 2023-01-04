mod foo {
    #[derive(Debug)]
    pub struct Bar {
        pub i_public: u8,
        i_private: String
    }

    impl Bar {
        pub fn new() -> Bar {
            Bar {
                i_public: 1,
                i_private: String::from("foo")
            }
        }
    }
}

pub fn run() {
    let item = foo::Bar::new();
    println!("{:?}", item);
}
