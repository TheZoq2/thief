#version 140

in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D diffuse_texture;
uniform sampler2D emissive_texture;
uniform vec2 resolution;
void main() {
    vec4 emissive_color = vec4(0., 0., 0., 0.);

    float blur_amount = 3;
    for(float x = -blur_amount; x <= blur_amount; x++)
    {
        for(float y = -blur_amount; y <= blur_amount; y++)
        {
            vec2 coords = v_tex_coords + vec2(x / resolution.x, y / resolution.y);
            emissive_color += texture(emissive_texture, coords);
        }
    }
    emissive_color = emissive_color / (blur_amount * blur_amount * 2 * 2);

    //emissive_color = texture(emissive_texture, v_tex_coords);

    vec4 diffuse_color = texture(diffuse_texture, v_tex_coords);
    //vec4 diffuse_color = vec4(0., 0., 0., 0.);
    //emissive_color = vec4(0., 0., 0., 0.);
    color = diffuse_color + emissive_color;
}
