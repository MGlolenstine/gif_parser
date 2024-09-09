use bypar::FromBytes as _;
use models::gif::Gif;

pub mod models;

fn main() {
    let bytes = include_bytes!("../test.gif");

    let gif = Gif::from_bytes(bytes).unwrap();
    dbg!(gif);
}
