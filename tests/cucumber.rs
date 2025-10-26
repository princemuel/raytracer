#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod steps;
mod support;

use cucumber::World as _;

use crate::support::world::TestWorld;

#[tokio::main]
async fn main() {
    TestWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("tests/features/")
        .await;
}
