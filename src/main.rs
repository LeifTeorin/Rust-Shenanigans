use std::io;

fn main()
{
   println!("What's 9 + 10?");

   let mut s = String::new();
   io::stdin()
      .read_line(&mut s)
      .expect("failed to read line");

   
   if s.trim() == "21"
   {
   	println!("Banger");
   }
   else 
   {
   	println!("You stoopid.");
   }
}