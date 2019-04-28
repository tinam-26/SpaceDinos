use crate::level::{Level, LevelObject};
use std::cmp::{min, max};

//use touch_visualizer::TouchVisualizer;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{ Context, Graphics };
use std::collections::HashMap;
use piston::window::{ Window, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

type AxisValues = HashMap<(i32, u8), f64>;

struct State {
    level: Level,
    cursor: [isize; 2],
    line_mode: bool,
    rect_mode: bool,
    line: [isize; 2],
}

pub fn editor(level: Level) {
    let mut state = State {
        level,
        cursor: [0, 0],
        line_mode: false,
        rect_mode: false,
        line: [0, 0]
    };
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow = WindowSettings::new("piston-example-user_input", [600, 600])
        .exit_on_esc(true).opengl(opengl).build().unwrap();

    let ref mut gl = GlGraphics::new(opengl);

    let mut axis_values: AxisValues = HashMap::new();

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => {
                    state.cursor[1] += 1;
                }
                Key::Down => {
                    state.cursor[1] -= 1;
                }
                Key::Left => {
                    state.cursor[0] -= 1;
                }
                Key::Right => {
                    state.cursor[0] += 1;
                }
                Key::Space => {
                    state.level.toggle_position(state.cursor[0], state.cursor[1]);
                }
                Key::Delete | Key::Backspace => {
                    state.level.get_at_pos(state.cursor[0], state.cursor[1]);
                }
                Key::S => {
                    let result = nfd::dialog_save().filter("json").open().unwrap_or_else(|e| {
                        panic!(e);
                    });

                    if let nfd::Response::Okay(file_path) = result {
                        state.level.save(file_path).ok();
                    }
                }
                Key::O => {
                    let result = nfd::open_file_dialog(Some("json"), None).unwrap_or_else(|e| {
                        panic!(e);
                    });

                    if let nfd::Response::Okay(file_path) = result {
                        state.level = Level::open(file_path).unwrap_or_default();
                    }
                }
                Key::F => {
                    for x in state.cursor[0]..state.cursor[0]+25 {
                        for y in state.cursor[1]-1..=state.cursor[1] {
                            state.level.get_at_pos(x, y);
                        }
                    }
                    
                    state.level.levelObjects.push(
                        LevelObject {
                            name: "floor".to_string(),
                            x: state.cursor[0],
                            y: state.cursor[1]
                        }
                    );
                    state.cursor[0] += 25;
                }
                Key::R => {
                    if !state.line_mode {
                        if !state.rect_mode {
                            state.line[0] = state.cursor[0];
                            state.line[1] = state.cursor[1];
                        } else {
                            // Apply rect
                            for x in min(state.cursor[0], state.line[0])..=max(state.cursor[0], state.line[0]) {
                                for y in min(state.cursor[1], state.line[1])..=max(state.cursor[1], state.line[1]) {
                                    state.level.get_at_pos(x, y);
                                    state.level.levelObjects.push(
                                        LevelObject {
                                            name: "block".to_string(),
                                            x,
                                            y,
                                        }
                                    );
                                }
                            }
                        }
                        state.rect_mode = !state.rect_mode;
                    }
                }
                Key::T => {
                    if !state.line_mode {
                        if !state.rect_mode {
                            state.line[0] = state.cursor[0];
                            state.line[1] = state.cursor[1];
                        } else {
                            // Apply rect
                            for x in min(state.cursor[0], state.line[0])..=max(state.cursor[0], state.line[0]) {
                                for y in min(state.cursor[1], state.line[1])..=max(state.cursor[1], state.line[1]) {
                                    state.level.get_at_pos(x, y);
                                }
                            }
                        }
                        state.rect_mode = !state.rect_mode;
                    }
                }
                Key::L => {
                    if !state.rect_mode {
                        if !state.line_mode {
                            state.line[0] = state.cursor[0];
                            state.line[1] = state.cursor[1];
                        } else {
                            // Apply line
                            if state.cursor[0] == state.line[0] {
                                for y in min(state.cursor[1], state.line[1])..=max(state.cursor[1], state.line[1]) {
                                    // Delete anything in its path
                                    state.level.get_at_pos(state.cursor[0], y);
                                    state.level.levelObjects.push(
                                        LevelObject {
                                            name: "block".to_string(),
                                            x: state.cursor[0],
                                            y,
                                        }
                                    );
                                }
                            } else {
                                let (x0, y0, x1, y1) = if state.line[0] > state.cursor[0] {
                                    (state.cursor[0] as f64, state.cursor[1] as f64, state.line[0] as f64, state.line[1] as f64)
                                } else {
                                    (state.line[0] as f64, state.line[1] as f64, state.cursor[0] as f64, state.cursor[1] as f64)
                                };
                                let deltax = x1 - x0;
                                let deltay = y1 - y0;
                                let deltaerr = (deltay / deltax).abs();
                                let mut error = 0.0;
                                let mut y = y0 as isize;
                                for x in x0 as isize..=x1 as isize {
                                    // Delete anything in its path
                                    state.level.get_at_pos(x, y);
                                    // Add block at x,
                                    state.level.levelObjects.push(
                                        LevelObject {
                                            name: "block".to_string(),
                                            x,
                                            y,
                                        }
                                    );
                                    error += deltaerr as f64;
                                    if error >= 0.5 {
                                        y = y + deltay.signum() as isize;
                                        error -= 1.0;
                                    }
                                }
                            }
                        }
                        state.line_mode = !state.line_mode;
                    }
                }
                Key::K => {
                    if !state.line_mode {
                        state.line[0] = state.cursor[0];
                        state.line[1] = state.cursor[1];
                    } else {
                        // Apply line
                        if state.cursor[0] == state.line[0] {
                            for y in min(state.cursor[1], state.line[1])..=max(state.cursor[1], state.line[1]) {
                                // Delete anything in its path
                                state.level.get_at_pos(state.cursor[0], y);
                            }
                        } else {
                            let (x0, y0, x1, y1) = if state.line[0] > state.cursor[0] {
                                (state.cursor[0] as f64, state.cursor[1] as f64, state.line[0] as f64, state.line[1] as f64)
                            } else {
                                (state.line[0] as f64, state.line[1] as f64, state.cursor[0] as f64, state.cursor[1] as f64)
                            };
                            let deltax = x1 - x0;
                            let deltay = y1 - y0;
                            let deltaerr = (deltay / deltax).abs();
                            let mut error = 0.0;
                            let mut y = y0 as isize;
                            for x in x0 as isize..=x1 as isize {
                                // Delete anything in its path
                                state.level.get_at_pos(x, y);
                                error += deltaerr as f64;
                                if error >= 0.5 {
                                    y = y + deltay.signum() as isize;
                                    error -= 1.0;
                                }
                            }
                        }
                    }
                    state.line_mode = !state.line_mode;
                }
                Key::Escape => {
                    state.line_mode = false;
                }
                _ => {}
            }
        };
        if let Some(args) = e.controller_axis_args() {
            axis_values.insert((args.id, args.axis), args.position);
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                    graphics::clear([1.0; 4], g);
                    draw_state(&state, &window, &c, g);
                }
            );
        }
    }
}

