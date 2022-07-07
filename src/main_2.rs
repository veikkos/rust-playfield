mod libi {
    pub struct Value(pub u8);

    pub trait Named {
        fn get_name(&self) -> String;
    }

    impl Named for Value {
        fn get_name(&self) -> String {
            String::from("Byte")
        }
    }

    impl Value {
        pub fn inc(&self) -> u8 {
            self.0 + 1
        }
    }
}

fn print_name(n: &dyn libi::Named) {
    println!("Hello, world!, {}", n.get_name());
}

fn main() {
    let v = libi::Value(6);
    let n: &dyn libi::Named = &v;
    print_name(n);
}
