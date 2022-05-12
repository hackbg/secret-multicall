use std::collections::BTreeMap;

use crate::tests::setup::QueryMock;

use super::setup::MulticallTestbed;

#[test]
fn init() {
    MulticallTestbed::new();
}
#[test]
fn chain_batch() {
    let testbed = MulticallTestbed::new();

    let msg = QueryMock::Regular {
        prop: "works".to_string(),
    };

    let queries = vec![msg.clone(), msg.clone()];

    let responses = testbed.batch_chain(queries);
    assert!(responses.len() == 2);
}
#[test]
fn map_batch() {
    let testbed = MulticallTestbed::new();

    let msg = QueryMock::Regular {
        prop: "works".to_string(),
    };

    let mut map = BTreeMap::new();

    map.insert("key1".to_string(), msg.clone());
    map.insert("key2".to_string(), msg.clone());

    let responses = testbed.batch_map(map);

    assert!(responses.len() == 2);
}
