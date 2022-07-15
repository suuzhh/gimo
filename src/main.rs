mod git;
mod matcher;
use std::env;

use crate::{git::{get_local_branches, delete_branch}, matcher::{create_matcher, is_match}};


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    if query.contains("*") {
        let matcher = create_matcher(query);

        let branches = get_local_branches();
        for branch in branches {
            let result = is_match(branch.as_str(), matcher.clone());
            if result {
                delete_branch(&branch);
                println!("delete branch: {}", branch)
            }
        }
    } else {
        delete_branch(query);
        println!("delete branch: {}", query)
    }
}

