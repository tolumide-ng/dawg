#[cfg(test)]
#[cfg(not(feature = "threading"))]
mod test_dawg_node {
    use crate::node::node::{self, DawgWrapper};

    #[test]
    fn initializes_a_dawgwrapper_with_id_0() {
        let dawg_wrapper = DawgWrapper::new();

        assert_eq!(dawg_wrapper.next_id, 0);
    }

    #[test]
    fn dawgwrapper_increases_id_of_new_dawgnodes() {
        let mut dawg_wrapper = DawgWrapper::new();

        assert_eq!(dawg_wrapper.next_id, 0);

        let node_zero = dawg_wrapper.create();
        assert_eq!(node_zero.borrow().id, 0);
        assert_eq!(node_zero.borrow().count, 0);
        assert_eq!(node_zero.borrow().terminal, false);
        assert_eq!(node_zero.borrow().edges.keys().len(), 0);

        assert_eq!(dawg_wrapper.next_id, 1);

        let node_one = dawg_wrapper.create();

        assert_eq!(node_one.borrow().id, 1);
        assert_eq!(node_one.borrow().count, 0);
        assert_eq!(node_one.borrow().terminal, false);
        assert_eq!(node_one.borrow().edges.keys().len(), 0);
        assert_eq!(node_one.borrow_mut().num_reachable(), 0);
    }

    // #[test]
    // fn find_a_specific_dawgnode_in_a_vector_of_dawgnodes() {
    //     let mut dawg_wrapper = DawgWrapper::new();

    //     assert_eq!(dawg_wrapper.next_id, 0);

    //     let node_zero = dawg_wrapper.create();
    //     // node_zero.count = 1;
    // }
}

#[cfg(test)]
#[cfg(feature = "threading")]
mod test_dawg_node {
    use crate::node::node::{self, DawgWrapper};

    #[test]
    fn initializes_a_dawgwrapper_with_id_0() {
        let dawg_wrapper = DawgWrapper::new();

        assert_eq!(dawg_wrapper.next_id, 0);
    }

    #[test]
    fn dawgwrapper_increases_id_of_new_dawgnodes() {
        let mut dawg_wrapper = DawgWrapper::new();

        assert_eq!(dawg_wrapper.next_id, 0);

        let node_zero = dawg_wrapper.create();
        assert_eq!(node_zero.lock().unwrap().id, 0);
        assert_eq!(node_zero.lock().unwrap().count, 0);
        assert_eq!(node_zero.lock().unwrap().terminal, false);
        assert_eq!(node_zero.lock().unwrap().edges.keys().len(), 0);

        assert_eq!(dawg_wrapper.next_id, 1);

        let node_one = dawg_wrapper.create();

        assert_eq!(node_one.lock().unwrap().id, 1);
        assert_eq!(node_one.lock().unwrap().count, 0);
        assert_eq!(node_one.lock().unwrap().terminal, false);
        assert_eq!(node_one.lock().unwrap().edges.keys().len(), 0);
        assert_eq!(node_one.lock().unwrap().num_reachable(), 0);
    }
}
