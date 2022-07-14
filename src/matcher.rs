use grep::{regex::RegexMatcher, searcher::{Searcher, sinks::UTF8}, matcher::Matcher};

pub fn is_match(path: &str, matcher: RegexMatcher) -> bool {
  let mut is_match = false;
  Searcher::new().search_slice(&matcher, path.as_bytes(), UTF8(|_, line| {
    match matcher.find(line.as_bytes()) {
      Ok(matches) => {
        is_match = matches.is_some();
        Ok(true)
      },
      Err(_) => {
        is_match = false;
        Ok(false)
      }
    }
  })).unwrap();
  is_match
}

pub fn create_matcher(query: &String) -> RegexMatcher {
  let query = query.replace("*", r"\w+");
  println!("{}",query);
  RegexMatcher::new(query.as_str()).unwrap()
}