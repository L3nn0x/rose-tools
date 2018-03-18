shader_type spatial;
render_mode unshaded;

uniform sampler2D layer1; // Bottom Texture
uniform sampler2D layer2; // Top texture

void vertex() {
}

void fragment() {
    vec4 tex1 = texture(layer1, UV); 
    vec4 tex2 = texture(layer2, UV2);
    ALBEDO = mix(tex1, tex2, tex2.a).rgb;
}
