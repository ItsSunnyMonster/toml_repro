use toml::Table;
use serde::Deserialize;

fn main() {
}

#[derive(Deserialize)]
struct Deserialized {
    test: u32
}

trait Dyn {
    type Deserialized: for<'de> Deserialize<'de>;
}

trait TypeErasedDyn {
    fn deserialize(table: &Table);
}

impl<T> TypeErasedDyn for T where T: Dyn {
    fn deserialize(table: &Table) {
        let deserialized: T::Deserialized = table.try_into().unwrap();
    }
}