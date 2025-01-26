use std::path::PathBuf;

use pixels::{Pixels as PixelBuffer, SurfaceTexture};
use quikpix::Pixels;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn usage() -> ! {
    println!("Usage: quikview [-s <1 | 2 | 3 | ...>] <FILE>");
    std::process::exit(1);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let (filepath, scale) = match args.len() {
        2 => (PathBuf::from(&args[1]), 1),
        4 => {
            if args[1] != "-s" {
                usage();
            }
            let filepath = PathBuf::from(&args[3]);
            let scale: u32 = args[2].parse().expect("failed to parse scale");
            if scale == 0 {
                panic!("scale may not be 0");
            }
            (filepath, scale)
        }
        _ => usage(),
    };

    let filename = filepath.file_name().unwrap().to_str().unwrap().to_owned();
    let pixels = Pixels::read(filepath);
    let logical_width = pixels.width() as u32;
    let logical_height = pixels.height() as u32;
    let physical_width = logical_width * scale;
    let physical_height = logical_height * scale;

    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(physical_width, physical_height))
        .with_resizable(false)
        .with_title(filename)
        .build(&event_loop)
        .expect("failed to create window");

    let surface_texture = SurfaceTexture::new(physical_width, physical_height, &window);
    let mut pixel_buffer = PixelBuffer::new(physical_width, physical_height, surface_texture)
        .expect("failed to create pixel buffer");

    event_loop
        .run(|event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::AboutToWait => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw in
                    // applications which do not always need to. Applications that redraw continuously
                    // can render here instead.
                    // window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let frame = pixel_buffer.frame_mut();
                    for y in 0..logical_height {
                        for x in 0..logical_width {
                            let color = pixels.get(x as usize, y as usize);
                            for py in (y * scale)..(y * scale) + scale {
                                for px in (x * scale)..(x * scale) + scale {
                                    let idx = 4 * ((py * physical_width + px) as usize);
                                    frame[idx] = color.0;
                                    frame[idx + 1] = color.1;
                                    frame[idx + 2] = color.2;
                                    frame[idx + 3] = 0xFF;
                                }
                            }
                        }
                    }

                    pixel_buffer
                        .render()
                        .expect("failed to render pixel buffer");
                }
                _ => (),
            }
        })
        .unwrap();
}
