macro_rules! day {
    ( $( $x:ident ),* ) => {
        $(
            println!("{}: Part One Solution: {}", stringify!($x), $x::solve(Part::One));
            println!("{}: Part Two Solution: {}", stringify!($x), $x::solve(Part::Two));
        )*
    };
}

mod utils;
use utils::Part;

mod day1;

fn main() {
    day!(day1);
}
