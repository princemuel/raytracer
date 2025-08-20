use cucumber::World;

#[tokio::main]
async fn main() {
    RayTracerWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("tests/features/")
        .await;
}

#[derive(Debug, World)]
pub struct RayTracerWorld {}
impl Default for RayTracerWorld {
    fn default() -> Self { todo!() }
}
