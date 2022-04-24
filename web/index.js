// import { Universe, Cell, Species } from "wasm-game-of-life";
import { Universe, Cell, Species } from '../pkg/wasm_game_of_life';
import { startWebGL } from "./render";
import {
  eventDistance,
  magnitude,
  norm,
  scale,
  add,
  sub
} from './paintUtils';

const canvas = document.getElementById('game-of-life-canvas');

let n = 400;
const universe = Universe.new(n, n);
let size = 10;

canvas.height = n * Math.ceil(window.devicePixelRatio);
canvas.width = n * Math.ceil(window.devicePixelRatio);

let lastWorm = 0;
let isPaused = false;

const handleReset = () => {
  universe.reset();
}

function canCreateWorm() {
  let d = Date.now() - lastWorm;
  if (d > 1000) {
    lastWorm = Date.now();
    return true;
  } else {
    return false;
  }
}

let painting = false;
let lastPaint = null;
let repeat = null;

canvas.addEventListener("mousedown", (event) => {
  event.preventDefault();
  painting = true;
  clearInterval(repeat);
  repeat = window.setInterval(() => paint(event), 100);
  paint(event);
  lastPaint = event;
});

document.body.addEventListener("mouseup", (event) => {
  clearInterval(repeat);
  if (painting) {
    event.preventDefault();
    lastPaint = null;
    painting = false;
  }
});

canvas.addEventListener("mousemove", (event) => {
  clearInterval(repeat);
  if (painting) {
    smoothPaint(event);
  }
});

canvas.addEventListener("mouseleave", (event) => {
  clearInterval(repeat);
  lastPaint = null;
});

canvas.addEventListener("touchstart", (event) => {
  if (event.cancelable) {
    event.preventDefault();
  }
  painting = true;
  lastPaint = event;
  handleTouches(event);
});

canvas.addEventListener("touchend", (event) => {
  if (event.cancelable) {
    event.preventDefault();
  }
  lastPaint = null;
  painting = false;
  clearInterval(repeat);
});

canvas.addEventListener("touchmove", (event) => {
  if (event.cancelable) {
    event.preventDefault();
  }
  clearInterval(repeat);
  handleTouches(event);
});

function smoothPaint(event) {
  clearInterval(repeat);
  repeat = window.setInterval(() => paint(event), 100);
  let startEvent = { clientX: event.clientX, clientY: event.clientY };
  if (!painting) {
    return;
  }
  let i = 0;
  let step = Math.max(size / 5, 1);
  if (lastPaint) {
    while (eventDistance(startEvent, lastPaint) > step) {
      let d = eventDistance(startEvent, lastPaint);
      lastPaint = add(
        lastPaint,
        scale(norm(sub(lastPaint, event)), -Math.min(step, d))
      );
      i++;
      if (i > 1000) {
        break;
      }
      paint(lastPaint);
    }
  }
  paint(startEvent);

  lastPaint = event;
}

const handleTouches = (event) => {
  let touches = Array.from(event.touches);
  if (touches.length == 1) {
    smoothPaint(touches[0]);
  } else {
    touches.forEach(paint);
  }
};

let species = Species.Empty;

const sizeButtons = Array.from(document.querySelectorAll('.size-button'));
const speciesButtons = Array.from(document.querySelectorAll('.species-button'));

const removeActiveFromSpecies = () => {
  speciesButtons.forEach(btn => btn.classList.remove('active'))
}

