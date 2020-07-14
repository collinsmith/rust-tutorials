//use std::io::Write;

mod menu;
use menu::Menu;

fn main() {
    let mut menu = Menu::new_named(String::from("Main"));
    menu.add_blank("Option");
    menu.add_sub_menu(Menu::new_named(String::from("Submenu")));
//    menu.println();
//
//    let mut input = String::new();
//    loop {
//        print!("Enter and option: ");
//        std::io::stdout().flush().ok().expect("Could not flush stdout");
//
//        std::io::stdin().read_line(&mut input).expect("Failed to read line");
//
//        let option: u8 = match input.trim().parse() {
//            Ok(i) => i,
//            _ => continue,
//        };
//
//        match option {
//            0 | _ => break,
//        }
//    }
    menu.show_menu().ok().expect("Failed to show menu");
}
