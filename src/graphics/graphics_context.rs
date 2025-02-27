use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use winit::window::Window;
use clay_layout::Clay;

// use crate::graphics::pipeline_builder::PipelineBuilder;
// use crate::graphics::mesh_builder::{self, as_u8_slice, Vertex};

use crate::ui::ui_renderer::UIState;
use crate::ui::ui_layout::{self, create_layout, ClayState};

use super::depth_texture::DepthTexture;

pub struct GraphicsContext<'a>{
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    depth_texture: DepthTexture,
    size: (i32,i32),
    pub window: Window,
    // render_pipeline: wgpu::RenderPipeline,
    // triangle_mesh: wgpu::Buffer,
    // quad_mesh: wgpu::Buffer,

    pub ui_state: Rc<RefCell<UIState>>,
    pub clay: Clay<'a>,
    pub clay_user_data: ui_layout::ClayState,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(window: Window) -> Self {
        let size = (window.inner_size().width as i32, window.inner_size().height as i32);

        let instance = wgpu::Instance::default();

        let surface = unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap())
        }.unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default()
            },
            None,
        )).unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities.formats.iter()
            .copied().filter(|f| f.is_srgb())
            .next().unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let depth_texture = DepthTexture::new(&device, &config);

        

        // let triangle_mesh = mesh_builder::make_triangle(&device);
        // let quad_mesh = mesh_builder::make_quad(&device);

        // let mut pipeline_builder = PipelineBuilder::new("shader.wgsl", config.format);
        // pipeline_builder.add_buffer_layout(mesh_builder::Vertex::get_layout());
        // let render_pipeline = pipeline_builder.build_pipeline(&device);

        let ui_state = Rc::<RefCell<UIState>>::new(RefCell::new(UIState::new(&device, &queue,config.format, window.inner_size())));

        let mut clay = Clay::new((size.0 as f32, size.1 as f32).into());
        clay.enable_debug_mode(false);
        
        clay.set_measure_text_function_user_data(ui_state.clone(), ui_layout::measure_text);
        let mut clay_user_data = ClayState::default();
        ui_layout::initialize_user_data(&mut clay_user_data);

        Self {
            instance,
            window,
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
            // render_pipeline,
            // triangle_mesh,
            // quad_mesh, 
            
            ui_state,
            clay,
            clay_user_data,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;

        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label:Some("Render Encoder"),
        });

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &drawable.texture.create_view(&wgpu::TextureViewDescriptor::default()),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("RenderPass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None
            });

            //let timestamp = Instant::now();
            let render_commands = create_layout(
                &mut self.clay, 
                &mut self.clay_user_data,
                0.016,
            );
            self.ui_state.borrow_mut().render_clay(render_commands, &mut render_pass, &self.device, &self.queue, &self.config);
            //println!("{:?}", timestamp.elapsed());
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        drawable.present();
        Ok(())
    }

    pub fn resize(&mut self) {
        let new_size = (self.window.inner_size().width as i32, self.window.inner_size().height as i32);

        self.ui_state.borrow_mut().resize(new_size);

        self.clay_user_data.size = (new_size.0 as f32, new_size.1 as f32);

        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
        }

        self.depth_texture = DepthTexture::new(&self.device, &self.config);
    }

    pub fn _update_surface(&mut self){
        let target = unsafe {
            wgpu::SurfaceTargetUnsafe::from_window(&self.window)
        }.unwrap();
        self.surface = unsafe {
            self.instance.create_surface_unsafe(target)
        }.unwrap();
    }
}