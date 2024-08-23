#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> size: f32;
@group(2) @binding(1) var<uniform> colour: vec4f;
@group(2) @binding(2) var<uniform> stroke_colour: vec4f;
@group(2) @binding(3) var<uniform> stroke_width: f32;
@group(2) @binding(4) var <storage, read> buffer: Data;

struct Data {
    values: array<vec2f>
}


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    //let r = buffer.values[0];
    //let g = buffer.values[1];
    //let b = buffer.values[2];
    //let a = buffer.values[3];
    let p = mesh.uv - vec2f(0.5 / size);
    let tile_p = vec2f((floor(p.x * size) + 0.5) / size, (floor(p.y * size) + 0.5) / size);


    let new_p = tile_p;//(normalize(p - tile_p) * stroke_width);
    if sdf_all_tiles(new_p, mesh) < 0.0 {
        return vec4f(-sdf_all_tiles(new_p, mesh), -sdf_all_tiles(new_p, mesh), -sdf_all_tiles(new_p, mesh), 1.0);
    }
    
    var d = sdf_all_tiles(p, mesh);
    //sdf_box(p - buffer.values[0], vec2f(0.5 / size, 0.5 / size));
    //for (var i = 1; i < i32(arrayLength(&buffer.values)); i = i + 1) {
    //    d = min(d, sdf_box(mesh.uv - buffer.values[i] - vec2f(0.5 / size), vec2f(0.5 / size, 0.5 / size)));
    //    //d = smooth_union(d, sdf_box(p - buffer.values[i], vec2f(0.5 / size, 0.5 / size)), 0.01);
    //}
    
    if d < 0.0 {
        return vec4f(1.0, 0.0, 0.0, 1.0);
    }
    return vec4f(0.0, 1.0, 0.0, 1.0);

    //d += stroke_width;  
    //let ddist = vec2(dpdx(d), dpdy(d));
    //let pixel_dist = d / length(ddist);
    //let a = saturate(0.5 / size - pixel_dist);  
    //let stroke_dist = abs(d) - stroke_width;
    //let stroke_ddist = vec2(dpdx(stroke_dist), dpdy(stroke_dist));
    //let stroke_pixel_dist = stroke_dist / length(stroke_ddist);
    //let stroke_alpha = saturate(0.5 / size - stroke_pixel_dist);

    //var blended_colour = mix(stroke_colour, colour, a);
    //blended_colour.a = max(a, stroke_alpha);
    //if d > stroke_width {
    //    //return vec4f(0.0, 1.0, 0.0, 1.0);
    //}
    //

    ////return vec4f(-d, -d, -d, 1.0);
    //return blended_colour;
}

fn sdf_all_tiles(p: vec2f, mesh: VertexOutput) -> f32 {
    var d = sdf_box(p - buffer.values[0], vec2f(0.5 / size, 0.5 / size));
    for (var i = 1; i < i32(arrayLength(&buffer.values)); i = i + 1) {
        d = min(d, sdf_box(mesh.uv - buffer.values[i] - vec2f(0.5 / size), vec2f(0.5 / size, 0.5 / size)));
        //d = smooth_union(d, sdf_box(p - buffer.values[i], vec2f(0.5 / size, 0.5 / size)), 0.01);
    }
    return d;
}

fn smooth_union_old(d1: f32, d2: f32, k: f32) -> f32 {
    let h = max(k - abs(d1 - d2), 0.0);
    return min(d1, d2) - h * h * 0.25 / k;
}

fn sdf_box(p: vec2f, b: vec2f) -> f32
{
    var d = abs(p)-b;
    return length(max(d,vec2f(0.0))) + min(max(d.x,d.y),0.0);
}

fn smooth_union(d1: f32, d2: f32, k: f32) -> f32
{
    let h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h);
}