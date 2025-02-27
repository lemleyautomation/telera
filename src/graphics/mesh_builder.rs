use wgpu::util::DeviceExt;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Color{pub r: f32, pub g: f32, pub b:f32}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Position{pub x:f32, pub y:f32, pub z:f32}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
}

impl Vertex{
    pub fn from(x:f32,y:f32,z:f32,r:f32,g:f32,b:f32)->Self {
        Self {
            position: Position {x, y, z},
            color: Color {r, g, b}
        }
    }

    pub fn new() -> Self {
        Self {
            position: Position {x: 0.0, y: 0.0, z: 0.0},
            color: Color {r: 0.0, g: 0.0, b: 0.0}
        }
    }

    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {

        const ATTR: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout { array_stride: std::mem::size_of::<Vertex>() as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &ATTR }
    }
}

pub unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer{

    let vertices = [
        Vertex::from( 0.0,  0.5, 0.0, 1.0, 0.0, 0.0),
        Vertex::from(-0.5, -0.5, 0.0, 0.0, 1.0, 0.0),
        Vertex::from( 0.5, -0.5, 0.0, 0.0, 0.0, 1.0),
    ];

    let bytes: &[u8] = unsafe {as_u8_slice(&vertices)};

    let buffer_desctriptor = wgpu::util::BufferInitDescriptor {
        label: Some("hi"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX
    };

    let buffer = device.create_buffer_init(&buffer_desctriptor);
    
    buffer
}

pub fn make_quad(device: &wgpu::Device) -> wgpu::Buffer{

    let vertices = [
        Vertex::from( -0.5, 0.5,0.0, 1.0, 0.0, 0.0),
        Vertex::from(-0.5,-0.5,0.0, 0.0, 1.0, 0.0),
        Vertex::from( 0.5, 0.5,0.0, 0.0, 0.0, 1.0),

        Vertex::from( -0.5,-0.5,0.0, 1.0, 0.0, 0.0),
        Vertex::from( 0.5,-0.5,0.0, 0.0, 1.0, 0.0),
        Vertex::from( 0.5, 0.5,0.0, 0.0, 0.0, 1.0),
    ];

    let bytes: &[u8] = unsafe {as_u8_slice(&vertices)};

    let buffer_desctriptor = wgpu::util::BufferInitDescriptor {
        label: Some("some Quad vertex buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX
    };

    let buffer = device.create_buffer_init(&buffer_desctriptor);

    buffer
}

pub fn make_buffer(device: &wgpu::Device, label: &str, number_of_triangles: usize) -> (wgpu::Buffer, Vec<Vertex>) {
    
    let vertices: Vec<Vertex> = vec![Vertex::new();number_of_triangles*3];

    let buffer_desctriptor = wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
    };

    let buffer = device.create_buffer_init(&buffer_desctriptor);
    
    (buffer, vertices)
}
