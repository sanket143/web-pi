use dpi::{LogicalPosition, LogicalSize};
use gtk::builders::WindowBuilder;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use wry::{Rect, WebViewBuilder};

fn main() -> wry::Result<()> {
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    {
        use gtk::prelude::DisplayExtManual;

        gtk::init().unwrap();
        if gtk::gdk::Display::default().unwrap().backend().is_wayland() {
            panic!("This example doesn't support wayland!");
        }

        // we need to ignore this error here otherwise it will be catched by winit and will be
        // make the example crash
        winit::platform::x11::register_xlib_error_hook(Box::new(|_display, error| {
            let error = error as *mut x11_dl::xlib::XErrorEvent;
            (unsafe { (*error).error_code }) == 170
        }));
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(800, 800))
        .build(&event_loop)
        .unwrap();

    let webview = WebViewBuilder::new_as_child(&window)
        .with_url("https://tauri.app")
        .build()?;

    event_loop
        .run(move |event, evl| {
            evl.set_control_flow(ControlFlow::Poll);

            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let size = size.to_logical::<u32>(window.scale_factor());
                    webview
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, 0).into(),
                            size: LogicalSize::new(size.width, size.height).into(),
                        })
                        .unwrap();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => evl.exit(),
                _ => {}
            }
        })
        .unwrap();

    Ok(())
}
