#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> shape_id: i32;
@group(2) @binding(1) var<uniform> colour: vec4f;
@group(2) @binding(2) var<uniform> stroke_colour: vec4f;
@group(2) @binding(3) var<uniform> stroke_width: f32;
@group(2) @binding(4) var <storage, read> buffer: Data;

struct Data {
    values: array<f32>
}


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let r = buffer.values[0];
    let g = buffer.values[1];
    let b = buffer.values[2];
    let a = buffer.values[3];
    return vec4f(r, g, b, a);

}

//@fragment
//fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
//    //var d = min(sdf_hexagram(mesh.uv - vec2f(0.5), 0.2), sdf_circle(mesh.uv - vec2f(0.5), 0.3)) - 0.01;
//    //d = sdf_circle(mesh.uv - vec2f(0.5), 0.5);
//    //d = sdf_hexagram(mesh.uv - vec2f(0.5), 0.2);
//    //var d = sdf_circle(mesh.uv - vec2f(0.5), 0.5);
//
//
//    var d = sdf_from_shape_id(mesh, shape_id);
//    
//    d += stroke_width;
//
//    let ddist = vec2(dpdx(d), dpdy(d));
//    let pixel_dist = d / length(ddist);
//    let a = saturate(0.5 - pixel_dist);
//
//    let stroke_dist = abs(d) - stroke_width;
//    let stroke_ddist = vec2(dpdx(stroke_dist), dpdy(stroke_dist));
//    let stroke_pixel_dist = stroke_dist / length(stroke_ddist);
//    let stroke_alpha = saturate(0.5 - stroke_pixel_dist);
//    
//    var blended_colour = mix(stroke_colour, colour, a);
//    blended_colour.a = max(a, stroke_alpha);
//    if d > stroke_width {
//        //return vec4f(1.0);
//    }
//    return blended_colour;
//}

fn sdf_from_shape_id(mesh: VertexOutput, shape_id: i32) -> f32 {
    if shape_id == 0 {
        let round_const = 0.1;
        return sdf_triangle_isosceles(mesh.uv - vec2f(0.5, round_const), vec2f(0.5 - round_const, 1.0 - (round_const * 2.0))) - round_const;
    }
    else if shape_id == 1 {
        let round_const = 0.05;
        return min(sdf_hexagram(mesh.uv - vec2f(0.5), 0.27 - round_const), sdf_circle(mesh.uv - vec2f(0.5), 0.34 - round_const)) - round_const;
    }
    else if shape_id == 2 {
        return sdf_box(mesh.uv - vec2f(0.5), vec2f(0.5));
    }
    else {
        return 0.0;
    }
}

fn sdf_circle(p: vec2f, r: f32) -> f32
{
    return length(p) - r;
}

fn sdf_hexagram(p: vec2f, r: f32) -> f32
{
    let k: vec4f = vec4f(-0.5,0.8660254038,0.5773502692,1.7320508076);
    var p2 = abs(p);
    p2 -= 2.0*min(dot(k.xy,p2),0.0)*k.xy;
    p2 -= 2.0*min(dot(k.yx,p2),0.0)*k.yx;
    p2 -= vec2(clamp(p2.x,r*k.z,r*k.w),r);
    return length(p2)*sign(p2.y);
}

fn sdf_triangle_isosceles(p: vec2f, q: vec2f) -> f32
{
    var p2 = p;
    p2.x = abs(p2.x);
    let a: vec2f = p2 - q*clamp( dot(p2,q)/dot(q,q), 0.0, 1.0 );
    let b: vec2f = p2 - q*vec2( clamp( p2.x/q.x, 0.0, 1.0 ), 1.0 );
    let s: f32 = -sign( q.y );
    let d: vec2f = min( vec2( dot(a,a), s*(p2.x*q.y-p2.y*q.x) ),
                  vec2( dot(b,b), s*(p2.y-q.y)  ));
    return -sqrt(d.x)*sign(d.y);
}

fn sdf_box(p: vec2f, b: vec2f) -> f32
{
    var d = abs(p)-b;
    return length(max(d,vec2f(0.0))) + min(max(d.x,d.y),0.0);
}