const handleUpdateSpeciesByKey = (key) => {
  // removeActiveFromSpecies();
  const chars = ['q', 'w', 'e', 'a', 's', 'd', 'f'];
  const nums = ['1', '2', '3', '4'];
  if (chars.includes(key)) {
    removeActiveFromSpecies();
  }

  if (nums.includes(key)) {
    removeActive();
  }
  switch (key) {
    case '1':
      document.querySelector('.five-button').classList.add('active');
      size = 5;
      break;
    case '2':
      document.querySelector('.ten-button').classList.add('active');
      size = 10;
      break;
    case '3':
      document.querySelector('.twenty-five-button').classList.add('active');
      size = 25;
      break;
    case '4':
      document.querySelector('.fifty-button').classList.add('active');
      size = 50;
      break;
    case 'q':
      document.querySelector(`.empty-button`).classList.add('active')
      species = Species['Empty'];
      break;
    case 'w':
      document.querySelector(`.sand-button`).classList.add('active')
      species = Species['Sand'];
      break;
    case 'e':
      document.querySelector(`.water-button`).classList.add('active')
      species = Species['Water'];
      break;
    case 'a':
      document.querySelector(`.dynamite-button`).classList.add('active')
      species = Species['Dynamite'];
      break;
    case 's':
      document.querySelector(`.worm-button`).classList.add('active')
      species = Species['Worm'];
      break;
    case 'd':
      document.querySelector(`.seed-button`).classList.add('active')
      species = Species['Seed'];
      break;
    case 'f':
      document.querySelector(`.stone-button`).classList.add('active')
      species = Species['Stone'];
      break;
    default:
      return
  }
}
document.addEventListener('keydown', (e) => {
  const key = e.key.toLowerCase().trim();
  handleUpdateSpeciesByKey(key);
});

speciesButtons.forEach(btn => {
  btn.addEventListener('click', (e) => {
    removeActiveFromSpecies();
    document.querySelector(`.${e.target.value.toLowerCase()}-button`).classList.add('active')
    species = Species[e.target.value]
  })
})

const removeActive = () => {
  sizeButtons.forEach(btn => btn.classList.remove('active'))
}


document.querySelector('.five-button').addEventListener('click', () => {
  removeActive();
  document.querySelector('.five-button').classList.add('active');
  size = 5;
});

document.querySelector('.ten-button').addEventListener('click', () => {
  removeActive();
  document.querySelector('.ten-button').classList.add('active');
  size = 10
});
document.querySelector('.twenty-five-button').addEventListener('click', () => {
  removeActive();
  document.querySelector('.twenty-five-button').classList.add('active');
  size = 25
});
document.querySelector('.fifty-button').addEventListener('click', () => {
  removeActive();
  document.querySelector('.fifty-button').classList.add('active');
  size = 50
});


const pauseButton = document.querySelector('.pause')
pauseButton.addEventListener('click', (e) => {
  if (isPaused) {
    isPaused = false;
    pauseButton.textContent = 'Pause';
    return
  }
  isPaused = true;
  pauseButton.textContent = 'Play';
})

document.querySelector('.reset').addEventListener('click', () => {
  handleReset()
})

document.querySelector('.about').addEventListener('click', () => {
  document.querySelector('.mask').classList.add('open')
  document.querySelector('.modal').classList.add('open')
})

document.querySelector('.close-modal').addEventListener('click', () => {
  document.querySelector('.mask').classList.remove('open')
  document.querySelector('.modal').classList.remove('open')
})
const paint = (event) => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX =
    canvas.width / Math.ceil(window.devicePixelRatio) / boundingRect.width;
  const scaleY =
    canvas.height / Math.ceil(window.devicePixelRatio) / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const x = Math.min(Math.floor(canvasLeft), 400 - 1);
  const y = Math.min(Math.floor(canvasTop), 400 - 1);

  if (species === Species.Worm) {
    size = 2;
    if (!canCreateWorm()) {
      return;
    }
  }

  if (species === Species.Seed) {
    size = 5;
    if (!canCreateWorm()) {
      return;
    }
  }

  universe.paint(
    x,
    y,
    size,
    species
  );
};

let draw = startWebGL({ canvas, universe });
const renderLoop = () => {
  if (!isPaused) {
    universe.tick();
  }
  draw();
  requestAnimationFrame(renderLoop);

}

renderLoop();