precision highp float;
uniform float t;
uniform float dpi;
uniform vec2 resolution;
uniform sampler2D data;

varying vec2 uv;

// clang-format off
#pragma glslify: hsv2rgb = require('glsl-hsv2rgb')
#pragma glslify: snoise3 = require(glsl-noise/simplex/3d)
#pragma glslify: snoise2 = require(glsl-noise/simplex/2d)
#pragma glslify: random = require(glsl-random)

// clang-format on

void main() {
  vec3 color;
  vec2 textCoord = ((uv * vec2(0.5, -0.5)) + vec2(0.5)).yx;

  vec4 data = texture2D(data, textCoord);
  int type = int((data.r * 255.) + 0.1);
  float hue = 0.0;
  float saturation = 0.6;
  float lightness = 0.3 + data.g * 0.5;
  float noise = snoise3(vec3(floor(uv * resolution / dpi), t * 0.05));
  float a = 1.0;

  // empty
  if (type == 0) {
    hue = 0.1;
    saturation = 0.3;
    lightness = 0.1;
  }
  // wall 
  if (type == 1) {
    hue = 0.15;
    saturation = 0.22;
    lightness = 0.18;
  } 
  // sand
  if (type == 2) {
    hue = 0.1;
    saturation = 0.5;
    lightness -= 0.1;
  } 
  // water 
  if (type == 3) {
    hue = 0.6;
    saturation = 0.7;
    lightness += 0.5;
  }
  // dynamite
  if (type == 5) {
    hue = 0.0;
    saturation = 0.66;
    lightness += 0.61;
  }
  // steam
  if (type == 6) {
    hue = 0.6;
    saturation = 0.1;
    lightness += 0.9;
  }
  // smoke
  if (type == 7) {
    hue = 0.0;
    saturation = 0.1;
    lightness = 0.3;
  }
  // worm
  if (type == 8) {
    hue = 0.0;
    saturation = .5;
    lightness = 0.81;
  }
  // seed
  if (type == 9) {
    hue = 0.51;
    saturation = 0.25;
    lightness = 0.4;
  }
    // plant
  if (type == 10) {
    hue = 0.51;
    saturation = 0.65;
    lightness += 0.3 + data.g * 0.3;
  }
  // stone
  if (type == 11) {
    hue = 0.0;
    saturation = 0.1;
    lightness += 0.3;
    // hue = 0.51;
    // saturation = 0.65;
    // lightness += 0.3 + data.g * 0.3;
  }
  if (type == 12) {
    hue = 0.0;
    saturation = 0.66;
    lightness += 0.61;
  }
  color = hsv2rgb(vec3(hue, saturation, lightness));
  gl_FragColor = vec4(color, a);
}