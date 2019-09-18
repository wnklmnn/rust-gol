extern crate graphics;
extern crate image as im;
extern crate piston_window;
extern crate rand;
extern crate rust_gol;

use piston_window::{
    clear, image, G2dTexture, PistonWindow, RenderEvent, Size, Texture, TextureSettings,
    UpdateEvent,
};
use rand::random;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 500;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let win_set = piston_window::WindowSettings::new(
        "mein Fenster",
        Size {
            width: f64::from(WIDTH),
            height: f64::from(HEIGHT),
        },
    );
    let mut field = rust_gol::GoLField::new(WIDTH, HEIGHT);
    let black: im::Rgba<u8> = im::Rgba([0_u8, 0_u8, 0_u8, 255_u8]);
    let white: im::Rgba<u8> = im::Rgba([255_u8, 255_u8, 255_u8, 255_u8]);

    for h in 0..field.get_height() {
        for w in 0..field.get_width() {
            if random::<bool>() {
                field.set_cell_alive(w, h);
            }
        }
    }

    let mut win: PistonWindow = win_set.exit_on_esc(true).build()?;

    let mut can = im::ImageBuffer::new(WIDTH, HEIGHT);
    let mut texture: G2dTexture =
        Texture::from_image(&mut win.factory, &can, &TextureSettings::new())?;

    while let Some(event) = win.next() {
        if event.update_args().is_some() {
            field = field.calc_next_iteration(&rust_gol::EdgeBehavior::Wrapping);
        }

        if event.render_args().is_some() {
            for h in 0..field.get_height() {
                for w in 0..field.get_width() {
                    if field.get_cell(w, h) == rust_gol::CellState::Alive {
                        can.put_pixel(w, h, black);
                    } else {
                        can.put_pixel(w, h, white);
                    }
                }
            }

            texture.update(&mut win.encoder, &can).unwrap();
            win.draw_2d(&event, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }

    Ok(())
}
