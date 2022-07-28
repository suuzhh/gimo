use regex::Regex;

pub fn is_match(path: &str, re: Regex) -> bool {
  re.is_match(path)
}

pub fn create_matcher(query: &String) -> Regex {
  let query = query.replace("*", r"\w+");
  let query = format!("^{}", query.replace("*", r"\w+"));
  // println!("{}",query);
  let re = Regex::new(query.as_str()).unwrap();
  re
}
