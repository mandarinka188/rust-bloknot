use std::fs;
use std::io;

fn main() {
   println! ("Ти в блокноті");
   let mut document = String::new();
   loop{
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Erorr");

   match input.trim() {
       ":wq" => {
           println!("Ти збуріг документ та вийшов з нього");
           break;}
       _ => { document.push_str(&input) }    
  }
}
 println!("Введи назву файлу (наприклад, my_idea.txt): ");
 let mut filename = String::new();
 io::stdin().read_line(&mut filename).expect("Erorr");
 fs::write(filename.trim(), document).expect("Помилка при збереженні");
}
