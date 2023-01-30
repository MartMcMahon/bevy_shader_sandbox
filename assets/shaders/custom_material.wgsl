struct CustomMaterial {
    time: f32,
};

struct Camera {
  pos: vec3<f32>,
}

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

    var value1 = cos(world_position.x);
    var value2 = cos(world_position.y + material.time);
    var value3 = sin(material.time);

    if (normals.z == 1.0) {
        return vec4<f32>(value1 - 0.1,value2 - 0.1,value3 - 0.1,1.0);
    } else if (normals.y == 1.0) {
        return vec4<f32>(value1 - 0.2,value2 - 0.2,value3 - 0.2,1.0);
    } else {
        return vec4<f32>(value1,value2,value3,1.0);
    }
}
