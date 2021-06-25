pub mod label;
pub mod progress_bar;
pub trait Renderable {
    fn draw(&self, x : usize, y : usize);
}