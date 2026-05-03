use ferris_says::say;
use std::io::{self, BufWriter, Write, stdout};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello, World!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(&message, width, &mut writer).unwrap();
}

#[test]
fn immutable_variable() {
    // let x = 1;
    // x = 2; //! error
}

#[test]
fn variable_ownership() {}

#[test]
fn stack_and_heap_fetcher() {
    let a = String::from("hi");
    let mut b = a.clone();
    b += "2";

    println!("{}", a + " " + &b);

    let c = 10;
    let mut d = c;
    d += 10;

    println!("{c} {d}");
}

#[test]
fn clone_heap_data() {}

#[test]
fn macros() {
    let v = vec![1, 2, 3, 4, 5];
    let mut vs: Vec<&str> = vec!["one", "two", "tree", "four"];

    vs.push("five");
    // let vs =
    for n in v {
        let c = vs[n - 1];
        let m = String::from("Number: ") + c + " " + &n.to_string();
        println!("{m}");

        // String Literal: &s
        // "+" Behavior: String + &str
    }
}

#[test]
fn buffer() -> io::Result<()> {
    let mut o = io::stdout();

    writeln!(o, "To buffer 1 ")?;
    writeln!(o, "To buffer 2 ")?;

    o.flush()?;

    return Ok(());
}

#[test]
fn buffer2() -> io::Result<()> {
    let mut o = io::stdout();

    match writeln!(o, "Buffer 1") {
        Ok(()) => {}
        Err(error) => return Err(error),
    }

    match o.flush() {
        Ok(()) => {}
        Err(error) => return Err(error),
    }

    return Ok(());
}
