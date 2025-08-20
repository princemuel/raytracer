mod world;

use cucumber::World as _;

use crate::world::TestWorld;

#[tokio::main]
async fn main() {
    TestWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("tests/features/")
        .await;
}
