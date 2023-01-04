mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hosting: add to waitlist")
        }

        pub fn seat_at_table() {
            println!("hosting: seat at table")
        }
    }

    mod serving {
        fn take_order() {
            println!("serving: take order")
        }

        fn serve_order() {
            println!("serving: serve order")
        }

        fn take_payment() {
            println!("serving: take payment")
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::seat_at_table();
}
