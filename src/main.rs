#[derive(Debug)]
enum NAMES {
    SUBRAT,
    KALLI,
}

impl std::fmt::Display for NAMES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let name1 = NAMES::SUBRAT;
    let name2 = NAMES::KALLI;
    println!(
        "
    {} 
    {}
    ",
        name1, name2
    );
}
