import * as wasm from './swrender_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getFloat32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}
/**
*/
export class SoftwareRenderer {

    static __wrap(ptr) {
        const obj = Object.create(SoftwareRenderer.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_softwarerenderer_free(ptr);
    }
    /**
    * @returns {SoftwareRenderer}
    */
    static new() {
        var ret = wasm.softwarerenderer_new();
        return SoftwareRenderer.__wrap(ret);
    }
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
    set_drawable(id, matrix, silhouette, effects, effect_bits, use_nearest_neighbor) {
        var ptr0 = isLikeNone(matrix) ? 0 : passArrayF32ToWasm0(matrix, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.softwarerenderer_set_drawable(this.ptr, id, ptr0, len0, !isLikeNone(silhouette), isLikeNone(silhouette) ? 0 : silhouette, isLikeNone(effects) ? 0 : addHeapObject(effects), effect_bits, use_nearest_neighbor);
    }
    /**
    * Delete the CPU-side drawable with the given ID.
    * @param {number} id
    */
    remove_drawable(id) {
        wasm.softwarerenderer_remove_drawable(this.ptr, id);
    }
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
    set_silhouette(id, w, h, data, nominal_width, nominal_height, premultiplied) {
        var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.softwarerenderer_set_silhouette(this.ptr, id, w, h, ptr0, len0, nominal_width, nominal_height, premultiplied);
    }
    /**
    * Delete the silhouette that corresponds to the skin with the given ID.
    * @param {number} id
    */
    remove_silhouette(id) {
        wasm.softwarerenderer_remove_silhouette(this.ptr, id);
    }
    /**
    * Check if a particular Drawable is touching any in a set of Drawables.
    * Will only check inside the given bounds.
    * @param {number} drawable
    * @param {Int32Array} candidates
    * @param {any} rect
    * @returns {boolean}
    */
    is_touching_drawables(drawable, candidates, rect) {
        var ptr0 = passArray32ToWasm0(candidates, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.softwarerenderer_is_touching_drawables(this.ptr, drawable, ptr0, len0, addHeapObject(rect));
        return ret !== 0;
    }
    /**
    * Check if a certain color in a drawable is touching a particular color.
    * @param {number} drawable
    * @param {Int32Array} candidates
    * @param {any} rect
    * @param {Uint8Array} color
    * @param {Uint8Array} mask
    * @returns {boolean}
    */
    color_is_touching_color(drawable, candidates, rect, color, mask) {
        var ptr0 = passArray32ToWasm0(candidates, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(color, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = passArray8ToWasm0(mask, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ret = wasm.softwarerenderer_color_is_touching_color(this.ptr, drawable, ptr0, len0, addHeapObject(rect), ptr1, len1, ptr2, len2);
        return ret !== 0;
    }
    /**
    * Check if a certain drawable is touching a particular color.
    * @param {number} drawable
    * @param {Int32Array} candidates
    * @param {any} rect
    * @param {Uint8Array} color
    * @returns {boolean}
    */
    is_touching_color(drawable, candidates, rect, color) {
        var ptr0 = passArray32ToWasm0(candidates, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(color, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.softwarerenderer_is_touching_color(this.ptr, drawable, ptr0, len0, addHeapObject(rect), ptr1, len1);
        return ret !== 0;
    }
    /**
    * Check if the drawable with the given ID is touching any pixel in the given rectangle.
    * @param {number} drawable
    * @param {any} rect
    * @returns {boolean}
    */
    drawable_touching_rect(drawable, rect) {
        var ret = wasm.softwarerenderer_drawable_touching_rect(this.ptr, drawable, addHeapObject(rect));
        return ret !== 0;
    }
    /**
    * Return the ID of the drawable that covers the most pixels in the given rectangle.
    * Drawables earlier in the list will occlude those lower in the list.
    * @param {Int32Array} candidates
    * @param {any} rect
    * @returns {number}
    */
    pick(candidates, rect) {
        var ptr0 = passArray32ToWasm0(candidates, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.softwarerenderer_pick(this.ptr, ptr0, len0, addHeapObject(rect));
        return ret;
    }
    /**
    * Calculate the convex hull points for the drawable with the given ID.
    * @param {number} drawable
    * @returns {Float32Array}
    */
    drawable_convex_hull_points(drawable) {
        wasm.softwarerenderer_drawable_convex_hull_points(8, this.ptr, drawable);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getInt32Memory0()[8 / 4 + 1];
        var v0 = getArrayF32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4);
        return v0;
    }
}

export const __wbg_left_e0e87a2e66be13a6 = function(arg0) {
    var ret = getObject(arg0).left;
    return ret;
};

export const __wbg_right_7b7bac033ade0b86 = function(arg0) {
    var ret = getObject(arg0).right;
    return ret;
};

export const __wbg_bottom_4666a55ceceeee8a = function(arg0) {
    var ret = getObject(arg0).bottom;
    return ret;
};

export const __wbg_top_84c6cfb6e6a6bd02 = function(arg0) {
    var ret = getObject(arg0).top;
    return ret;
};

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbg_ucolor_ec62c5e559a2a5a3 = function(arg0) {
    var ret = getObject(arg0).u_color;
    return ret;
};

export const __wbg_ufisheye_6aa56ae214de6428 = function(arg0) {
    var ret = getObject(arg0).u_fisheye;
    return ret;
};

export const __wbg_uwhirl_677f66c116ae8d9b = function(arg0) {
    var ret = getObject(arg0).u_whirl;
    return ret;
};

export const __wbg_upixelate_eb81083d476dfa89 = function(arg0) {
    var ret = getObject(arg0).u_pixelate;
    return ret;
};

export const __wbg_umosaic_7bc9d9ddd07459c3 = function(arg0) {
    var ret = getObject(arg0).u_mosaic;
    return ret;
};

export const __wbg_ubrightness_d29d8f78f9c8e71d = function(arg0) {
    var ret = getObject(arg0).u_brightness;
    return ret;
};

export const __wbg_ughost_d81ebfbc362e40b0 = function(arg0) {
    var ret = getObject(arg0).u_ghost;
    return ret;
};

export const __wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return addHeapObject(ret);
};

export const __wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