fn draw_obj<G: Graphics>(
    obj: &LevelObject,
    center: f64,
    _window: &Window,
    c: &Context,
    g: &mut G,
) {
    let x = 10.0 * obj.x as f64;
    let y = (-10.0 * obj.y as f64) + center;
    match &obj.name[..] {
        "coin" => {
            graphics::ellipse(
                [0.9, 0.9, 0.0, 1.0],
                graphics::ellipse::circle(x + 5.0, y + 5.0, 5.0),
                c.transform,
                g
            );
        }
        "platform" => {
            graphics::Rectangle::new(
                [0.0, 0.0, 0.0, 1.0]
            ).draw(
                [
                    x,
                    y,
                    10.0,
                    5.0
                ],
                &c.draw_state, c.transform, g
            );
        }
        "spikes" => {
            let line = graphics::Line::new([0.5, 0.5, 0.5, 1.0], 0.5);
            line.draw([
                x,
                y + 9.5,
                x + 10.0,
                y + 9.5
            ], &c.draw_state, c.transform, g);
            for i in 0..3 {
                line.draw([
                    x + 2.5 + (2.5 * i as f64),
                    y + 10.0,
                    x + 2.5 + (2.5 * i as f64),
                    y + 5.0
                ], &c.draw_state, c.transform, g);
            }
        }
        "block" => {
            graphics::Rectangle::new(
                [0.0, 0.0, 0.0, 1.0]
            ).draw(
                [
                    x,
                    y,
                    10.0,
                    10.0
                ],
                &c.draw_state, c.transform, g
            );
        }
        "invisible_block" => {
            let line = graphics::Line::new([1.0, 0.0, 0.0, 1.0], 1.0);
            line.draw([
                x,
                y,
                x + 10.0,
                y + 10.0
            ], &c.draw_state, c.transform, g);
            line.draw([
                x,
                y + 10.0,
                x + 10.0,
                y
            ], &c.draw_state, c.transform, g);
        }
        "floor" => {
            graphics::Rectangle::new(
                [0.3, 0.3, 0.3, 1.0]
            ).draw(
                [
                    x,
                    y,
                    250.0,
                    20.0
                ],
                &c.draw_state, c.transform, g
            );
        }
        "finish_block" => {
            let rect = graphics::Rectangle::new([0.0, 0.0, 0.0, 1.0]);
            for i in 0..4 {
                for j in 0..4 {
                    if (i + j) % 2 == 0 {
                        rect.draw(
                            [
                                x + (10.0 / 4.0) * i as f64,
                                y + (10.0 / 4.0) * j as f64,
                                10.0 / 4.0,
                                10.0 / 4.0,                            
                            ], &c.draw_state, c.transform, g
                        );
                    }
                }
            }
        }
        _ => {}
    }
}

