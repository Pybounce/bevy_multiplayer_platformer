#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Data {
    values: array<f32>
}

@group(0) @binding(0)
var <storage, read> data: Data;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    let r = values[0];
    let g = values[1];
    let b = values[2];
    let a = values[3];
    return vec4f(r, g, b, a);
}


fn sdf_box(p: vec2f, b: vec2f) -> f32
{
    var d = abs(p)-b;
    return length(max(d,vec2f(0.0))) + min(max(d.x,d.y),0.0);
}






