use crate::core::{prelude::*, usecases};
use std::{cmp::min, collections::HashSet};

#[derive(Debug, PartialEq, Serialize)]
pub enum DuplicateType {
    SimilarChars,
    SimilarWords,
}

// return vector of places like: (entry1ID, entry2ID, reason)
// where entry1 and entry2 are similar places
pub fn find_duplicates(
    places: &[(Place, ReviewStatus)],
    possible_duplicate_places: &[(Place, ReviewStatus)],
) -> Vec<(Id, Id, DuplicateType)> {
    let mut duplicates = Vec::new();
    for (p1, _) in &places[..] {
        for (p2, _) in &possible_duplicate_places[..] {
            if p1.id >= p2.id {
                continue;
            }
            if let Some(t) = is_duplicate(p1, p2) {
                duplicates.push((p1.id.clone(), p2.id.clone(), t));
            }
        }
    }
    duplicates
}

pub fn find_duplicate_places(
    new_place: &Place,
    possible_duplicate_places: &[(Place, ReviewStatus)],
) -> Vec<(Id, DuplicateType)> {
    let mut duplicates = Vec::new();
    for (p, _) in &possible_duplicate_places[..] {
        if let Some(t) = is_duplicate(new_place, p) {
            duplicates.push((p.id.clone(), t));
        }
    }
    duplicates
}

const DUPLICATE_MAX_DISTANCE: Distance = Distance::from_meters(100.0);

// returns a DuplicateType if the two places have a similar title, returns None otherwise
fn is_duplicate(e1: &Place, e2: &Place) -> Option<DuplicateType> {
    if similar_title(e1, e2, 0.3, 0) && in_close_proximity(e1, e2, DUPLICATE_MAX_DISTANCE) {
        Some(DuplicateType::SimilarChars)
    } else if similar_title(e1, e2, 0.0, 2) && in_close_proximity(e1, e2, DUPLICATE_MAX_DISTANCE) {
        Some(DuplicateType::SimilarWords)
    } else {
        None
    }
}

fn in_close_proximity(e1: &Place, e2: &Place, max_dist: Distance) -> bool {
    if let Some(dist) = MapPoint::distance(e1.location.pos, e2.location.pos) {
        return dist <= max_dist;
    }
    false
}

fn similar_title(
    e1: &Place,
    e2: &Place,
    max_percent_different: f32,
    max_words_different: u32,
) -> bool {
    let max_dist =
        ((min(e1.title.len(), e2.title.len()) as f32 * max_percent_different) + 1.0) as usize; // +1 is to get the ceil

    levenshtein_distance_small(&e1.title, &e2.title, max_dist)
        || words_equal_except_k_words(&e1.title, &e2.title, max_words_different)
}

// returns true if all but k words are equal in str1 and str2
// (and one of them has more than one word)
// (words in str1 and str2 are treated as sets, order & multiplicity of words doesn't matter)
fn words_equal_except_k_words(str1: &str, str2: &str, k: u32) -> bool {
    let len1 = str1.split_whitespace().count();
    let len2 = str2.split_whitespace().count();

    if (len1 == 1) & (len2 == 1) {
        return false;
    }

    let (s1, s2) = if len1 <= len2 {
        (str1, str2)
    } else {
        (str2, str1)
    };

    let words = s1.split_whitespace();

    let mut diff = 0;
    let mut set = HashSet::new();

    for w in words {
        set.insert(w);
    }

    for w in s2.split(' ') {
        if !set.contains(w) {
            diff += 1;
        }
    }
    diff <= k
}

// Levenshtein Distance more realistically captures typos (all of the following
// operations are counted as distance 1: add one character in between, delete
// one character, change one character)
// but it proved to be way too slow to be run on the whole dataset
fn levenshtein_distance_small(s: &str, t: &str, max_dist: usize) -> bool {
    levenshtein_distance(s, t) <= max_dist
}

// Algorithm from
// https://en.wikipedia.org/wiki/Levenshtein_distance#Computing_Levenshtein_distance
fn levenshtein_distance(s: &str, t: &str) -> usize {
    let max_s: usize = s.len() + 1;
    let max_t: usize = t.len() + 1;

    // for all i and j, d[i,j] will hold the Levenshtein distance between
    // the first i characters of s and the first j characters of t
    // comment that d has (m+1)*(n+1) values
    let mut d: Vec<Vec<usize>> = vec![];
    for _ in 0..max_s {
        d.push(vec![0; max_t]);
    }

    // source (s) prefixes can be transformed into empty string by
    // dropping all characters
    for (i, item) in d.iter_mut().enumerate().take(max_s).skip(1) {
        item[0] = i;
    }

    // target (t) prefixes can be reached from empty source prefix
    // by inserting every character
    for j in 1..max_t {
        d[0][j] = j;
    }

    for j in 1..max_t {
        for i in 1..max_s {
            let substitution_cost = if s.chars().nth(i) == t.chars().nth(j) {
                0
            } else {
                1
            };
            d[i][j] = min3(
                d[i - 1][j] + 1,                     // deletion
                d[i][j - 1] + 1,                     // insertion
                d[i - 1][j - 1] + substitution_cost, // substitution
            )
        }
    }

    d[max_s - 1][max_t - 1]
}

