#[allow(dead_code)]
mod front_of_house {
    // the `mod` keyword defines a module
    // the contents of a module defines other items - structs, enums, consts, traits and functions
    // this module is available at `crate::front_of_house`
    mod hosting {
        // we can define as many levels of submodule as needed
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

