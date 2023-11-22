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
        use std::{collections::HashMap, borrow::BorrowMut, rc::Rc};

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
                assert!(dawgie.edges().get("B").is_some());
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
        fn should_remove_all_unchecked_nodes_when_finish_is_called() {

        }

        // if the previously existing node that is now mapped to the parent (see line 79 - 91) was not a terminal node
        // and the child of this parent was a terminal node, ensure that there is no mistakes 
        // (because updating this to a terminal would cause issues for the previos nodes that relied on it, and not updating it to terminal
        // would cause issues for the new parent)
        // #[test]
        // fn minimize_works_fine_for_termina_and_non_terminal_letters_with_common_prefixes() {
        //     let words = [ "PIC", "PICK", "PICKBACK", "PICKED", "PICKLE", "PICKY",];
        // }

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
