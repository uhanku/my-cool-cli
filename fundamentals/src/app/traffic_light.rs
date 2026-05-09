use std::io::Write;
use std::time::Duration;
use std::{io, thread};

#[test]
fn run() {
    let loops = 2;
    let mut u = 0;
    let mut system = System {
        current: TrafficLight::Red,
    };

    while u < loops {
        print!("ON: {}", system.sign());
        io::stdout().flush().unwrap();
        system.wait();

        system.change();

        match system.current {
            TrafficLight::Red => u += 1,
            _ => {}
        }
    }
}

struct System {
    current: TrafficLight,
}

impl System {
    fn duration(&self) -> u64 {
        match self.current {
            TrafficLight::Red => 7,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 4,
        }
    }

    fn next(&self) -> TrafficLight {
        match self.current {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }

    fn sign(&self) -> &str {
        match self.current {
            TrafficLight::Red => "❤️",
            TrafficLight::Yellow => "💛",
            TrafficLight::Green => "💚",
        }
    }

    fn change(&mut self) {
        self.current = self.next();
    }

    fn wait(&self) {
        let d = self.duration();
        let mut u = 0;

        while u < d {
            thread::sleep(Duration::from_secs(1));
            print!(".");
            io::stdout().flush().unwrap();
            u += 1;
        }

        match self.current {
            TrafficLight::Red => print!("GO!"),
            TrafficLight::Yellow => print!("STOP!"),
            TrafficLight::Green => print!("SLOW DOWN!"),
        }

        println!();
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}
