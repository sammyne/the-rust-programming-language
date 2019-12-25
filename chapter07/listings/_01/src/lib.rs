/// define a module by starting with the `mod` keyword and then specify the
/// name of the module (in this case, `front_of_house`) and place curly brackets
/// around the body of the module. Inside modules, we can have other modules, as in
/// this case with the modules `hosting` and `serving`. Modules can also hold
/// definitions for other items, such as structs, enums, constants, traits
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
