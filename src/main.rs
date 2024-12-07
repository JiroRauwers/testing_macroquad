use macroquad::prelude::*;

mod falling_sand;
mod life;

fn main() {
    life::main();
    falling_sand::main();
}
