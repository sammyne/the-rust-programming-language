pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Listing 12-17: Iterating through each line in contents
    for line in contents.lines() {
        // Listing 12-18: Adding functionality to see whether the line contains the string in query
        if line.contains(query) {
            // Listing 12-19: Storing the lines that match so we can return them
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
