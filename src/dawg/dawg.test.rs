#[cfg(test)]
mod test_dawg {

    use crate::dawg::dawg::Dawg;

    fn setup_dawg() -> Dawg {
        let mut dawg = Dawg::new();
        let mut words = vec![
            "BAM", "BAT", "BATH", "CATH", "BATHE", "CAR", "CARS", "CAREERS", "CATH", "CRASE",
            "HUMAN", "a", "aliancia", "alpa", "aloa", "alobal",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.to_uppercase())
        .collect::<Vec<_>>();

        words.sort();

        for i in 0..words.len() {
            dawg.insert(words[i].clone());
        }

        dawg.finish();
        return dawg;
    }

    fn adapt(word: impl AsRef<str>) -> Vec<String> {
        word.as_ref()
            .split_terminator("")
            .skip(1)
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_dawg_creation() {
        let dawg = Dawg::new();

        assert_eq!(dawg.unchecked_nodes.len(), 0);
        assert_eq!(dawg.previous_word, "");
        assert_eq!(dawg.minimized_nodes.keys().len(), 0);
        #[cfg(not(feature = "threading"))]
        assert_eq!(dawg.root.borrow().count, 0);
        #[cfg(feature = "threading")]
        assert_eq!(dawg.root.lock().unwrap().count, 0);

        #[cfg(not(feature = "threading"))]
        assert_eq!(dawg.root.borrow().edges.len(), 0);
        #[cfg(feature = "threading")]
        assert_eq!(dawg.root.lock().unwrap().edges.len(), 0);

        #[cfg(not(feature = "threading"))]
        assert_eq!(dawg.root.borrow().terminal, false);
        #[cfg(feature = "threading")]
        assert_eq!(dawg.root.lock().unwrap().terminal, false);
    }

    #[test]
    fn test_dawg_word_search() {
        let dawg = setup_dawg();

        assert_eq!(dawg.is_word(adapt("BAM"), true).unwrap(), adapt("BAM"));
        assert_eq!(dawg.is_word(adapt("BATHE"), true).unwrap(), adapt("BATHE"));
        assert_eq!(
            dawg.is_word(adapt("CAREERS"), true).unwrap(),
            adapt("CAREERS")
        );
        assert_eq!(dawg.is_word(adapt("HUMAN"), true).unwrap(), adapt("HUMAN"));
        assert_eq!(dawg.is_word(adapt("CAREE"), true), None);
        assert_eq!(dawg.is_word(adapt("CAREERZS"), true), None);
    }

    #[test]
    fn test_dawg_search_is_case_insensitive() {
        let dawg = setup_dawg();

        assert_eq!(dawg.is_word(adapt("BaM"), false).unwrap(), adapt("BaM"));
        assert_eq!(dawg.is_word(adapt("bat"), false).unwrap(), adapt("bat"));
        assert_eq!(
            dawg.is_word(adapt("cAreeRs"), false).unwrap(),
            adapt("cAreeRs")
        );
        assert_eq!(dawg.is_word(adapt("caree"), false), None);
        assert_eq!(dawg.is_word(adapt("CAREERZS"), false), None);
        assert_eq!(dawg.is_word(adapt("HUMAN"), false).unwrap(), adapt("HUMAN"));
    }

    #[test]
    fn test_prefix_exists() {
        let dawg = setup_dawg();

        assert!(dawg.lookup(adapt("care"), false).is_some());
        #[cfg(not(feature = "threading"))]
        let lookup = dawg.lookup(adapt("ba"), false).unwrap();
        #[cfg(not(feature = "threading"))]
        let lookup = lookup.borrow();

        #[cfg(feature = "threading")]
        let lookup = dawg.lookup(adapt("ba"), false).unwrap();
        #[cfg(feature = "threading")]
        let lookup = lookup.lock().unwrap();

        assert_eq!(lookup.edges.keys().len(), 2);

        assert!(dawg.lookup(adapt("CATH"), false).is_some());
    }
}
