#define DEPTH disabled
#define CULL disabled
#define SHAPE filled_triangles

#define VERTEX_LOCAL_POSITION

layout(location = 0) out vec4 out_color;

float PI = 3.14159265359;

vec2 curve_uv(vec2 uv, vec2 curvature) {
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs(uv.yx) / curvature;
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void fragment() {
    // remap the uvs
    vec2 uv = curve_uv(in_uv, material.b.xy);
    float black = material.c.x;
    vec2 size = material.d.xy;

    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
        // output black when out of bounds
        out_color = vec4(vec3(0.0), 1.0);
    } else {
        // calculate scanlines
        float x_scan = sin((uv.x + 0.005) * size.x * PI * 2.0) * 0.5 + 0.5;
        float y_scan = sin((uv.y + 0.005) * size.y * PI * 2.0) * 0.5 + 0.5;
        float scan = smoothstep(0.05, 0.3, x_scan * y_scan) * (1.0 - black) + black;

        // calculate vignette
        float x_vignette = clamp(1.0 - pow(in_uv.x * 2.0 - 1.0, 4.0), 0.0, 1.0);
        float y_vignette = clamp(1.0 - pow(in_uv.y * 2.0 - 1.0, 4.0), 0.0, 1.0);
        float vignette = x_vignette * y_vignette;

        out_color = tex(uint(material.a.x), uv) * vec4(vec3(scan * vignette), 1.0);
    }
}
