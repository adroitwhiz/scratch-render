/**
 * @fileoverview
 * A representation of a Skin's silhouette that can test if a point on the skin
 * renders a pixel where it is drawn.
 */

/**
 * <canvas> element used to update Silhouette data from skin bitmap data.
 * @type {CanvasElement}
 */
let __SilhouetteUpdateCanvas;

// Optimized Math.min and Math.max for integers;
// taken from https://web.archive.org/web/20190716181049/http://guihaire.com/code/?p=549
const intMin = (i, j) => j ^ ((i ^ j) & ((i - j) >> 31));
const intMax = (i, j) => i ^ ((i ^ j) & ((i - j) >> 31));

/**
 * Internal helper function (in hopes that compiler can inline).  Get a pixel's alpha
 * from silhouette data, matching texture sampling rules.
 * @private
 * @param {Silhouette} silhouette - has data width and height
 * @param {number} x - x
 * @param {number} y - y
 * @return {number} Alpha value for x/y position
 */
const getPoint = ({_width: width, _height: height, _colorData: data}, x, y) => {
    // Clamp coords to edge, matching GL_CLAMP_TO_EDGE.
    x = intMax(0, intMin(x, width - 1));
    y = intMax(0, intMin(y, height - 1));

    return data[(((y * width) + x) * 4) + 3];
};

/**
 * Memory buffers for doing 4 corner sampling for linear interpolation
 */
const __cornerWork = [
    new Uint8ClampedArray(4),
    new Uint8ClampedArray(4),
    new Uint8ClampedArray(4),
    new Uint8ClampedArray(4)
];

/**
 * Get the color from a given silhouette at an x/y local texture position.
 * Multiply color values by alpha for proper blending.
 * @param {Silhouette} The silhouette to sample.
 * @param {number} x X position of texture [0, width).
 * @param {number} y Y position of texture [0, height).
 * @param {Uint8ClampedArray} dst A color 4b space.
 * @return {Uint8ClampedArray} The dst vector.
 */
const getColor4b = ({_width: width, _height: height, _colorData: data}, x, y, dst) => {
    // Clamp coords to edge, matching GL_CLAMP_TO_EDGE.
    x = intMax(0, intMin(x, width - 1));
    y = intMax(0, intMin(y, height - 1));

    const offset = ((y * width) + x) * 4;
    // premultiply alpha
    const alpha = data[offset + 3] / 255;
    dst[0] = data[offset] * alpha;
    dst[1] = data[offset + 1] * alpha;
    dst[2] = data[offset + 2] * alpha;
    dst[3] = data[offset + 3];
    return dst;
};

/**
 * Get the color from a given silhouette at an x/y local texture position.
 * Do not multiply color values by alpha, as it has already been done.
 * @param {Silhouette} The silhouette to sample.
 * @param {number} x X position of texture (0-1).
 * @param {number} y Y position of texture (0-1).
 * @param {Uint8ClampedArray} dst A color 4b space.
 * @return {Uint8ClampedArray} The dst vector.
 */
const getPremultipliedColor4b = ({_width: width, _height: height, _colorData: data}, x, y, dst) => {
    // 0 if outside bouds, otherwise read from data.
    if (x >= width || y >= height || x < 0 || y < 0) {
        return dst.fill(0);
    }
    const offset = ((y * width) + x) * 4;
    dst[0] = data[offset];
    dst[1] = data[offset + 1];
    dst[2] = data[offset + 2];
    dst[3] = data[offset + 3];
    return dst;
};

class Silhouette {
    constructor () {
        /**
         * The width of the data representing the current skin data.
         * @type {number}
         */
        this._width = 0;

        /**
         * The height of the data representing the current skin date.
         * @type {number}
         */
        this._height = 0;

        /**
         * The data representing a skin's silhouette shape.
         * @type {Uint8ClampedArray}
         */
        this._colorData = null;

        /**
         * Whether or not the color data is premultiplied with its alpha channel.
         * If it isn't, it will be multiplied here.
         * @type {boolean}
         */
        this._isPremultiplied = false;

        // By default, silhouettes are assumed not to contain premultiplied image data,
        // so when we get a color, we want to multiply it by its alpha channel.
        // Point `_getColor` to the version of the function that multiplies.
        this._getColor = getColor4b;

        this.colorAtNearest = this.colorAtLinear = (_, dst) => dst.fill(0);
    }

    /**
     * @returns {boolean} true if the silhouette color data is premultiplied, false if not.
     */
    get premultiplied () {
        return this._isPremultiplied;
    }

    /**
     * Set the alpha premultiplication state of this silhouette, to ensure proper color values are returned.
     * If set to true, the silhouette will assume it is being set with premultiplied color data,
     * and will not multiply color values by alpha.
     * If set to false, it will multiply color values by alpha.
     * @param {boolean} isPremultiplied Whether this silhouette will be populated with premultiplied color data.
     */
    set premultiplied (isPremultiplied) {
        this._isPremultiplied = isPremultiplied;

        if (isPremultiplied) {
            this._getColor = getPremultipliedColor4b;
        } else {
            this._getColor = getColor4b;
        }
    }

