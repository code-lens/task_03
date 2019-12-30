fn talk(msg: String)
{
  let conv=msg.trim();
  if msg.trim() =="How are you?"
  {
    println!("Sure");
  }
  // yell at bob
  else if msg.to_uppercase()==msg
  {
    // yell a question 
    if conv.ends_with('?') 
    {
      println!("Calm down, I know what I'm doing!");
    }
    else
    {
      println!("Whoa,Chill out!");
    }
  }
  // just adrdess bob
  else if msg.trim() =="Bob"
  {
    println!("Fine.Be that way!");
  }

  else
  {
    println!("Whatever");
  }


}

//to take input from user
fn main()
{
  let mut line = String::new();
   println!("Talk to Bob:");
   let _b1 = std::io::stdin().read_line(&mut line).unwrap();
  talk(line);
}

