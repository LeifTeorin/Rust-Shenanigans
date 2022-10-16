fn main()
{
   println!("What's 9 + 10?");

   let mut s = String::new();
   std::io::stdin().read_line(&mut s).unwrap();

   //let int:i32 = s.parse().unwrap();

   if &s == "21"
   {
   	println!("Banger");
   }
   else 
   {
   	println!("You stoopid.");
   }
}