fn draw_state<G: Graphics>(
    state: &State,
    window: &Window,
    c: &Context,
    g: &mut G,
) {
    let draw_size = window.draw_size();
    let rect_border = graphics::Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0);

    let h = draw_size.height as isize;
    let center = ((h + (h % 20)) / 2) as f64;

    // Draw level
    for obj in &state.level.levelObjects {
        draw_obj(obj, center, window, c, g);
    }

    // Calculate cursor screen pos
    let cursor_pos = (10.0 * state.cursor[0] as f64, (-10.0 * state.cursor[1] as f64) + center);
    
    // Draw line if applicable
    if state.line_mode {
        let line_start_x = state.line[0] as f64 * 10.0;
        let line_start_y = state.line[1] as f64 * -10.0 + center; 
        
        let line = graphics::Line::new([1.0, 0.0, 0.0, 1.0], 1.0);
            line.draw([
                line_start_x + 5.0,
                line_start_y + 5.0,
                cursor_pos.0 + 5.0,
                cursor_pos.1 + 5.0
        ], &c.draw_state, c.transform, g);
    }

    // Draw rect if applicable
    if state.rect_mode {
        let line_start_x = state.line[0] as f64 * 10.0;
        let line_start_y = state.line[1] as f64 * -10.0 + center; 
        
        let rect_left = f64::min(line_start_x + 5.0, cursor_pos.0 + 5.0);
        let rect_width = ((line_start_x + 5.0) - (cursor_pos.0 + 5.0)).abs();
        let rect_top = f64::min(line_start_y + 5.0, cursor_pos.1 + 5.0);
        let rect_height = ((line_start_y + 5.0) - (cursor_pos.1 + 5.0)).abs();

        let rect = graphics::Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0);
        rect.draw([
            rect_left,
            rect_top,
            rect_width,
            rect_height
        ], &c.draw_state, c.transform, g);
    }

    // Draw cursor
    rect_border.draw([
        cursor_pos.0,
        cursor_pos.1,
        10.0,
        10.0
    ],
    &c.draw_state, c.transform, g);
}
