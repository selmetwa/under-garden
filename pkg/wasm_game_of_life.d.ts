/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Species {
  Empty,
  Wall,
  Sand,
  Water,
  Mud,
  Dynamite,
  Steam,
  Smoke,
  Worm,
  Seed,
  Plant,
  Stone,
}
/**
*/
export class Cell {
  free(): void;
}
/**
*/
export class Universe {
  free(): void;
/**
*/
  tick(): void;
/**
*/
  reset(): void;
/**
* @param {number} width
* @param {number} height
* @returns {Universe}
*/
  static new(width: number, height: number): Universe;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  height(): number;
/**
* @returns {number}
*/
  cells(): number;
/**
* @param {number} x
* @param {number} y
* @param {number} size
* @param {number} species
*/
  paint(x: number, y: number, size: number, species: number): void;
}
