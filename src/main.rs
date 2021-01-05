use rand::Rng;
use rust_roguelike_like::direction::Dir;
use rust_roguelike_like::field::Field;
use rust_roguelike_like::item::Item;
use rust_roguelike_like::message::Message;
use rust_roguelike_like::player::Player;
use sdl2;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadSurface};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::path::Path;
use std::thread;
use std::time::Duration;

const TITLE: &'static str = "RUST ROGUELIKE LIKE";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const GRID_SIZE: u32 = 100;
const FIELD_SIZE_X: usize = 12;
const FIELD_SIZE_Y: usize = 5;
const ITEM_QUANTITY: u32 = 5;
const ITEM_TYPE: u32 = 3;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;
    let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_ctx.event_pump()?;

    let _image_ctx = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsys
        .window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let player_img = Path::new("assets/image/player.png");
    let player_surface = Surface::from_file(&player_img)?;
    let player_tex = texture_creator
        .create_texture_from_surface(&player_surface)
        .map_err(|e| e.to_string())?;

    let lawn_img = Path::new("assets/image/lawn.png");
    let lawn_surface = Surface::from_file(&lawn_img)?;
    let lawn_tex = texture_creator
        .create_texture_from_surface(&lawn_surface)
        .map_err(|e| e.to_string())?;

    let apple_img = Path::new("assets/image/apple.png");
    let apple_surface = Surface::from_file(&apple_img)?;
    let apple_tex = texture_creator
        .create_texture_from_surface(&apple_surface)
        .map_err(|e| e.to_string())?;

    let orange_img = Path::new("assets/image/orange.png");
    let orange_surface = Surface::from_file(&orange_img)?;
    let orange_tex = texture_creator
        .create_texture_from_surface(&orange_surface)
        .map_err(|e| e.to_string())?;

    let lemon_img = Path::new("assets/image/lemon.png");
    let lemon_surface = Surface::from_file(&lemon_img)?;
    let lemon_tex = texture_creator
        .create_texture_from_surface(&lemon_surface)
        .map_err(|e| e.to_string())?;

    let mut rng = rand::thread_rng();

    let player_init_x = rng.gen_range(0..FIELD_SIZE_X);
    let player_init_y = rng.gen_range(0..FIELD_SIZE_Y);
    let mut player = Player::new(player_init_x as u32, player_init_y as u32);

    let mut field = Field::new();

    let mut item_cnt = 0;

    while item_cnt < ITEM_QUANTITY {
        let item_x = rng.gen_range(0..FIELD_SIZE_X);
        let item_y = rng.gen_range(0..FIELD_SIZE_Y);
        if item_x == player_init_x && item_y == player_init_y {
            continue;
        }
        let item_type = match rng.gen_range(0..ITEM_TYPE) {
            0 => Item::Apple,
            1 => Item::Orange,
            2 => Item::Lemon,
            _ => panic!("generate item_type number outside of range"),
        };
        match field.get_grid(item_x, item_y).create_item(Some(item_type)) {
            Ok(_) => {
                item_cnt += 1;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    let mut player_dest = Rect::new(
        player.get_grid(&mut field).get_pos().x as i32,
        player.get_grid(&mut field).get_pos().y as i32,
        GRID_SIZE,
        GRID_SIZE,
    );

    let mut field_dests = [[Rect::new(0, 0, 0, 0); FIELD_SIZE_X]; FIELD_SIZE_Y];

    for i in 0..FIELD_SIZE_Y {
        for j in 0..FIELD_SIZE_X {
            field_dests[i][j] = Rect::new(
                field.get_grid(j, i).get_pos().x as i32,
                field.get_grid(j, i).get_pos().y as i32,
                GRID_SIZE,
                GRID_SIZE,
            );
        }
    }

    let font_path = Path::new("assets/font/PixelMplus12-Regular.ttf");
    let font = ttf_ctx.load_font(font_path, 100)?;

    let mut msg = Message::new();
    msg.push(String::from("ゲームスタート!"));

    'running: loop {
        //背景の描画
        canvas.set_draw_color(Color::RGB(60, 60, 60));
        canvas.clear();

        //フィールドとアイテムの描画
        for i in 0..FIELD_SIZE_Y {
            for j in 0..FIELD_SIZE_X {
                canvas.copy(&lawn_tex, None, field_dests[i][j])?;

                match field.get_grid(j, i).get_item() {
                    Some(Item::Apple) => {
                        canvas.copy(&apple_tex, None, field_dests[i][j])?;
                    }
                    Some(Item::Orange) => {
                        canvas.copy(&orange_tex, None, field_dests[i][j])?;
                    }
                    Some(Item::Lemon) => {
                        canvas.copy(&lemon_tex, None, field_dests[i][j])?;
                    }
                    None => {}
                }
            }
        }

        //プレイヤーの描画
        canvas.copy(&player_tex, None, player_dest)?;

        //メッセージの描画
        for (i, message) in msg.messages.iter().enumerate() {
            let font_surface = font
                .render(message)
                .blended(Color::RGB(255, 255, 255))
                .map_err(|e| e.to_string())?;
            let font_tex = texture_creator
                .create_texture_from_surface(&font_surface)
                .map_err(|e| e.to_string())?;
            //TODO:文字数に合わせてdestを作成する
            let font_dest = Rect::new(10, 550 + 50 * (i as i32), 400, 40);
            canvas.copy(&font_tex, None, font_dest)?;
        }

        canvas.present();

        for ev in event_pump.poll_iter() {
            match ev {
                Event::Quit { .. } => break 'running,
                Event::KeyUp {
                    keycode: Some(k), ..
                } => match k {
                    Keycode::Escape => break 'running,
                    Keycode::W | Keycode::Up => {
                        player.set_pos(Dir::Up);
                        if let Some(item_name) = player.take_item(&mut field) {
                            msg.push(format!("{}を手に入れた。", item_name));
                        }
                    }
                    Keycode::S | Keycode::Down => {
                        player.set_pos(Dir::Down);
                        if let Some(item_name) = player.take_item(&mut field) {
                            msg.push(format!("{}を手に入れた。", item_name));
                        }
                    }
                    Keycode::A | Keycode::Left => {
                        player.set_pos(Dir::Left);
                        if let Some(item_name) = player.take_item(&mut field) {
                            msg.push(format!("{}を手に入れた。", item_name));
                        }
                    }
                    Keycode::D | Keycode::Right => {
                        player.set_pos(Dir::Right);
                        if let Some(item_name) = player.take_item(&mut field) {
                            msg.push(format!("{}を手に入れた。", item_name));
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        player_dest.set_x(player.get_grid(&mut field).get_pos().x as i32);
        player_dest.set_y(player.get_grid(&mut field).get_pos().y as i32);

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
