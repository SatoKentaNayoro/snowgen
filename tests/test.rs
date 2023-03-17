use snowflake_rs::{Epoch, SnowflakeBuilder};

#[test]
fn test_snowflake_generation() {
    let snowflake = SnowflakeBuilder::new()
        .node_id(1)
        .epoch(Epoch::SystemTime(std::time::UNIX_EPOCH))
        .build()
        .unwrap();

    let id1 = snowflake.next_id().unwrap();
    let id2 = snowflake.next_id().unwrap();
    assert_ne!(id1, id2);
}