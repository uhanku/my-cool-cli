#[test]
fn run() {
    let mut cs: Vec<GameCharacter> = Vec::new();
    cs.push({
        GameCharacter {
            class: Class::Warrior,
            age: 23,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Paladin,
            age: 19,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Healer,
            age: 18,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Goblin,
            age: 200,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Archer,
            age: 40,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Elf,
            age: 90,
        }
    });

    for c in cs {
        println!("======================================");
        println!("Class: {}", c.title());
        println!("Age: {}", c.age);
    }
}

struct GameCharacter {
    class: Class,
    age: i32,
}

impl GameCharacter {
    fn title(&self) -> &str {
        match self.class {
            Class::Warrior => "⚔️  Warrior",
            Class::Paladin => "☀️  Paladin",
            Class::Healer => "💚  Healer",
            Class::Goblin => "👺  Goblin",
            Class::Archer => "🎯  Archer",
            Class::Elf => "🍃 Elf",
        }
    }
}

enum Class {
    Warrior,
    Paladin,
    Healer,
    Goblin,
    Archer,
    Elf,
}
