use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct PrintError;

impl fmt::Display for PrintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrintError!")
    }
}

fn print_val<T: fmt::Display>(v: &T) -> Result<(), PrintError> {
    println!("Print {}", v);
    Ok(())
}

fn main() {
    let p = Point{x: 6, y: 7};
    print_val(&p).unwrap();
    print_val(&p).unwrap();

    let five = Box::new(5);
    print_val(five.as_ref()).unwrap();
    print_val(five.as_ref()).unwrap();

    println!("{}", PrintError);
}
