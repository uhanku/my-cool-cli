use rand::Rng;

#[test]
fn run() {
    let ex1 = ex1(15);
    println!("ex1: {ex1}");

    let ex2i = rand::thread_rng().gen_range(0..=3);
    let ex2vec = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let ex2 = ex2(ex2vec[ex2i].clone());

    println!("ex2: {ex2}");

    let ex3vec = vec![
        Command::Quit,
        Command::Move { x: 50, y: 100 },
        Command::Write(String::from("Vinicius")),
        Command::ChangeColor(43, 30, 89),
    ];
    ex3(ex3vec[ex2i].clone());

    let ex4i = rand::thread_rng().gen_range(0..=1);
    let ex4vec = vec![Command::Quit, Command::ChangeColor(1, 2, 3)];
    let ex4 = ex4(ex4vec[ex4i].clone());

    println!("ex4: {}", ex4);

    let ex5x = rand::thread_rng().gen_range(-10..=10);
    let ex5y = rand::thread_rng().gen_range(-10..=10);
    let ex5 = ex5(ex5x, ex5y);

    println!("ex5: {ex5}");

    let ex6x = rand::thread_rng().gen_range(-10..=10);
    let ex6y = rand::thread_rng().gen_range(-10..=10);
    let ex6 = ex6(ex6x, ex6y);

    println!("ex6: {ex6}");

    let ex7x = rand::thread_rng().gen_range(60..=100);
    let ex7b = ex7b(ex7x);
    let ex7a = ex7a(ex7x);

    println!("ex7b: {ex7b}");
    println!("ex7a: {ex7a}");

    let msg1 = Message::Text(String::from("Vinicius"));
    let msg2 = Message::Number(67);

    ex8(&msg1);
    ex8(&msg1);
    ex8(&msg2);
    ex8(&msg2);

    let ex9vec = vec![
        LoginState::LoggedOut,
        LoginState::LoggedIn(String::from("Vinicius")),
    ];
    let ex9x = rand::thread_rng().gen_range(0..=1);
    ex9v1(ex9vec[ex9x].clone());
    ex9v2(ex9vec[ex9x].clone());

    let ex10vec = vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90];
    ex10(ex10vec);
}

fn ex1<'a>(n: i32) -> &'a str {
    match n {
        0 => "zero",
        1..=9 => "small positive",
        -9..=-1 => "small negative",
        10..=100 => "medium positive",
        -100..=-10 => "medium negative",
        _ => "large",
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn ex2<'a>(d: Direction) -> &'a str {
    match d {
        Direction::Up => "Moving Up",
        Direction::Down => "Moving Down",
        Direction::Left => "Moving Left",
        Direction::Right => "Moving Right",
    }
}

#[derive(Clone)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
fn ex3(command: Command) {
    match command {
        Command::Quit => println!("ex3: Quitting"),
        Command::Move { x, y } => println!("ex3: Moving to {}, {}", x, y),
        Command::Write(str) => println!("ex3: Writing: {}", str),
        Command::ChangeColor(a, b, c) => {
            println!("ex3: Changing color to rgb({}, {}, {})", a, b, c)
        }
    }
}

fn ex4(command: Command) -> bool {
    match command {
        Command::Quit => true,
        _ => false,
    }
}

fn ex5<'a>(x: i32, y: i32) -> &'a str {
    match (x, y) {
        (0, 0) => "origin",
        (_, 0) => "on x-axis",
        (0, _) => "on y-axis",
        (x, y) if x == y => "diagonal",
        (x, y) if x == -y => "opposite diagonal",
        _ => "regular point",
    }
}

fn ex6<'a>(php: i32, ehp: i32) -> &'a str {
    match (php, ehp) {
        (php, ehp) if php <= 0 && ehp <= 0 => "draw",
        (php, _) if php <= 0 => "player lost",
        (_, ehp) if ehp <= 0 => "player won",
        (_, _) => "battle continues",
    }
}

fn ex7b<'a>(score: u8) -> &'a str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    }
}

fn ex7a<'a>(score: u8) -> &'a str {
    match score {
        e if e >= 90 => "A",
        e if e >= 80 => "B",
        e if e >= 70 => "C",
        e if e >= 60 => "D",
        _ => "F",
    }
}

enum Message {
    Text(String),
    Number(i32),
}

fn ex8(msg: &Message) {
    match msg {
        Message::Number(e) => println!("Number: {e}"),
        Message::Text(e) => println!("Text: {e}"),
    }
}

#[derive(Clone)]
enum LoginState {
    LoggedOut,
    LoggedIn(String),
}

fn ex9v1(state: LoginState) -> String {
    match state {
        LoginState::LoggedIn(e) => String::from("Hello, ") + &e,
        LoginState::LoggedOut => String::from("Please log in"),
    }
}

fn ex9v2(state: LoginState) -> String {
    if let LoginState::LoggedIn(e) = state {
        String::from("Hello") + &e
    } else {
        String::from("Please log in")
    }
}

fn ex10(mut v: Vec<i32>) {
    while let Some(v) = v.pop() {
        print!(" {}", v);
    }
    println!();
}
