// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


mod delicious_snacks {
    // TODO: Fix these use statements
    use self::fruits as fruit_mod;
    use self::veggies as veggie_mod;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    pub use self::fruit_mod::PEAR;
    pub use self::veggie_mod::CUCUMBER;
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::PEAR,
        delicious_snacks::CUCUMBER
    );
}
