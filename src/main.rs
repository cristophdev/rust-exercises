pub mod grains;

fn main() {
    println!("Grains on square 1: {}", grains::square(1));
    println!("Total grains on the chessboard: {}", grains::total());
}
