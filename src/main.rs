mod render;
mod simulation;

#[macroquad::main("Liquid simulation")]
async fn main() {
    render::run().await;
}
