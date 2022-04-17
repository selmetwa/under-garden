import { Universe, Cell, Species } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import r from 'regl';
let fsh = require('./glsl/cell.glsl');
let vsh = require('./glsl/cellVertex.glsl');

let startWebGL = ({ canvas, universe }) => {
  const regl = r({
    canvas,
  });

  const width = universe.width();
  const height = universe.height();

  let cell_pointer = universe.cells();
  let cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4);
  const dataTexture = regl.texture({ width, height, data: cells });

  let drawSand = regl({
    frag: fsh,
    uniforms: {
      t: ({ tick }) => tick,
      data: () => {
        cell_pointer = universe.cells();
        cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4);
        return dataTexture({ width, height, data: cells });
      },
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
      dpi: window.devicePixelRatio * 2,
    },

    vert: vsh,
    attributes: {
      position: [
        [-1, 4],
        [-1, -1],
        [4, -1],
      ],
    },
    count: 3,
  });

  return () => {
    regl.poll();
    drawSand();
  };
};

export { startWebGL };