fn min3(s: usize, t: usize, u: usize) -> usize {
    if s <= t {
        min(s, u)
    } else {
        min(t, u)
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal, clippy::excessive_precision)]
mod tests {
    use super::*;

    fn new_place(title: String, description: String, pos: MapPoint) -> Place {
        Place::build()
            .id(&title)
            .title(&title)
            .description(&description)
            .pos(pos)
            .finish()
    }

    #[test]
    fn test_in_close_proximity() {
        let e1 = new_place(
            "Entry 1".to_string(),
            "Punkt1".to_string(),
            MapPoint::from_lat_lng_deg(48.23153745093964, 8.003816366195679),
        );
        let e2 = new_place(
            "Entry 2".to_string(),
            "Punkt2".to_string(),
            MapPoint::from_lat_lng_deg(48.23167056421013, 8.003558874130248),
        );

        assert!(in_close_proximity(&e1, &e2, Distance::from_meters(30.0)));
        assert!(!in_close_proximity(&e1, &e2, Distance::from_meters(10.0)));
    }

    #[test]
    fn test_similar_title() {
        let e1 = new_place(
            "0123456789".to_string(),
            "Hallo! Ein Eintrag".to_string(),
            MapPoint::from_lat_lng_deg(48.23153745093964, 6.003816366195679),
        );
        let e2 = new_place(
            "01234567".to_string(),
            "allo! Ein Eintra".to_string(),
            MapPoint::from_lat_lng_deg(48.23153745093964, 6.003816366195679),
        );
        let e3 = new_place(
            "eins zwei drei".to_string(),
            "allo! Ein Eintra".to_string(),
            MapPoint::from_lat_lng_deg(48.23153745093964, 6.003816366195679),
        );
        let e4 = new_place(
            "eins zwei fünf sechs".to_string(),
            "allo! Ein Eintra".to_string(),
            MapPoint::from_lat_lng_deg(48.23153745093964, 6.003816366195679),
        );

        assert_eq!(true, similar_title(&e1, &e2, 0.2, 0)); // only 2 characters changed
        assert_eq!(false, similar_title(&e1, &e2, 0.1, 0)); // more than one character changed
        assert_eq!(true, similar_title(&e3, &e4, 0.0, 2)); // only 2 words changed
        assert_eq!(false, similar_title(&e3, &e4, 0.0, 1)); // more than 1 word changed
    }

    #[test]
    fn test_is_duplicate() {
        let e1 = new_place(
            "Ein Eintrag Blablabla".to_string(),
            "Hallo! Ein Eintrag".to_string(),
            MapPoint::from_lat_lng_deg(47.23153745093964, 5.003816366195679),
        );
        let e2 = new_place(
            "Eintrag".to_string(),
            "Hallo! Ein Eintrag".to_string(),
            MapPoint::from_lat_lng_deg(47.23153745093970, 5.003816366195679),
        );
        let e3 = new_place(
            "Enn Eintrxg Blablalx".to_string(),
            "Hallo! Ein Eintrag".to_string(),
            MapPoint::from_lat_lng_deg(47.23153745093955, 5.003816366195679),
        );
        let e4 = new_place(
            "En Eintrg Blablala".to_string(),
            "Hallo! Ein Eintrag".to_string(),
            MapPoint::from_lat_lng_deg(47.23153745093955, 5.003816366195679),
        );
        let e5 = new_place(
            "Ein Eintrag Blabla".to_string(),
            "Hallo! Ein Eintrag".to_string(),
            MapPoint::from_lat_lng_deg(40.23153745093960, 5.003816366195670),
        );

        // titles have a word that is equal
        assert_eq!(Some(DuplicateType::SimilarWords), is_duplicate(&e1, &e2));
        // titles similar: small levenshtein distance
        assert_eq!(Some(DuplicateType::SimilarChars), is_duplicate(&e1, &e4));
        // titles similar: small hamming distance
        assert_eq!(Some(DuplicateType::SimilarChars), is_duplicate(&e1, &e3));
        // titles not similar
        assert_eq!(None, is_duplicate(&e2, &e4));
        // places not located close together
        assert_eq!(None, is_duplicate(&e4, &e5));
    }

    #[test]
    fn test_min() {
        assert_eq!(1, min3(1, 2, 3));
        assert_eq!(2, min3(3, 2, 3));
        assert_eq!(2, min3(3, 3, 2));
        assert_eq!(1, min3(1, 1, 1));
    }

    #[test]
    fn test_words_equal() {
        assert_eq!(true, words_equal_except_k_words("ab abc a", "ab abc b", 1));
        assert_eq!(true, words_equal_except_k_words("ab abc a", "abc ab", 1));
        assert_eq!(true, words_equal_except_k_words("ab ac a", "abc ab ab", 2));
        assert_eq!(false, words_equal_except_k_words("a a a", "ab abc", 2));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(3, levenshtein_distance("012a34c", "0a3c")); // delete 1,2 and 4
        assert_eq!(1, levenshtein_distance("12345", "a12345")); // insert a
        assert_eq!(1, levenshtein_distance("aabaa", "aacaa")); // replace b by c
    }
}
