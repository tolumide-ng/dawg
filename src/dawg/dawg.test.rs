#[cfg(test)]
mod test_dawg {

    use crate::dawg::dawg::Dawg;

    fn setup_dawg() -> Dawg {
        let mut dawg = Dawg::new();
        let mut words = vec![
            "BAM", "BAT", "BATH", "CATH", "BATHE", "CAR", "CARS", "CAREERS", "CATH", "CRASE",
            "HUMAN", "a", "aliancia", "alpa", "aloa", "alobal", "TAB", "SILENT", "LISTEN", "LIST",
            "TEN", "TIL", "STIL", "NEST", "IS", "EAT", "ATE", "TEA", "ETA", 
            // yorub words (not valid yoruba words though)
            "ayò", "òya"
        ]
        .iter()
        .map(|x| x.to_string())
        .map(|x| x.to_uppercase())
        .collect::<Vec<_>>();

        words.sort();

        for i in 0..words.len() {
            dawg.insert(words[i].clone());
        }

        dawg.finish();
        return dawg;
    }


    #[test]
    fn should_create_a_new_dawg() {
        let dawg = Dawg::new();

        assert_eq!(dawg.minimized_nodes.len(), 0);
        assert_eq!(dawg.unchecked_nodes.len(), 0);
        assert_eq!(dawg.previous_word, String::new());
        #[cfg(not(feature = "threading"))]
        {   
            assert_eq!(dawg.root.borrow().count, 0);
            assert_eq!(dawg.root.borrow().id, 0);
            assert_eq!(dawg.root.borrow().edges().len(), 0);
        }
        #[cfg(feature = "threading")]
        {
            assert_eq!(dawg.root.lock().unwrap().count, 0);
            assert_eq!(dawg.root.lock().unwrap().id, 0);
        }
    }

    #[cfg(test)]
    mod insert_new_word {
        use crate::Dawg;

        #[test]
        fn should_insert_a_new_word_into_a_new_dawg() {
            let word = String::from("success");
            let mut dawg = Dawg::new();
            dawg.insert(word.to_owned());

            {
                #[cfg(not(feature = "threading"))]
                let dawgie = dawg.root.borrow();
                #[cfg(feature = "threading")]
                let dawgie = dawg.root.lock().unwrap();

                assert_eq!(dawgie.edges().len(), 1);
                assert_eq!(dawgie.terminal, false);
            }

            assert_eq!(dawg.minimized_nodes.len(), 0);
            assert_eq!(dawg.unchecked_nodes.len(), word.len());
            assert_eq!(dawg.previous_word, word);
        }

        #[test]
        #[should_panic]
        fn should_panic_if_inserted_words_are_not_ordered() {
            let words = vec!["background", "backend"];
            let mut dawg = Dawg::new();
            for word in words {
                dawg.insert(word.to_string());
            }
        }

        #[test]
        fn should_insert_multiple_words_into_a_dawg() {
            let words = vec!["BACKEND", "BACKGROUND"];
            let mut dawg = Dawg::new();
            for word in &words {
                dawg.insert(word.to_string());
            }

            {
                #[cfg(not(feature = "threading"))]
                let dawgie = dawg.root.borrow();
                #[cfg(feature = "threading")]
                let dawgie = dawg.root.lock().unwrap();

                assert_eq!(dawgie.edges().len(), 1);
                assert!(dawgie.edges().get(&"B".to_string()).is_some());
                assert_eq!(dawgie.terminal, false);
            }
            

            assert_eq!(dawg.minimized_nodes.len(), 3); // E, N, D (the nodes removed from unchecked node after the addition of background)
            assert_eq!(dawg.unchecked_nodes.len(), words.last().unwrap().len());

            let new_word = "COMEDY".to_string();
            dawg.insert(new_word.clone());
            assert_eq!(dawg.minimized_nodes.len(), words.last().unwrap().len());
            assert_eq!(dawg.unchecked_nodes.len(), new_word.len());
        }
    }


