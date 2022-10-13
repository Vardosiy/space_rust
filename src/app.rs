use std::rc::Rc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::dpi::PhysicalSize;

use crate::game_level::GameLevel;
use crate::globals::set_screen_size;
use crate::input_mgr::InputMgr;
use crate::math::Vec2i;

pub struct App;

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn run(&self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let input_mgr = Rc::new(InputMgr {});
        let mut game = GameLevel::new(Rc::clone(&input_mgr));

        window.set_inner_size(PhysicalSize { width: 600, height: 600 });
        set_screen_size(Vec2i { x: 200, y: 200 });

        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            control_flow.set_poll();
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Exit requested");
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                    window.request_redraw();  // temporary, don't really understand why it is needed here
                }
                Event::RedrawRequested(_) => {
                    game.update();
                    game.render();
                }
                _ => ()
            }
        });
    }
}
