mod sound {
    // fn guitar() {

    // }
    // Nested modules
    pub mod instrument {
        pub fn clarinet() {

        }
        mod woodwind {
            fn clarinet() {

            }
        }
    }
    mod voice {

    }
}


fn main() {
    // Abs path
    crate::sound::instrument::clarinet();
    sound::instrument::clarinet();
}
