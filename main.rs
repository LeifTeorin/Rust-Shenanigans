fn main()
{
   println!("What's 9 + 10?");

   let mut s = String::new();
   std::io::stdin().read_line(&mut s).unwrap();

   //let goal: String = String::from("21");

   let banana: &str = "banana";
   if banana == "banana"{
      println!("bruh");
   }

   if s.eq(goal){
   	println!("Banger");
   }else{
   	println!("You stoopid.");
   }
}