fn main()
{
   println!("What's 9 + 10?");

   let mut s = String::new();
   std::io::stdin().read_line(&mut s).unwrap();

   let goal:&str = "21";

   if &s == goal
   {
   	println!("Banger");
   }
   else 
   {
   	println!("You stoopid.");
   }
}