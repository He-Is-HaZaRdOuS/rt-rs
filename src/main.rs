use std::time::{Duration, Instant};

/*idea*/

use image::ColorType::Rgba8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use crate::test::*;

mod matrix3;
mod matrix4;
mod renderer;
mod test;
mod vector3;
mod vector4;

/* toggles constructor messages */
const DEBUG: bool = false;

fn main() {
    trace_rays();
}

fn trace_rays() {
    println!("DEBUG: {}", DEBUG);
    test_vector();
    println!();
    test_matrix();
    println!();
    test_json_vector();
    println!();
    test_json_matrix();
    println!();
    sdl2();
}

pub fn sdl2() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 512;
    let height = 512;

    // Create a window
    let window = video_subsystem
        .window("SDL2", width, height)
        .position_centered()
        .build()
        .unwrap();

    // Create a canvas for rendering
    let mut canvas = window.into_canvas().build().unwrap();

    // Create an image buffer (RGBA8 format)
    let mut s_renderer = renderer::Renderer::new(width, height);
    //let mut image_buffer: RgbaImage = s.render();
    let mut raw_pixels: Vec<u8>;

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame_time = Instant::now();

    //image_buffer = s.render();
    raw_pixels = s_renderer.render().into_raw();

    'running: loop {
        let frame_start_time = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    /* save last image buffer to output.png before exiting */
                    image::save_buffer("output.png", &*raw_pixels, width, height, Rgba8)
                        .expect("COULDN'T SAVE IMAGE BUFFER BEFORE EXITING!");
                    break 'running;
                }
                _ => { /* do nothing */ }
            }
        }

        //image_buffer = s.render();
        raw_pixels = s_renderer.render().into_raw();
        let surface = sdl2::surface::Surface::from_data(
            &mut raw_pixels,
            width,
            height,
            width * PixelFormatEnum::RGBA32.byte_size_per_pixel() as u32,
            PixelFormatEnum::RGBA32,
        )
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();

        // Clear the canvas
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the texture to the canvas
        canvas
            .copy(&texture, None, Some(Rect::new(0, 0, width, height)))
            .unwrap();

        // Calculate frame time and FPS
        let frame_time = frame_start_time.duration_since(last_frame_time);
        last_frame_time = frame_start_time;
        let fps = 1.0 / frame_time.as_secs_f64();
        println!("FPS: {:?}, Frametime: {:?}", fps, frame_time);

        // Update the window
        canvas.present();

        // Add a slight delay to avoid consuming too much CPU
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}
