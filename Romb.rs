fn main() {
    const SIZE: usize = 5;

    for i in 0..SIZE {
        for j in 0..(SIZE - i - 1) {
            print!(" ");
        }
        for j in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    for i in (0..SIZE - 1).rev() {
        for j in 0..(SIZE - i - 1) {
            print!(" ");
        }
        for j in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}
