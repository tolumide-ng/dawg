#[cfg(test)]
mod test_dawg_node {
    #[cfg(feature = "threading")]
    use std::{sync::{Arc, Mutex}};
    #[cfg(not(feature = "threading"))]
    use std::{rc::Rc, cell::RefCell};

    use crate::node::node::{DawgWrapper, DawgNode};

    #[test]
    fn first_id_on_dawgwrapper_is_0() {
        let dawg_wrapper = DawgWrapper::new();
        
        assert_eq!(dawg_wrapper.next_id, 0);
    }

    #[test]
    fn dawgwrapper_assigns_new_id_of_new_dawgnodes() {
        let mut dawg_wrapper = DawgWrapper::new();

        assert_eq!(dawg_wrapper.next_id, 0);

        let dawg_node = dawg_wrapper.create();
        
        
        
        {
            #[cfg(feature = "threading")]
            let node_zero = dawg_node.lock().unwrap();
            #[cfg(not(feature = "threading"))]
            let node_zero = dawg_node.borrow();
            
            assert_eq!(node_zero.id, 0);
            assert_eq!(node_zero.count, 0);
            assert_eq!(node_zero.terminal, false);
            assert_eq!(node_zero.edges.keys().len(), 0);
        }
        
        assert_eq!(dawg_wrapper.next_id, 1);        

        
        let new_dawg_node = dawg_wrapper.create();
        {
            #[cfg(feature = "threading")]
            let mut node_one = new_dawg_node.lock().unwrap();
            #[cfg(not(feature = "threading"))]
            let mut node_one = new_dawg_node.borrow_mut();
            
            assert_eq!(node_one.id, 1);
            assert_eq!(node_one.count, 0);
            assert_eq!(node_one.terminal, false);
            assert_eq!(node_one.edges.keys().len(), 0);
            assert_eq!(node_one.num_reachable(), 0);
        }
    }


    #[test]
    fn should_get_the_nodes_reachable_from_any_node_n() {
        let mut dawg_wrapper = DawgWrapper::new();

        //                          0
        //              /\                   /\  
        //            a    b(t)             c    x
        //                 /   \ 
        //               d(t)  e(t)
        // 
        // 
        // where (t) means it's a terminal i.e. (end of a word) -> terminal = true

        #[cfg(not(feature = "threading"))]
        let mut nodes: Vec<Rc<RefCell<DawgNode>>>  = Vec::with_capacity(8);
        #[cfg(feature = "threading")]
        let mut nodes: Vec<Arc<Mutex<DawgNode>>>  = Vec::with_capacity(8);

        let dawg_node = dawg_wrapper.create();
        nodes.push(dawg_node);

        for id in 'a'..='f' {
            let dawg_node = dawg_wrapper.create();

            #[cfg(feature = "threading")]
            let mut ref_node = dawg_node.lock().unwrap();
            #[cfg(not(feature = "threading"))]
            let mut ref_node = dawg_node.borrow_mut();

            // setup
            if ['a', 'b', 'c'].contains(&id) {
                if let Some(node_at_zero) = nodes.get_mut(0) {
                    #[cfg(not(feature = "threading"))]
                    node_at_zero.borrow_mut().edges.insert(id.to_string(), Rc::clone(&dawg_node));
                    #[cfg(feature = "threading")]
                    node_at_zero.lock().unwrap().edges.insert(id.to_string(), Arc::clone(&dawg_node));
                }
            } else if ['d', 'e'].contains(&id) {
                // 4th and 5th node are terminals (and the children of node with id 2)
                // genealogy (0(root) --->> [1, 2(this one-*), 3] --->> [4, 5])
                if let Some(node_at_zero) = nodes.get_mut(2) {
                    ref_node.terminal = true;

                    #[cfg(not(feature = "threading"))]
                    node_at_zero.borrow_mut().edges.insert(id.to_string(), Rc::clone(&dawg_node));
                    #[cfg(feature = "threading")]
                    node_at_zero.lock().unwrap().edges.insert(id.to_string(), Arc::clone(&dawg_node));
                }
            } else if id == 'f' {
                // 6th node is a terminal
                // genealogy (0(root) --->> [1, 2(this one-*), 3] --->> [4, 5(this one-*)] --->> [6])
                if let Some(node_at_zero) = nodes.get_mut(5) {
                    ref_node.terminal = true;

                    #[cfg(not(feature = "threading"))]
                    node_at_zero.borrow_mut().edges.insert(id.to_string(), Rc::clone(&dawg_node));
                    #[cfg(feature = "threading")]
                    node_at_zero.lock().unwrap().edges.insert(id.to_string(), Arc::clone(&dawg_node));
                }
            }

            drop(ref_node);
            nodes.push(dawg_node);
        }




        // we know that node at 0 has 3 direct children, and 5 children in total, of all of them only 3 are terminals
        // and the terminals are on nodes with id = [4, 5, 6]
        let root_node = nodes.get(0).unwrap();

        #[cfg(feature = "threading")]
        let mut root_node = root_node.lock().unwrap();
        #[cfg(not(feature = "threading"))]
        let mut root_node = root_node.borrow_mut();
        
        assert_eq!(root_node.edge_keys().len(), 3);
        for id in ["a", "b", "c"] {
            assert!(root_node.edges().keys().collect::<Vec<_>>().contains(&&id.to_string()));
        }
        assert_eq!(root_node.terminal, false);
        assert_eq!(root_node.num_reachable(), 3);

        #[cfg(feature = "threading")]
        let mut root_nodes_child_two = root_node.edges.get(&"b".to_string()).unwrap().lock().unwrap();
        #[cfg(not(feature = "threading"))]
        let mut root_nodes_child_two = root_node.edges.get(&"b".to_string()).unwrap().borrow_mut();

        assert_eq!(root_nodes_child_two.edge_keys().len(), 2);
        assert_eq!(root_nodes_child_two.terminal, false);
        assert_eq!(root_nodes_child_two.num_reachable(), 3);

        
        #[cfg(feature = "threading")]
        let mut grand_child = root_nodes_child_two.edges.get(&"e".to_string()).unwrap().lock().unwrap();
        #[cfg(not(feature = "threading"))]
        let mut grand_child = root_nodes_child_two.edges.get(&"e".to_string()).unwrap().borrow_mut();
        
        assert_eq!(grand_child.edge_keys().len(), 1);
        assert_eq!(grand_child.terminal, true);
        assert_eq!(grand_child.num_reachable(), 2);

    }
}
