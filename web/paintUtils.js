const eventDistance = (a, b) => {
  return Math.sqrt(
    Math.pow(a.clientX - b.clientX, 2) + Math.pow(a.clientY - b.clientY, 2),
    2
  );
};

const magnitude = (a) => {
  return Math.sqrt(Math.pow(a.clientX, 2) + Math.pow(a.clientY, 2), 2);
};

const norm = (a) => {
  let mag = magnitude(a);
  return { clientX: a.clientX / mag, clientY: a.clientY / mag };
};
const scale = (a, s) => {
  return { clientX: a.clientX * s, clientY: a.clientY * s };
};
const add = (a, b) => {
  return { clientX: a.clientX + b.clientX, clientY: a.clientY + b.clientY };
};
const sub = (a, b) => {
  return { clientX: a.clientX - b.clientX, clientY: a.clientY - b.clientY };
};

export {
  eventDistance,
  magnitude,
  norm,
  scale,
  add,
  sub
};