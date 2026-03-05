struct VertexInput {
    float3 position;
    float4 color;
    float3 normal;
    float2 texture_coordinates;

    /// Default constructor
    __init(float3 position, float4 color, float3 normal = float3(0.), float2 texture_coordinates = float2(0.)) {
        this.position = position;
        this.color = color;
        this.normal = normal;
        this.texture_coordinates = texture_coordinates;
    }
};

struct VertexOutput {
    float4 color;
    float4 sv_position : SV_Position;
};

// TODO: remove this temporary code
static VertexInput input_triangles[3] = VertexInput[](
    // Top triangle
    VertexInput(
        float3(float2(0., -0.5), 0.),
        float4(1., 0., 0., 1.),
    ),
    // Right triangle
    VertexInput(
        float3(float2(0.5, 0.5), 0.),
        float4(0., 1., 0., 1.),
    ),
    // Left triangle
    VertexInput(
        float3(float2(-0.5, 0.5), 0.),
        float4(0., 0., 1., 1.),
    ),
);

[shader("vertex")]
VertexOutput vert_main(uint vid: SV_VertexID) {
    VertexInput in_vert = input_triangles[vid];
    VertexOutput output;
    output.sv_position = float4(in_vert.position, 1.0);
    output.color = in_vert.color;
    return output;
}

[shader("fragment")]
float4 frag_main(VertexOutput out_vert) : SV_Target {
    return out_vert.color;
}
