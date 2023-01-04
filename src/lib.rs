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

    pub mod bar {
        pub fn println() {
            println!("foo bar");
        }
    }
}

use crate::foo::bar::println;

mod baz {
    pub fn function() {
        super::println()
    }
}

use std::fmt;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    todo!()
}

fn function2() -> IoResult<()> {
    todo!()
}

pub fn run() {
    baz::function()
}