    /**
     * Update this silhouette with the bitmapData for a skin.
     * @param {*} bitmapData An image, canvas or other element that the skin
     * rendering can be queried from.
     */
    update (bitmapData) {
        let imageData;
        if (bitmapData instanceof ImageData) {
            // If handed ImageData directly, use it directly.
            imageData = bitmapData;
            this._width = bitmapData.width;
            this._height = bitmapData.height;
        } else {
            // Draw about anything else to our update canvas and poll image data
            // from that.
            const canvas = Silhouette._updateCanvas();
            const width = this._width = canvas.width = bitmapData.width;
            const height = this._height = canvas.height = bitmapData.height;
            const ctx = canvas.getContext('2d');

            if (!(width && height)) {
                return;
            }
            ctx.clearRect(0, 0, width, height);
            ctx.drawImage(bitmapData, 0, 0, width, height);
            imageData = ctx.getImageData(0, 0, width, height);
        }

        this._colorData = imageData.data;
        // delete our custom overriden "uninitalized" color functions
        // let the prototype work for itself
        delete this.colorAtNearest;
        delete this.colorAtLinear;
    }

    /**
     * Sample a color from the silhouette at a given local position using
     * "nearest neighbor"
     * @param {twgl.v3} vec [x,y] texture space (0-1)
     * @param {Uint8ClampedArray} dst The memory buffer to store the value in. (4 bytes)
     * @returns {Uint8ClampedArray} dst
     */
    colorAtNearest (vec, dst) {
        return this._getColor(
            this,
            Math.floor(vec[0] * this._width),
            Math.floor(vec[1] * this._height),
            dst
        );
    }

    /**
     * Sample a color from the silhouette at a given local position using
     * "linear interpolation"
     * @param {twgl.v3} vec [x,y] texture space (0-1)
     * @param {Uint8ClampedArray} dst The memory buffer to store the value in. (4 bytes)
     * @returns {Uint8ClampedArray} dst
     */
    colorAtLinear (vec, dst) {
        // In texture space, pixel centers are at integer coords. Here, the *corners* are at integers.
        // We cannot skip the "add 0.5 in Drawable.getLocalPosition -> subtract 0.5 here" roundtrip
        // because the two spaces are different--we add 0.5 in Drawable.getLocalPosition in "Scratch space"
        // (-240,240 & -180,180), but subtract 0.5 in silhouette space (0, width or height).
        // See https://web.archive.org/web/20190125211252/http://hacksoflife.blogspot.com/2009/12/texture-coordinate-system-for-opengl.html
        const x = (vec[0] * (this._width)) - 0.5;
        const y = (vec[1] * (this._height)) - 0.5;

        const x1D = x % 1;
        const y1D = y % 1;
        const x0D = 1 - x1D;
        const y0D = 1 - y1D;

        const xFloor = Math.floor(x);
        const yFloor = Math.floor(y);

        const x0y0 = this._getColor(this, xFloor, yFloor, __cornerWork[0]);
        const x1y0 = this._getColor(this, xFloor + 1, yFloor, __cornerWork[1]);
        const x0y1 = this._getColor(this, xFloor, yFloor + 1, __cornerWork[2]);
        const x1y1 = this._getColor(this, xFloor + 1, yFloor + 1, __cornerWork[3]);

        dst[0] = (x0y0[0] * x0D * y0D) + (x0y1[0] * x0D * y1D) + (x1y0[0] * x1D * y0D) + (x1y1[0] * x1D * y1D);
        dst[1] = (x0y0[1] * x0D * y0D) + (x0y1[1] * x0D * y1D) + (x1y0[1] * x1D * y0D) + (x1y1[1] * x1D * y1D);
        dst[2] = (x0y0[2] * x0D * y0D) + (x0y1[2] * x0D * y1D) + (x1y0[2] * x1D * y0D) + (x1y1[2] * x1D * y1D);
        dst[3] = (x0y0[3] * x0D * y0D) + (x0y1[3] * x0D * y1D) + (x1y0[3] * x1D * y0D) + (x1y1[3] * x1D * y1D);

        return dst;
    }

    /**
     * Test if texture coordinate touches the silhouette using nearest neighbor.
     * @param {twgl.v3} vec A texture coordinate.
     * @return {boolean} If the nearest pixel has an alpha value.
     */
    isTouchingNearest (vec) {
        if (!this._colorData) return;

        // Never touching if the coord falls outside the texture space.
        if (vec[0] < 0 || vec[0] > 1 ||
            vec[1] < 0 || vec[1] > 1) {
            return false;
        }

        return getPoint(
            this,
            Math.floor(vec[0] * this._width),
            Math.floor(vec[1] * this._height)
        ) > 0;
    }

    /**
     * Test to see if any of the 4 pixels used in the linear interpolate touch
     * the silhouette.
     * @param {twgl.v3} vec A texture coordinate.
     * @return {boolean} Any of the pixels have some alpha.
     */
    isTouchingLinear (vec) {
        if (!this._colorData) return;

        // Never touching if the coord falls outside the texture space.
        if (vec[0] < 0 || vec[0] > 1 ||
            vec[1] < 0 || vec[1] > 1) {
            return;
        }

        const x = Math.floor((vec[0] * this._width) - 0.5);
        const y = Math.floor((vec[1] * this._height) - 0.5);
        return getPoint(this, x, y) > 0 ||
            getPoint(this, x + 1, y) > 0 ||
            getPoint(this, x, y + 1) > 0 ||
            getPoint(this, x + 1, y + 1) > 0;
    }

    /**
     * Get the canvas element reused by Silhouettes to update their data with.
     * @private
     * @return {CanvasElement} A canvas to draw bitmap data to.
     */
    static _updateCanvas () {
        if (typeof __SilhouetteUpdateCanvas === 'undefined') {
            __SilhouetteUpdateCanvas = document.createElement('canvas');
        }
        return __SilhouetteUpdateCanvas;
    }
}

module.exports = Silhouette;
