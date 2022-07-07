enum MyEnum {
    Text(String),
    Number(u8),
}

fn f(v: MyEnum) {
    match v {
        MyEnum::Text(s) => println!("This one is string: {}", s),
        MyEnum::Number(n) => println!("This one is number: {}", n),
    }
}

fn main() {
    let t = MyEnum::Text(String::from("123"));
    let n = MyEnum::Number(7);

    f(t);
    f(n);
}
