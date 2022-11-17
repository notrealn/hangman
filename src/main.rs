use std::io;
use std::fs;
use std::collections::HashMap;

fn main() -> io::Result<()> {
  // load dictionary into memory
  let file = fs::read_to_string("/home/gamer/coding/pset1/p3/words_alpha.txt")?;
  let mut dict: Vec<&str> = file.lines().collect();

  let mut word = String::new();
  println!("enter word: ");
  io::stdin().read_line(&mut word)?;
  word.pop();

  if !dict.contains(&word.as_str()) {
    println!("word not in dictionary");
    return Ok(());
  }

  filter(&mut dict, &|s| s.len() != word.len());

  let mut fails = 0;
  let mut blacklist = vec![];

  while fails < 5 {
    if dict.len() < 2 {
      break
    }
    
    let guess = most_common_char(&dict, &blacklist);
    blacklist.push(guess);
    println!("guesses: {:?}", blacklist);

    if word.contains(guess) {
      let indices: Vec<_> = word.match_indices(guess).map(|e| e.0).collect();
      
      filter(&mut dict, &|s| {
        for (i, c) in s.char_indices() {
          if indices.contains(&i) {
            if c != guess {
              return true
            }
          } else {
            if c == guess {
              return true
            }
          }
        }
        false
      });
    } else {
      fails += 1;
      println!("fails: {}", fails);
      filter(&mut dict, &|s| s.contains(guess));
    }
  }

  if fails >= 5 {
    println!("epic fail");
  }

  println!("guesses: {:?}", dict);

  Ok(())
}

fn most_common_char(words: &Vec<&str>, blacklist: &Vec<char>) -> char {
  let mut frequencies = HashMap::new();

  for word in words {
    for char in word.chars() {
      if blacklist.contains(&char) {
        continue
      }
      *frequencies.entry(char).or_insert(0) += 1;
    }
  }

  frequencies.into_iter().fold(('a', 0), |acc, e| {
    if e.1 > acc.1 { e } else { acc }
  }).0
}

fn filter(vec: &mut Vec<&str>, fun: &dyn Fn(&str) -> bool) {
  let mut i = 0;
  while i < vec.len() {
    if fun(vec[i]) {
      vec.remove(i);
    } else {
      i += 1;
    }
  }
}