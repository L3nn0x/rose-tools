shader_type spatial;
render_mode unshaded;

uniform sampler2D layer1;
uniform sampler2D layer2;

void vertex() {
}

void fragment() {
    vec4 tex1 = texture(layer1, UV); // Base texture
    vec4 tex2 = texture(layer2, UV2);
    ALBEDO = tex1.rgb;
}
