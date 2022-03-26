/* tslint:disable */
/* eslint-disable */
/**
* @returns {Promise<void>}
*/
export function init(): Promise<void>;
/**
* @param {Vao} vao
* @returns {number}
*/
export function get_width(vao: Vao): number;
/**
* @param {Vao} vao
* @returns {number}
*/
export function get_height(vao: Vao): number;
/**
* @param {string} image_name
* @returns {Promise<Vao>}
*/
export function create_quad(image_name: string): Promise<Vao>;
/**
* @param {number} width
* @param {number} height
* @returns {Vao}
*/
export function create_quad_line(width: number, height: number): Vao;
/**
*/
export function prepare(): void;
/**
* @param {Vao} vao
* @param {number} rotation
*/
export function draw_line(vao: Vao, rotation: number): void;
/**
*/
export function draw(): void;
/**
*/
export class Shader {
  free(): void;
}
/**
*/
export class Texture {
  free(): void;
/**
*/
  height: number;
/**
*/
  width: number;
}
/**
*/
export class Vao {
  free(): void;
}
