use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct MyNumber(i32);

impl Add for MyNumber {
    type Output = MyNumber;

    fn add(self, other: MyNumber) -> MyNumber {
        MyNumber(self.0 + other.0)
    }
}

trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for MyNumber {
    fn do_something(&self) {
        println!("Doing something with MyNumber: {}", self.0);
    }
}

impl MyTrait for String {
    fn do_something(&self) {
        println!("Doing something with String: {}", self);
    }
}

fn call_method(obj: &dyn MyTrait) {
    obj.do_something();
}

fn main() {
    let num1 = MyNumber(10);
    let num2 = MyNumber(20);
    let result = num1 + num2;
    println!("Result: {:?}", result);

    let string = String::from("Hello, Trait Object!");
    call_method(&num1 as &dyn MyTrait);
    call_method(&string as &dyn MyTrait);
}
