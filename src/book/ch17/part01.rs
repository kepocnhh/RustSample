pub fn run() {
    const CHAPTER: u8 = 17;
    const PART: u8 = 1;
    const TITLE: &str = "Characteristics of Object-Oriented Languages";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    todo!();
}

mod _1701 {
    pub struct MyInts {
        list: Vec<i32>,
        average: f64,
    }

    impl MyInts {
        pub fn new(list: Vec<i32>) -> MyInts {
            let average = get_average(&list);
            return MyInts {
                list,
                average
            }
        }

        pub fn push(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn pop(&mut self) -> Option<i32> {
            let option = self.list.pop();
            if let Some(_) = option {
                self.update_average();
            }
            return option;
        }

        fn update_average(&mut self) {
            self.average = get_average(&self.list);
        }

        pub fn get_average(&self) -> f64 {
            return self.average;
        }
    }

    fn get_average(list: &Vec<i32>) -> f64 {
        let sum: i32 = list.iter().sum();
        return  sum as f64 / list.len() as f64;
    }
}

fn _01() {
    println!("\nEncapsulation that Hides Implementation Details");

    let list = vec![1, 2, 3];
    let mut ints = _1701::MyInts::new(list.clone());
    ints.push(4);
    assert_eq!(2.5, ints.get_average());
    println!("{list:?} + 4: average {}", ints.get_average());
    ints.pop();
    ints.pop();
    assert_eq!(1.5, ints.get_average());
    println!("{list:?} - pop - pop: average {}", ints.get_average());
}
