use crate::tests::setup::QueryMock;

use super::setup::MulticallTestbed;

#[test]
fn init() {
    MulticallTestbed::new();
}
#[test]
fn chain_batch_regular() {
    let testbed = MulticallTestbed::new();

    let msg = QueryMock::Regular {
        prop: "works".to_string(),
    };

    let queries = vec![msg.clone(), msg.clone()];

    let responses = testbed.batch_chain(queries);
    assert!(responses.len() == 2);
}
#[test]
fn has_exception() {
    let testbed = MulticallTestbed::new();

    let msg = QueryMock::Regular {
        prop: "works".to_string(),
    };
    let err = QueryMock::Exception {};

    let queries = vec![msg.clone(), err.clone()];

    let responses = testbed.batch_chain(queries);

    assert!(responses.len() == 2);
    assert!(responses[1].error.is_some());
}
