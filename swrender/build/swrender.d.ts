/* tslint:disable */
/* eslint-disable */
export class SoftwareRenderer {
  free(): void;
/**
* @returns {SoftwareRenderer} 
*/
  static new(): SoftwareRenderer;
/**
* Update the given CPU-side drawable\'s attributes given its ID.
* Will create a new drawable on the CPU side if one doesn\'t yet exist.
* @param {number} id 
* @param {Float32Array | undefined} matrix 
* @param {number | undefined} silhouette 
* @param {any | undefined} effects 
* @param {number} effect_bits 
* @param {boolean} use_nearest_neighbor 
*/
  set_drawable(id: number, matrix: Float32Array | undefined, silhouette: number | undefined, effects: any | undefined, effect_bits: number, use_nearest_neighbor: boolean): void;
/**
* Delete the CPU-side drawable with the given ID.
* @param {number} id 
*/
  remove_drawable(id: number): void;
/**
* Update the given silhouette\'s attributes and data given the corresponding skin\'s ID.
* Will create a new silhouette if one does not exist.
* @param {number} id 
* @param {number} w 
* @param {number} h 
* @param {Uint8Array} data 
* @param {number} nominal_width 
* @param {number} nominal_height 
* @param {boolean} premultiplied 
*/
  set_silhouette(id: number, w: number, h: number, data: Uint8Array, nominal_width: number, nominal_height: number, premultiplied: boolean): void;
/**
* Delete the silhouette that corresponds to the skin with the given ID.
* @param {number} id 
*/
  remove_silhouette(id: number): void;
/**
* Check if a particular Drawable is touching any in a set of Drawables.
* Will only check inside the given bounds.
* @param {number} drawable 
* @param {Int32Array} candidates 
* @param {any} rect 
* @returns {boolean} 
*/
  is_touching_drawables(drawable: number, candidates: Int32Array, rect: any): boolean;
/**
* Check if a certain color in a drawable is touching a particular color.
* @param {number} drawable 
* @param {Int32Array} candidates 
* @param {any} rect 
* @param {Uint8Array} color 
* @param {Uint8Array} mask 
* @returns {boolean} 
*/
  color_is_touching_color(drawable: number, candidates: Int32Array, rect: any, color: Uint8Array, mask: Uint8Array): boolean;
/**
* Check if a certain drawable is touching a particular color.
* @param {number} drawable 
* @param {Int32Array} candidates 
* @param {any} rect 
* @param {Uint8Array} color 
* @returns {boolean} 
*/
  is_touching_color(drawable: number, candidates: Int32Array, rect: any, color: Uint8Array): boolean;
/**
* Check if the drawable with the given ID is touching any pixel in the given rectangle.
* @param {number} drawable 
* @param {any} rect 
* @returns {boolean} 
*/
  drawable_touching_rect(drawable: number, rect: any): boolean;
/**
* Return the ID of the drawable that covers the most pixels in the given rectangle.
* Drawables earlier in the list will occlude those lower in the list.
* @param {Int32Array} candidates 
* @param {any} rect 
* @returns {number} 
*/
  pick(candidates: Int32Array, rect: any): number;
/**
* Calculate the convex hull points for the drawable with the given ID.
* @param {number} drawable 
* @returns {Float32Array} 
*/
  drawable_convex_hull_points(drawable: number): Float32Array;
}
