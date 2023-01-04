mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            println!("hosting: add to waitlist")
        }

        fn seat_at_table() {
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
