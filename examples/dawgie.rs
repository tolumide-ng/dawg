fn main() {
    use dawg::Dawg;

    let mut dawgie = Dawg::new();

    let mut words = vec![
        "BAM", "BAT", "BATH", "CATH", "BATHE", "CAR", "CARS", "CAREERS", "CATH", "CRASE", "HUMAN",
        "a", "aliancia", "alpa", "aloa", "alobal",
    ]
    .iter()
    .map(|x| x.to_string().to_uppercase())
    .collect::<Vec<_>>();

    words.sort();

    for word in words {
        dawgie.insert(word);
    }

    // seal the dawg once you're done
    dawgie.finish();

    assert_eq!(dawgie.is_word(String::from("BATH"), true), Some("BATH".to_string()));
    assert!(dawgie.is_word(String::from("NOTHINGHERE"), true).is_none());

    assert!(dawgie.lookup(String::from("case"), false).is_none());
    assert!(dawgie.lookup(String::from("human"), false).is_some());
}