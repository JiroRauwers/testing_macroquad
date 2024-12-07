use macroquad::prelude::*;

const PIXEL_SIZE: f32 = 10.0;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Material {
    Sand,
    // Water,
    Air,
}

impl Material {
    fn color(&self) -> Color {
        match self {
            Material::Sand => YELLOW,
            Material::Air => BLACK,
        }
    }

    fn fall(&self, x: usize, y: usize, cells: &mut Vec<Vec<Material>>) {
        match self {
            Material::Sand => {
                if y + 1 >= cells.len() {
                    return;
                }

                let below = cells[y + 1][x];
                if below == Material::Air {
                    cells[y + 1][x] = Material::Sand;
                    cells[y][x] = Material::Air;
                    return;
                } else {
                    if y + 2 >= cells.len() {
                        return;
                    }

                    let rDir: i32 = [-1, 1][rand::gen_range(0, 2)];

                    let sub = (x as i32 - rDir) as usize;
                    let sum = (x as i32 + rDir) as usize;

                    // Check for two cells below being air
                    if sub < cells[y + 1].len()
                        && cells[y + 1][sub] == Material::Air
                        && cells[y + 2][sub] == Material::Air
                    {
                        cells[y + 1][sub] = Material::Sand;
                        cells[y][x] = Material::Air;
                        return;
                    }

                    if sum < cells[y + 1].len()
                        && cells[y + 1][sum] == Material::Air
                        && cells[y + 2][sum] == Material::Air
                    // Check for two cells below
                    {
                        cells[y + 1][sum] = Material::Sand;
                        cells[y][x] = Material::Air;
                        return;
                    }
                }
            }
            Material::Air => {}
        }
    }
}

#[macroquad::main("Falling Sand")]
pub async fn main() {
    let w = (screen_width() / PIXEL_SIZE) as usize;
    let h = (screen_height() / PIXEL_SIZE) as usize;

    let mut cells = vec![vec![Material::Air; w]; h];
    let mut buffer = vec![vec![Material::Air; w]; h];

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

    let texture: Texture2D = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        // clear_background(BLACK);

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let x = (mouse_pos.0 / PIXEL_SIZE) as usize;
            let y = (mouse_pos.1 / PIXEL_SIZE) as usize;

            if y < cells.len() && x < cells[y].len() {
                cells[y][x] = Material::Sand;
            }

            println!(
                "Mouse pressed: x: {}, y: {}, MousePos: {:?}",
                x, y, mouse_pos
            );
        }

        let w = image.width();
        let h = image.height();

        for y in 0..h as usize {
            for x in 0..w as usize {
                cells[y][x].fall(x as usize, y as usize, &mut buffer);
            }
        }

        for y in 0..buffer.len() {
            for x in 0..buffer[y].len() {
                cells[y][x] = buffer[y][x];

                image.set_pixel(x as u32, y as u32, buffer[y][x].color());
            }
        }

        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        next_frame().await
    }
}
