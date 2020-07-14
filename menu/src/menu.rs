use std;
use std::io::Write;

#[derive(Debug)]
pub struct Menu {
    text: String,
    children: Vec<Menu>,
}

impl Menu {
    #[allow(unused)]
    pub const fn new() -> Menu {
        Menu::new_named(String::new())
    }

    pub const fn new_named(name: String) -> Menu {
        Menu { text: name, children: Vec::new() }
    }

    pub fn add_blank(&mut self, name: &str) {
        self.add_sub_menu(Menu::new_named(name.to_string()));
    }

    pub fn add_sub_menu(&mut self, menu: Menu) {
        self.children.push(menu);
    }

    pub fn println(&self) {
        println!("{}:", self.text);
        for (i, item) in self.children.iter().enumerate() {
            println!("{}. {}", i + 1, item.text)
        }
        println!("0. Exit");
    }

    pub fn show_menu(&self) -> Result<usize, &'static str> {
        self.println();
        print!("Enter an option: ");
        std::io::stdout().flush().ok().expect("Could not flush stdout");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let option = match input.trim().parse() {
            Ok(i) => i,
            Err(_) => 0,
        };

        match option {
            0 => Ok(option),
//            _ => self.children.get(option).show_menu()
            _ => match self.children.get::<usize>(option - 1) {
                Some(sub_menu) => sub_menu.show_menu(),
                None => self.show_menu(),
            }
        }
    }
}