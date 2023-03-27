pub fn run() {
    const CHAPTER: u8 = 17;
    const PART: u8 = 2;
    const TITLE: &str = "Using Trait Objects That Allow for Values of Different Types";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
}

mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn new(components: Vec<Box<dyn Draw>>) -> Screen {
            return Screen { components }
        }

        pub fn run(&self) {
            for it in self.components.iter() {
                it.draw();
            }
        }
    }
}

struct Button {
    pub title: String
}

struct CheckBox {
    pub is_checked: bool
}

impl gui::Draw for Button {
    fn draw(&self) {
        println!("draw button: {}", self.title);
    }
}

impl gui::Draw for CheckBox {
    fn draw(&self) {
        println!("draw check box: {}", self.is_checked);
    }
}

fn _01() {
    println!("\nDefining a Trait for Common Behavior");

    let components: Vec<Box<dyn gui::Draw>> = vec![
        Box::new(Button { title: String::from("foo") }),
        Box::new(CheckBox { is_checked: false }),
    ];
    gui::Screen::new(components).run();
}
