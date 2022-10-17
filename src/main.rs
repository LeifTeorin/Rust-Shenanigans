fn main()
{
   println!("What's 9 + 10?");

   let mut s = String::new();
   std::io::stdin().read_line(&mut s).unwrap();

   //dude wtf
   let num:i32 = s.parse();
   let goal:i32 = 21;

   println!("{}", goal);
   println!("{}", num);

   if num == goal
   {
   	println!("Banger");
   }
   else 
   {
   	println!("You stoopid.");
   }
}