// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
// This file is a part of `piet-cosmic-text`.
//
// `piet-cosmic-text` is free software: you can redistribute it and/or modify it under the
// terms of either:
//
// * GNU Lesser General Public License as published by the Free Software Foundation, either
//   version 3 of the License, or (at your option) any later version.
// * Mozilla Public License as published by the Mozilla Foundation, version 2.
// * The Patron License (https://github.com/notgull/piet-cosmic-text/blob/main/LICENSE-PATRON.md)
//   for sponsors and contributors, who can ignore the copyleft provisions of the above licenses
//   for this project.
//
// `piet-cosmic-text` is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public License or the Mozilla Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and the Mozilla
// Public License along with `piet-cosmic-text`. If not, see <https://www.gnu.org/licenses/>.

//! An example for drawing some basic text using `piet-cosmic-text` and `softbuffer`.

use cosmic_text::SwashCache;

use piet::{FontFamily, Text as _, TextLayoutBuilder as _};
use piet_cosmic_text::Text;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() {
    let mut width = 720;
    let mut height = 480;

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("piet text example")
        .with_inner_size(LogicalSize::new(width as u32, height as u32))
        .build(&event_loop)
        .unwrap();

    let mut context = unsafe { softbuffer::GraphicsContext::new(&window, &window).unwrap() };

    let mut text = Text::new();
    let mut buffer = vec![0u32; width * height];

    let mut swash_cache = SwashCache::new();

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                width = size.width as usize;
                height = size.height as usize;
                buffer.resize(width * height, 0);
            }

            Event::RedrawRequested(_) => {
                // Fill buffer with white.
                buffer.fill(0x00FFFFFF);

                // Calculate text layout.
                let text_layout = text
                    .new_text_layout("Line #1\nLine #2\nمرحبا بالعالم\n💀 💀 💀\nThis is an exceptionally long line! foobar foobar foobar foobar")
                    .font(FontFamily::SANS_SERIF, 24.0)
                    .max_width(width as _)
                    .build()
                    .unwrap();

                // Draw pixels.
                text.with_font_system_mut(|font_system| {
                    text_layout.buffer().draw(
                        font_system,
                        &mut swash_cache,
                        cosmic_text::Color::rgba(0, 0, 0, 0xFF),
                        |x, y, _, _, color| {
                            if x < 0 || y < 0 {
                                return;
                            }

                            let pixel_start = (y as usize * width) + x as usize;
                            let rgba = {
                                let alpha_filter = (color.a() as f32) / 255.0;

                                let cvt = |x| {
                                    (((x as f32) * alpha_filter)
                                        + (255.0 * (1.0 - alpha_filter)))
                                        as u32
                                };

                                ((cvt(color.r())) << 16)
                                    | ((cvt(color.g())) << 8)
                                    | (cvt(color.b()))
                            };

                            if let Some(pixel) = buffer.get_mut(pixel_start) {
                                *pixel = rgba;
                            }
                        },
                    );
                });

                // Push buffer to softbuffer.
                context.set_buffer(&buffer, width as _, height as _);
            }

            _ => (),
        }
    });
}
