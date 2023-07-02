mod mod_1;

fn main() {
    println!("print ’a’~’Z’ : ");
    mod_1::print_a_to_Z();
    
    print!("--------------------\n");

    println!("print ’A’~’z’ : ");
    mod_1::print_A_to_z();
}
