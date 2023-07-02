mod mod_2;
pub use mod_2::print_A_to_z;

pub fn print_a_to_Z() {
    for ch in ('Z'..='a').rev() {
        println!("{}", ch);
    }
}
