#![feature(test)]
#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate test;

use regex::Regex;

lazy_static! {
    static ref DOLLAR_SEARCH: Regex = Regex::new(r"DOLLAR|USD|\$").unwrap();
}
/// Finds the multiplier within a couple of lines that dollar is mentioned
///
/// #Examples
///
/// ```
/// use rust_vs_python_2;
///
/// let table_header = vec![
///     "header line 1".to_string(),
///     "header line 2".to_string(),
///     "header line 3".to_string(),
///     "dollars in billion".to_string(),
///     "header line 5".to_string(),
///     "header line 6".to_string(),
///     "header line 7".to_string(),
/// ];
///
/// let multiplier = rust_vs_python_2::find_multiplier(&table_header);
/// ```
pub fn find_multiplier(table_header: &Vec<String>) -> u32 {
    // find the line within the header that contains the dollar term
    let line_index = table_header
        .iter()
        .enumerate()
        .find(|(_, line)| DOLLAR_SEARCH.is_match(&line.to_uppercase()))
        .map(|(x, _)| x);

    // if dollar is found within the index then find the number multiplier nearby
    if let Some(index) = line_index {
        table_header
            .iter()
            .skip(index - 1)
            .take(3)
            .find_map(|line| {
                [
                    ("THOUSAND", 1000),
                    ("MILLION", 1000000),
                    ("BILLION", 1000000000),
                ]
                .iter()
                .find_map(|(key, val)| {
                    if line.to_uppercase().contains(key) {
                        Some(*val)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(1)
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_find_multiplier() {
        let table_header_same = vec![
            "header line 1".to_string(),
            "header line 2".to_string(),
            "header line 3".to_string(),
            "dollars in billion".to_string(),
            "header line 5".to_string(),
            "header line 6".to_string(),
            "header line 7".to_string(),
        ];
        let table_header_above = vec![
            "header line 1".to_string(),
            "header line 2".to_string(),
            "in Thousands".to_string(),
            "$".to_string(),
            "header line 5".to_string(),
            "header line 6".to_string(),
            "header line 7".to_string(),
        ];
        let table_header_below = vec![
            "header line 1".to_string(),
            "header line 2".to_string(),
            "USD".to_string(),
            "in millions".to_string(),
            "header line 5".to_string(),
            "header line 6".to_string(),
            "header line 7".to_string(),
        ];
        assert_eq!(find_multiplier(&table_header_same), 1000000000);
        assert_eq!(find_multiplier(&table_header_above), 1000);
        assert_eq!(find_multiplier(&table_header_below), 1000000);
    }
    #[bench]
    fn bench_find_multiplier(b: &mut Bencher) {
        let table_header_same = vec![
            "header line 1".to_string(),
            "header line 2".to_string(),
            "header line 3".to_string(),
            "dollars in billion".to_string(),
            "header line 5".to_string(),
            "header line 6".to_string(),
            "header line 7".to_string(),
        ];
        b.iter(|| find_multiplier(&table_header_same));
    }
}
