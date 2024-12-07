use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, InputHandler, Skin};

const PIXEL_SIZE: f32 = 10.0;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Material {
    Sand,
    Water,
    Air,
}

impl Material {
    fn color(&self) -> Color {
        match self {
            Material::Sand => YELLOW,
            Material::Water => BLUE,
            Material::Air => BLACK,
        }
    }

    fn fall(&self, x: usize, y: usize, cells: &mut Vec<Vec<Material>>) {
        match self {
            Material::Water => {
                if y + 1 >= cells.len() {
                    return;
                }

                fn is_lighter(a: Material) -> bool {
                    matches!(a, Material::Air)
                }

                let below = cells[y + 1][x];
                if is_lighter(below) {
                    let temp = cells[y + 1][x];
                    cells[y + 1][x] = *self;
                    cells[y][x] = temp;
                    return;
                } else {
                    let r_dir: i32 = [-1, 1][rand::gen_range(0, 2)];

                    let sub: usize = (x as i32 - r_dir) as usize;
                    let sum = (x as i32 + r_dir) as usize;

                    // -- Check for two adjacent cells below being air
                    if sub < cells[y + 1].len() && is_lighter(cells[y + 1][sub]) {
                        let temp = cells[y + 1][sub];
                        cells[y + 1][sub] = *self;
                        cells[y][x] = temp;
                        return;
                    }

                    if sum < cells[y + 1].len() && is_lighter(cells[y + 1][sum]) {
                        let temp = cells[y + 1][sum];
                        cells[y + 1][sum] = *self;
                        cells[y][x] = temp;
                        return;
                    }

                    // -- Check for a adjacent cell being air
                    if sub < cells[y].len() && is_lighter(cells[y][sub]) {
                        let temp = cells[y][sub];
                        cells[y][sub] = *self;
                        cells[y][x] = temp;
                        return;
                    }

                    if sum < cells[y].len() && is_lighter(cells[y][sum]) {
                        let temp = cells[y][sum];
                        cells[y][sum] = *self;
                        cells[y][x] = temp;
                        return;
                    }
                }
            }

            Material::Sand => {
                if y + 1 >= cells.len() {
                    return;
                }

                fn is_lighter(a: Material) -> bool {
                    matches!(a, Material::Air) || matches!(a, Material::Water)
                }

                let below = cells[y + 1][x];
                if is_lighter(below) {
                    let temp = cells[y + 1][x];
                    cells[y + 1][x] = *self;
                    cells[y][x] = temp;
                    return;
                } else {
                    if y + 2 >= cells.len() {
                        return;
                    }

                    let r_dir: i32 = [-1, 1][rand::gen_range(0, 2)];

                    let sub = (x as i32 - r_dir) as usize;
                    let sum = (x as i32 + r_dir) as usize;

                    // Check for two cells below being air
                    if sub < cells[y + 1].len()
                        && is_lighter(cells[y + 1][sub])
                        && is_lighter(cells[y + 2][sub])
                    {
                        let temp = cells[y + 1][sub];
                        cells[y + 1][sub] = *self;
                        cells[y][x] = temp;
                        return;
                    }

                    if sum < cells[y + 1].len()
                        && is_lighter(cells[y + 1][sum])
                        && is_lighter(cells[y + 2][sum])
                    {
                        let temp = cells[y + 1][sum];
                        cells[y + 1][sum] = *self;
                        cells[y][x] = temp;
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

    let mut selected_material = Material::Sand;

    loop {
        // root_ui().push_skin(&Material::get_ui_skin());
        // clear_background(BLACK);

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

        let start_ui = Vec2::new(10., 10.);
        let size_ui = Vec2::new(450., 200.);
        root_ui().window(hash!(), start_ui, Vec2::new(100., 200.), |ui| {
            ui.label(None, &format!("Current material: {:?}", selected_material));

            if ui.button(None, "Sand") {
                selected_material = Material::Sand;
            }

            if ui.button(None, "Water") {
                selected_material = Material::Water;
            }
        });

        println!("Mouse captured root {:?}", root_ui().is_mouse_captured());

        let mouse_pos = mouse_position();

        let is_mouse_in_ui = mouse_pos.0 >= start_ui.x
            && mouse_pos.0 <= start_ui.x + size_ui.x
            && mouse_pos.1 >= start_ui.y
            && mouse_pos.1 <= start_ui.y + size_ui.y;

        if is_mouse_button_down(MouseButton::Left) && !is_mouse_in_ui {
            let x = (mouse_pos.0 / PIXEL_SIZE) as usize;
            let y = (mouse_pos.1 / PIXEL_SIZE) as usize;

            if y < cells.len() && x < cells[y].len() {
                cells[y][x] = selected_material;
            }
        }

        // root_ui().pop_skin();
        next_frame().await
    }
}
