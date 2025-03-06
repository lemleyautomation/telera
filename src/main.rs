//#![windows_subsystem = "windows"]

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, ActiveEventLoop};
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use winit::platform::windows::WindowAttributesExtWindows;
use winit::window::{Icon, Window, WindowId};

use std::cell::RefCell;
use std::rc::Rc;

use clay_layout::Clay;

use graphics_context::GraphicsContext;
use ui_renderer::{UIBorderThickness, UIColor, UICornerRadii, UIPosition, UIState};
mod graphics_context;
mod ui_renderer;
mod ui_layout;

#[rustfmt::skip]
fn main() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(_) => return
    };
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(&mut App::default()).unwrap();
}

#[derive(Default)]
pub struct App<'a> {
    ctx: Option<GraphicsContext<'a>>,

    pub ui_state: Option<Rc<RefCell<UIState>>>,
    pub clay: Option<Clay>,
    pub clay_user_data: ui_layout::ClayState,

    pub lcon: bool,
    pub rcon: bool,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Telera".to_string())
            .with_inner_size(LogicalSize::new(800, 600))
            //.with_window_icon(load_icon())
            .with_taskbar_icon(load_icon());

        let window = event_loop.create_window(window_attributes).unwrap();
        let size = window.inner_size();
        let dpi_scale = window.scale_factor() as f32;

        let ctx = GraphicsContext::new(window);

        let ui_state = Rc::<RefCell<UIState>>::new(RefCell::new(UIState::new(&ctx.device, &ctx.queue,ctx.config.format, size, dpi_scale)));

        let mut clay = Clay::new((size.width as f32, size.height as f32).into());
        clay.enable_debug_mode(false);
        
        clay.set_measure_text_function_user_data(ui_state.clone(), ui_layout::measure_text);
        
        ui_layout::initialize_user_data(&mut self.clay_user_data);

        self.ctx = Some(ctx);
        self.ui_state = Some(ui_state);
        self.clay = Some(clay);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                self.ctx.as_mut().unwrap().resize();
                
                self.ui_state.as_mut().unwrap().borrow_mut().resize((size.width as i32, size.height as i32), &self.ctx.as_ref().unwrap().queue);
                self.clay_user_data.size = (size.width as f32, size.height as f32);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer:_ } => {
                self.ui_state.as_mut().unwrap().borrow_mut().dpi_scale = scale_factor as f32;
            }
            WindowEvent::RedrawRequested => {
                let render_commands = ui_layout::create_layout(self.clay.as_mut().unwrap(), &mut self.clay_user_data, 0.016);
                let mut ui_renderer = self.ui_state.as_mut().unwrap().borrow_mut();

                self.ctx.as_mut().unwrap().render(
                    |mut render_pass, device, queue, config| {
                        ui_renderer.filled_rectangle(
                            UIPosition{ x:200.0, y:200.0, z:0.0 },
                            UIPosition { x: 200.0, y: 200.0, z: 0.0 }, 
                            UIColor{r:0.5,g:0.8,b:0.5}, 
                            UICornerRadii{top_left:0.0, top_right:0.0,bottom_left:0.0, bottom_right:0.0}
                        );
                        ui_renderer.render(&mut render_pass, &queue);

                        //ui_renderer.render_clay(render_commands, &mut render_pass, &device, &queue, &config);
                    }
                ).unwrap();
                self.clay_user_data.mouse_down_rising_edge = false;
            }
            WindowEvent::MouseInput { device_id:_, state, button } => {
                match button {
                    winit::event::MouseButton::Left => {
                        self.clay_user_data.mouse_down_rising_edge = state.is_pressed();
                    }
                    _ => {}
                }
                self.ctx.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseWheel { device_id:_, delta, phase:_ } => {
                self.clay_user_data.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(x,y ) => (x,y),
                    MouseScrollDelta::PixelDelta(position) => position.into()
                };
                self.ctx.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::CursorMoved { device_id:_, position } => {
                self.clay_user_data.mouse_position = position.into();
                self.ctx.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::KeyboardInput { device_id:_, event, is_synthetic:_ } => {
                match event.physical_key {
                    PhysicalKey::Code(c)  => {
                        match c {
                            KeyCode::ControlLeft => {
                                match event.state {
                                    ElementState::Pressed => self.lcon = true,
                                    ElementState::Released => self.lcon = false
                                }
                            }
                            KeyCode::ControlRight => {
                                match event.state {
                                    ElementState::Pressed => self.rcon = true,
                                    ElementState::Released => self.rcon = false
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                match event.logical_key {
                    Key::Character(char) => {
                        if (self.lcon || self.rcon) && char.contains("+") {
                            self.clay_user_data.user_scale += 1;    
                        }
                        if (self.lcon || self.rcon) && char.contains("-") &&  self.clay_user_data.user_scale > 0 {
                            self.clay_user_data.user_scale -= 1;    
                        }
                    }
                    _ => {}
                }
                self.ctx.as_ref().unwrap().window.request_redraw();
            }
            _ => (),
        }
    }
}

fn load_icon() -> Option<Icon>{
    let img = match image::load_from_memory(include_bytes!("telera.png")) {
        Ok(img) => img,
        Err(_) => return None
    };

    let (icon_rgba, icon_width, icon_height) = {
        let image = img.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    
    match Icon::from_rgba(icon_rgba, icon_width, icon_height) {
        Ok(icon) => {Some(icon)}
        Err(_) => None
    }
}