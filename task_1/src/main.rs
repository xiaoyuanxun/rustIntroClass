// 使用enum
enum MyEnum {
    Type1(Type1),
    Type2(Type2),
    Type3(Type3),
}

struct Type1;
struct Type2;
struct Type3;

impl Type1 {
    fn method1(&self) {
        println!("Type1 method called.");
    }
}

impl Type2 {
    fn method2(&self) {
        println!("Type2 method called.");
    }
}

impl Type3 {
    fn method3(&self) {
        println!("Type3 method called.");
    }
}

fn use_enum() {
    let vec: Vec<MyEnum> = vec![MyEnum::Type1(Type1), MyEnum::Type2(Type2), MyEnum::Type3(Type3)];

    for item in vec {
        match item {
            MyEnum::Type1(t) => t.method1(),
            MyEnum::Type2(t) => t.method2(),
            MyEnum::Type3(t) => t.method3(),
        }
    }
}

// 使用 Trait Object
trait MyTrait {
    fn method(&self);
}

impl MyTrait for Type1 {
    fn method(&self) {
        println!("Type1 method called.");
    }
}

impl MyTrait for Type2 {
    fn method(&self) {
        println!("Type2 method called.");
    }
}

impl MyTrait for Type3 {
    fn method(&self) {
        println!("Type3 method called.");
    }
}

fn use_trait_object() {
    let vec: Vec<Box<dyn MyTrait>> = vec![Box::new(Type1), Box::new(Type2), Box::new(Type3)];

    for item in vec {
        item.method();
    }
}

/// 使用Enum来包裹一些类型可以将枚举中的类型放进一个Vec中，
/// 但是类型仅限于Enum定义的几个枚举类型。
/// 而使用Trait Object方法，
/// 可以使实现了该 Trait 的类型都可以以Box<dyn MyTrait>的形式放入Vec中，
/// 实现更大的灵活性。
fn main() {
    use_enum();
    use_trait_object();
}
