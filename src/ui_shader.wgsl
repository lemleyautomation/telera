struct SizeUniform{
    x: f32,
    y: f32,
};

struct Vertex {
    @location(0)position: vec3<f32>,
    @location(1)color: vec3<f32>,
};

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@group(1) @binding(0)
var<uniform> size: SizeUniform;

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
    var out: VertexPayload;
    out.position = vec4<f32>(
        (vertex.position.x/(size.x/2.0))-1,
        -((vertex.position.y/(size.y/2.0))-1),
        vertex.position.z, 
        1.0
    );
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in:VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}