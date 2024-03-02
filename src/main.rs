use std::mem::{size_of, size_of_val};
use std::time::{Duration, Instant};

use image::ColorType::Rgba8;
use sdl2::{rect::Rect, render::Canvas, video::Window};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

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

    // Create a font context
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context
        .load_font("res/font/Acme 9 Regular.ttf", 10)
        .expect("INVALID FONT PATH!"); // Specify the path to your font

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame_time = Instant::now();
    let mut frames = 0;
    let mut fps_sum = 0.0;
    let mut frame_time_sum = Duration::new(0, 0);
    const AVERAGE_FRAMES: usize = 20; // Adjust this value to change the number of frames to average over
    let mut average_fps: f64; // Declare average FPS outside the if block
    let mut average_frame_time: Duration; // Declare average frame time outside the if block
    let initial_fps_text = "Average FPS: 0.00";
    let initial_frame_time_text = "Average Frame Time: 0";

    let mut surface_fps = font
        .render(initial_fps_text)
        .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)) // White text color
        .unwrap();
    let mut surface_frame_time = font
        .render(initial_frame_time_text)
        .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)) // White text color
        .unwrap();
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

        /* draw avg fps and frametime in corner */
        if DEBUG {
            // Add FPS and frame time to the running sum
            fps_sum += fps;
            frame_time_sum += frame_time;

            // Increment frame count
            frames += 1;

            // If we have accumulated enough frames, calculate the average
            if frames >= AVERAGE_FRAMES {
                average_fps = fps_sum / frames as f64;
                average_frame_time = frame_time_sum / frames as u32;

                // Reset sums and frame count
                fps_sum = 0.0;
                frame_time_sum = Duration::new(0, 0);
                frames = 0;

                // Create surfaces for displaying average FPS and frame time
                let fps_text = format!("Avg FPS: {:.2}", average_fps);
                let frame_time_text = format!("Avg Frame Time: {:?}", average_frame_time);
                surface_fps = font
                    .render(&fps_text)
                    .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)) // White text color
                    .unwrap();
                surface_frame_time = font
                    .render(&frame_time_text)
                    .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)) // White text color
                    .unwrap();
            }

            // Draw FPS and frame time to the corner of the window if displaying average
            draw_text(&mut canvas, &surface_fps, 10, 10);
            draw_text(&mut canvas, &surface_frame_time, 10, 50);
        }

        // Update the window
        canvas.present();

        // Add a slight delay to avoid consuming too much CPU
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}

fn draw_text(canvas: &mut Canvas<Window>, surface: &sdl2::surface::Surface, x: i32, y: i32) {
    let binding = canvas.texture_creator();
    let texture = binding.create_texture_from_surface(surface).unwrap();
    let texture_query = texture.query();
    let dest_rect = Rect::new(x, y, texture_query.width, texture_query.height);
    canvas.copy(&texture, None, dest_rect).unwrap();
}
