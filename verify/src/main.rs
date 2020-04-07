#[cfg(feature = "levenshtein")]
use fst::automaton::Levenshtein;
#[cfg(feature = "levenshtein")]
use fst::raw::{Builder, Fst};

fn main() {
    #[cfg(feature = "levenshtein")]
    let keys = vec!["fa", "fo", "fob", "focus", "foo", "food", "foul"];
    #[cfg(feature = "levenshtein")]
    let set = Set::from_iter(keys).unwrap();

    #[cfg(feature = "levenshtein")]
    let lev = Levenshtein::new("foo", 1).unwrap();
    #[cfg(feature = "levenshtein")]
    let mut stream = set.search(&lev).into_stream();

    #[cfg(feature = "levenshtein")]
    let mut keys = vec![];
    #[cfg(feature = "levenshtein")]
    while let Some(key) = stream.next() {
        keys.push(key.to_vec());
    }
    #[cfg(feature = "levenshtein")]
    assert_eq!(
        keys,
        vec![
            "fo".as_bytes(),   // 1 deletion
            "fob".as_bytes(),  // 1 substitution
            "foo".as_bytes(),  // 0 insertions/deletions/substitutions
            "food".as_bytes(), // 1 insertion
        ]
    );
}
