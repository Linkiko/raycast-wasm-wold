use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::map::SCREEN_HEIGHT;
use crate::map::SCREEN_WIDTH;

mod map;
mod utils;

use crate::utils::{Point, Vector};
use map::MapBuilder;
extern crate chrono;
extern crate serde;
extern crate web_sys;
use wasm_bindgen::prelude::Closure;

macro_rules! log {
    ( $( $t:tt )* ) => {
        unsafe {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
}

pub struct GameState {
    pub pos: Point,
    pub map: MapBuilder,
    pub plane: Vector,
    pub dir: Vector,
    pub move_speed: f64,
    pub rotation_speed: f64,
    pub current_frame_time: f64,
}

impl GameState {
    pub fn handle_keys_input(&mut self, input_arr: &RefMut<Vec<bool>>) {
        // W
        if input_arr[87] {
            if self.map.map[(self.pos.x + self.dir.x * self.move_speed) as usize]
                [self.pos.y as usize]
                == 0
            {
                self.pos.x += self.dir.x * self.move_speed;
            }
            if self.map.map[(self.pos.x) as usize]
                [(self.pos.y + self.dir.y * self.move_speed) as usize]
                == 0
            {
                self.pos.y += self.dir.y * self.move_speed;
            }
        }
        if input_arr[83] {
            if self.map.map[(self.pos.x - self.dir.x * self.move_speed) as usize]
                [self.pos.y as usize]
                == 0
            {
                self.pos.x -= self.dir.x * self.move_speed;
            }
            if self.map.map[(self.pos.x) as usize]
                [(self.pos.y - self.dir.y * self.move_speed) as usize]
                == 0
            {
                self.pos.y -= self.dir.y * self.move_speed;
            }
        }
        if input_arr[65] {
            let old_dir_x = self.dir.x.clone();
            let inverse_rotation_speed = -1.0 * self.rotation_speed;
            self.dir.x = self.dir.x * inverse_rotation_speed.cos()
                - self.dir.y * inverse_rotation_speed.sin();
            self.dir.y = old_dir_x * inverse_rotation_speed.sin()
                + self.dir.y * inverse_rotation_speed.cos();
            let old_plane_x = self.plane.x.clone();
            self.plane.x = self.plane.x * inverse_rotation_speed.cos()
                - self.plane.y * inverse_rotation_speed.sin();
            self.plane.y = old_plane_x * inverse_rotation_speed.sin()
                + self.plane.y * inverse_rotation_speed.cos();
        }
        if input_arr[68] {
            let old_dir_x = self.dir.x.clone();
            self.dir.x =
                self.dir.x * self.rotation_speed.cos() - self.dir.y * self.rotation_speed.sin();
            self.dir.y =
                old_dir_x * self.rotation_speed.sin() + self.dir.y * self.rotation_speed.cos();
            let old_plane_x = self.plane.x.clone();
            self.plane.x =
                self.plane.x * self.rotation_speed.cos() - self.plane.y * self.rotation_speed.sin();
            self.plane.y =
                old_plane_x * self.rotation_speed.sin() + self.plane.y * self.rotation_speed.cos();
        }
    }
}
// A macro to provide `println!(..)`-style syntax for `console.log` logging.

fn get_window() -> Option<web_sys::Window> {
    web_sys::window()
}

fn document() -> Option<web_sys::Document> {
    get_window().and_then(|win| win.document())
}

pub const MAP_RECT_SIZE: f64 = 7.0;

pub fn set_canevas() -> web_sys::HtmlCanvasElement {
    let canvas = document().unwrap().get_element_by_id("canvas").unwrap();
    log!("{:?}", "Loading canevas");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(SCREEN_WIDTH);
    canvas.set_height(SCREEN_HEIGHT);
    return canvas;
}

pub fn draw_map_info(context: &web_sys::CanvasRenderingContext2d, map_builder: &MapBuilder) {
    for i in 0..map_builder.height {
        for j in 0..map_builder.width {
            if map_builder.map[i][j] > 0 {
                context.fill_rect(
                    i as f64 * MAP_RECT_SIZE,
                    j as f64 * MAP_RECT_SIZE,
                    MAP_RECT_SIZE,
                    MAP_RECT_SIZE,
                )
            }
        }
    }
}

pub fn main_loop(
    context: &web_sys::CanvasRenderingContext2d,
    // player_pos: &mut Point,
    mut game_state: RefMut<GameState>,
    input_arr: RefMut<Vec<bool>>,
) {
    game_state.handle_keys_input(&input_arr);
    let new_frame_time = get_window().unwrap().performance().unwrap().now();
    let frame_time = (new_frame_time - game_state.current_frame_time) / 1000.0;
    log!("fps: {}", 1.0 / frame_time);
    game_state.current_frame_time = new_frame_time;
    game_state.move_speed = 3.0 * frame_time;
    game_state.rotation_speed = 3.0 * frame_time;
    // Clean
    context.clear_rect(0.0, 0.0, SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
    context.set_fill_style(&"rgb(255,255,255)".into());
    // Draw
    context.begin_path();
    // if player_pos.x < 9.0 {
    //     player_pos.x += 0.1;
    // }
    draw_game(context, &game_state);
    context.fill();

    context.begin_path();
    draw_map_info(&context, &game_state.map);
    context.fill();

    context.begin_path();
    context
        .arc(
            game_state.pos.x * MAP_RECT_SIZE,
            game_state.pos.y * MAP_RECT_SIZE,
            2.0,
            0.0,
            2.0 * std::f64::consts::PI,
        )
        .unwrap();
    context.fill();
    context.begin_path();
    context.set_stroke_style(&"red".into());
    context.move_to(
        game_state.pos.x * MAP_RECT_SIZE,
        game_state.pos.y * MAP_RECT_SIZE,
    );
    context.line_to(
        (game_state.pos.x + game_state.dir.x) * MAP_RECT_SIZE,
        (game_state.pos.y + game_state.dir.y) * MAP_RECT_SIZE,
    );
    context.stroke();
    log!("drawing player");
}

fn draw_game(context: &web_sys::CanvasRenderingContext2d, game_state: &RefMut<GameState>) -> () {
    log!(
        "dir x : {:?}, dir y: {:?}",
        game_state.dir.x,
        game_state.dir.y
    );
    log!(
        "pos x : {:?}, pos y: {:?}",
        game_state.pos.x,
        game_state.pos.y
    );

    let time = 0.0;
    let previous_time = 0.0;
    for x in 0..SCREEN_WIDTH {
        context.set_stroke_style(&"rgb(255,255,255)".into());
        let camera_x = 2.0 * x as f64 / (SCREEN_WIDTH as f64 - 1.0);
        let mut ray_dir = Vector {
            x: game_state.dir.x + game_state.plane.x * camera_x,
            y: game_state.dir.y + game_state.plane.y * camera_x,
        };
        let mut map_pos = Point {
            x: game_state.pos.x as i32 as f64,
            y: game_state.pos.y as i32 as f64,
        };
        let mut side_dist = Vector { x: 0.0, y: 0.0 };
        let delta_dist = Vector {
            x: (1.0 / ray_dir.x).abs(),
            y: (1.0 / ray_dir.y).abs(),
        };
        let mut step = Point { x: 0.0, y: 0.0 };
        let mut hit = false;
        let mut side = 0;
        if ray_dir.x < 0.0 {
            step.x = -1.0;
            side_dist.x = (game_state.pos.x - map_pos.x) * delta_dist.x;
        } else {
            step.x = 1.0;
            side_dist.x = (map_pos.x + 1.0 - game_state.pos.x) * delta_dist.x;
        }
        if ray_dir.y < 0.0 {
            step.y = -1.0;
            side_dist.y = (game_state.pos.y - map_pos.y) * delta_dist.y;
        } else {
            step.y = 1.0;
            side_dist.y = (map_pos.y + 1.0 - game_state.pos.y) * delta_dist.y;
        }
        while hit == false {
            //jump to next map square, OR in x-direction, OR in y-direction
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_pos.x += step.x;
                side = 0;
            } else {
                side_dist.y += delta_dist.y;
                map_pos.y += step.y;
                side = 1;
            }
            //Check if ray has hit a wall
            if game_state.map.map[map_pos.x as usize][map_pos.y as usize] > 0 {
                hit = true;
            }
        }
        let mut perp_wall_dist = 0.0;
        if side == 0 {
            perp_wall_dist = ((map_pos.x - game_state.pos.x) + (1.0 - step.x) / 2.0) / ray_dir.x;
        } else {
            perp_wall_dist = ((map_pos.y - game_state.pos.y) + (1.0 - step.y) / 2.0) / ray_dir.y;
        }
        let color = match game_state.map.map[map_pos.x as usize][map_pos.y as usize] {
            1 => "red",
            2 => "green",
            3 => "blue",
            4 => "white",
            _ => "yellow",
        };
        context.set_stroke_style(&color.into());
        //Calculate height of line to draw on screen
        let line_height = (SCREEN_HEIGHT as f64 / perp_wall_dist) as f64;
        //calculate lowest and highest pixel to fill in current stripe
        let mut draw_start: i32 =
            ((-1.0 * line_height as f64) / 2.0 + SCREEN_HEIGHT as f64 / 2.0) as i32;
        if draw_start < 0 {
            draw_start = 0;
        }
        let mut draw_end: i32 = (line_height as f64 / 2.0 + SCREEN_HEIGHT as f64 / 2.0) as i32;
        if draw_end >= SCREEN_HEIGHT as i32 {
            draw_end = SCREEN_HEIGHT as i32 - 1;
        }
        context.begin_path();
        context.move_to(x as f64, draw_start as f64);
        context.line_to(x as f64, draw_end as f64);
        context.stroke();
    }
}

#[wasm_bindgen]
pub fn start(val: &JsValue) {
    let canvas = set_canevas();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let map_builder: MapBuilder = val.into_serde().unwrap();
    let game_state: Rc<RefCell<GameState>> = Rc::new(RefCell::new(GameState {
        pos: Point { x: 2.0, y: 2.0 },
        map: map_builder,
        plane: Vector { x: 0.0, y: 0.90 },
        dir: Vector { x: 1.0, y: 1.0 },
        move_speed: 5.0,
        rotation_speed: 3.0,
        current_frame_time: get_window().unwrap().performance().unwrap().now(),
    }));
    // Cloning only the ptr on the heap and not he whole struc. Which means the struct is mutable
    let game_state_for_key_input = game_state.clone();
    // let event_key_callback = move |event: web_sys::KeyboardEvent| -> () {
    //     game_state_for_key_input
    //         .borrow_mut()
    //         .handle_keys_input(event);
    //     // let KEY_STR = "KEY";
    // };
    let input_arr = Rc::new(RefCell::new(vec![false; 255]));
    let input_arr_down = input_arr.clone();
    let key_down_closure =
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            input_arr_down.borrow_mut()[event.key_code() as usize] = true;
            log!("down");
        });
    canvas
        .add_event_listener_with_callback("keydown", key_down_closure.as_ref().unchecked_ref())
        .unwrap();
    key_down_closure.forget();
    let input_arr_up = input_arr.clone();
    let key_up_closure =
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            input_arr_up.borrow_mut()[event.key_code() as usize] = false;
            log!("up");
        });
    canvas
        .add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())
        .unwrap();
    key_up_closure.forget();
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();
    let window = get_window().unwrap();
    let current_game_state = game_state.clone();
    let input_arr_gameloop = input_arr.clone();
    *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        main_loop(
            &context,
            current_game_state.borrow_mut(),
            input_arr_gameloop.borrow_mut(),
        );

        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }) as Box<dyn FnMut()>));
    let window = get_window().unwrap();
    window
        .request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("failed requesting animation frame");
    //.as_ref().dyn_into::<js_sys::Function>().unwrap();
    // Interval::new(15, move || main_loop(&context, &map_builder));
    // loop {
    //     context.begin_path();
    //     if player_pos.x < 9.0 {
    //         player_pos.x += 0.1;
    //     }
    //     draw_map_info(&context, &map_builder);
    //     context
    //         .arc(
    //             player_pos.x * MAP_RECT_SIZE,
    //             player_pos.y * MAP_RECT_SIZE,
    //             2.0,
    //             0.0,
    //             2.0 * std::f64::consts::PI,
    //         )
    //         .unwrap();
    //     log!("drawing player");
    //     context.fill();
    //     context.clear_rect(0.0, 0.0, SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
    // }
}
