struct CustomMaterial {
    time: f32,
};

struct Camera {
  pos: vec3<f32>,
}

let TAU = 6.28318530717;

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var<uniform> camera: Camera;

@fragment
fn fragment(
    @builtin(position) coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) uv: vec2<f32>
    ) -> @location(0) vec4<f32> {
    var input1: vec2<f32> = vec2<f32>(world_position.x  , material.time);
    var input2: vec2<f32> = vec2<f32>(world_position.y  , material.time);
    var input3: vec2<f32> = vec2<f32>(world_position.z  , material.time);


    var offset = cos(uv.y * TAU * 4.0) * 0.1;
    var t = tan( (uv.x + offset - material.time * 0.2) * TAU * 4.0) * 0.5 + 0.5;

    /* t *= 1.0 - uv.y; */

      return vec4<f32>(t);

}
