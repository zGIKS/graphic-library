struct Globals {
    viewport_size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vertex_main(
    @location(0) unit_pos: vec2<f32>,          // (0..1) quad
    @location(1) rect_pos: vec2<f32>,          // pixels
    @location(2) rect_size: vec2<f32>,         // pixels
    @location(3) rect_color: vec4<f32>,
) -> VertexOut {
    var out: VertexOut;

    let pixel_pos = rect_pos + unit_pos * rect_size;
    let ndc_x = (pixel_pos.x / globals.viewport_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pixel_pos.y / globals.viewport_size.y) * 2.0;

    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.color = rect_color;
    return out;
}

@fragment
fn fragment_main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}

