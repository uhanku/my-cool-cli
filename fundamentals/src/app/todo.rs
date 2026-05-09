struct TodoItem {
    message: String,
    status: Status,
}

impl TodoItem {
    fn build_status(&self) -> &str {
        match self.status {
            Status::Working => "🛫 Working",
            Status::Completed => "✅ Completed",
        }
    }
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn add(&mut self, item: TodoItem) {
        self.list.push(item);
    }

    fn print(self) {
        for i in 0..self.list.len() {
            println!("=================== TODO {} ===================", i + 1);
            println!("Message: {}", self.list[i].message);
            println!("Status: {}", self.list[i].build_status());
        }
    }
}

enum Status {
    Working,
    Completed,
}

#[test]
fn todo() {
    let mut list = TodoList { list: Vec::new() };

    let item1 = TodoItem {
        message: String::from("Big Workout"),
        status: Status::Working,
    };

    let item2 = TodoItem {
        message: String::from("Sleep 19 hours"),
        status: Status::Completed,
    };

    list.add(item1);
    list.add(item2);

    list.print();
}
