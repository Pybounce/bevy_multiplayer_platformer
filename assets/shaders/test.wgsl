#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let d = min(sdf_hexagram(mesh.uv - vec2f(0.5), 0.2), sdf_circle(mesh.uv - vec2f(0.5), 0.3)) - 0.01;
    let stroke_width = 0.01;
    if d < -stroke_width {
        let t = smoothstep(0.001, -0.001, d + stroke_width);
        return vec4f(
            mix(1.0, 0.0, t), 
            mix(1.0, mesh.uv.x, t),
            mix(1.0, mesh.uv.y, t),
            1.0);
        }

    let a = smoothstep(0.001, -0.001, d);
    return vec4f(1.0, 1.0, 1.0, a);
}

fn sdf_circle(p: vec2f, r: f32) -> f32
{
    return length(p) - r;
}

fn sdf_hexagram(p: vec2f, r: f32 ) -> f32
{
    let k: vec4f = vec4f(-0.5,0.8660254038,0.5773502692,1.7320508076);
    var p2 = abs(p);
    p2 -= 2.0*min(dot(k.xy,p2),0.0)*k.xy;
    p2 -= 2.0*min(dot(k.yx,p2),0.0)*k.yx;
    p2 -= vec2(clamp(p2.x,r*k.z,r*k.w),r);
    return length(p2)*sign(p2.y);
}