    #[test]
    fn should_cleanup_after_calling_finish() {
        let words = vec!["BACKEND", "BACKGROUND"];
        let mut dawg = Dawg::new();
        for word in &words {
            dawg.insert(word.to_string());
        }

        assert_eq!(dawg.minimized_nodes.len(), 3); // E, N, D (the nodes removed from unchecked node after the addition of background)
        assert_eq!(dawg.unchecked_nodes.len(), words.last().unwrap().len());

        dawg.finish();
        assert_eq!(dawg.minimized_nodes.len(), 0);
        assert_eq!(dawg.unchecked_nodes.len(), 0);
        assert_eq!(dawg.previous_word.len(), 0);
    }


    #[test]
    fn test_dawg_word_search() {
        let dawg = setup_dawg();

        assert_eq!(dawg.is_word(String::from("BAM"), true).unwrap(), String::from("BAM"));
        assert_eq!(dawg.is_word(String::from("BATHE"), true).unwrap(), String::from("BATHE"));
        assert_eq!(dawg.is_word(String::from("BA"), false), None);
        assert_eq!(
            dawg.is_word(String::from("CAREERS"), true).unwrap(),
            String::from("CAREERS")
        );
        assert_eq!(dawg.is_word(String::from("HUMAN"), true).unwrap(), String::from("HUMAN"));
        assert_eq!(dawg.is_word(String::from("CAREE"), true), None);
        assert_eq!(dawg.is_word(String::from("CAREERZS"), true), None);
    }

    #[test]
    fn test_dawg_search_is_case_insensitive() {
        let dawg = setup_dawg();

        assert_eq!(dawg.is_word(String::from("BaM"), false).unwrap(), String::from("BaM"));
        assert_eq!(dawg.is_word(String::from("bat"), false).unwrap(), String::from("bat"));
        assert_eq!(
            dawg.is_word(String::from("cAreeRs"), false).unwrap(),
            String::from("cAreeRs")
        );
        assert_eq!(dawg.is_word(String::from("caree"), false), None);
        assert_eq!(dawg.is_word(String::from("CAREERZS"), false), None);
        assert_eq!(dawg.is_word(String::from("HUMAN"), false).unwrap(), String::from("HUMAN"));
    }

    #[test]
    fn test_prefix_exists() {
        let dawg = setup_dawg();

        assert!(dawg.lookup(String::from("care"), false).is_some());
        #[cfg(not(feature = "threading"))]
        let lookup = dawg.lookup(String::from("ba"), false).unwrap();
        #[cfg(not(feature = "threading"))]
        let lookup = lookup.borrow();

        #[cfg(feature = "threading")]
        let lookup = dawg.lookup(String::from("ba"), false).unwrap();
        #[cfg(feature = "threading")]
        let lookup = lookup.lock().unwrap();

        assert_eq!(lookup.edges.keys().len(), 2);

        assert!(dawg.lookup(String::from("CATH"), false).is_some());
    }


    #[cfg(test)]
    mod anagrams {
        use std::collections::HashSet;

        use super::setup_dawg;

        #[test]
        fn should_return_all_the_possible_angrams() {
            let dawg = setup_dawg();

            let mut received = dawg.find_anagrams("LISTEN");
            let mut expected = vec!["LISTEN".to_string(), "SILENT".to_string()];
            
            received.sort();
            expected.sort();

            assert_eq!(expected, received);

            let mut received = dawg.find_anagrams("EAT");
            let mut expected = vec!["EAT".to_string(), "TEA".to_string(), "ATE".to_string(), "ETA".to_string()];

            received.sort();
            expected.sort();

            assert_eq!(expected, received);

            let mut received = dawg.find_anagrams("AYÒ"); // (re do)
            let mut expected = vec!["AYÒ".to_string(), "ÒYA".to_string()]; // ÒYA (do re)
            
            received.sort();
            expected.sort();

            assert_eq!(expected, received);



            let mut received = dawg.find_anagrams("AYÓ"); // (re mi) - same letters different diacritical/tonal marks
            let expected: Vec<String> = vec![];
            
            received.sort();

            assert_eq!(expected, received);
        }
    }
}
