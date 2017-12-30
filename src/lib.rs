#[macro_use]
extern crate helix;

ruby! {
    class Rstrie {
        def insert(value: String) -> String {
            println!("LOG: {:?}", value);
            value
        }
    }
}
