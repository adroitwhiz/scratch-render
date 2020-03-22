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

/**
 * Internal helper function (in hopes that compiler can inline).  Get a pixel
 * from silhouette data, or 0 if outside it's bounds.
 * @private
 * @param {Silhouette} silhouette - has data width and height
 * @param {number} x - x
 * @param {number} y - y
 * @return {number} Alpha value for x/y position
 */
const getPoint = ({_width: width, _height: height, _colorData: data}, x, y) => {
    // 0 if outside bounds, otherwise read from data.
    if (x >= width || y >= height || x < 0 || y < 0) {
        return 0;
    }
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
    }

    /**
     * Update this silhouette with the bitmapData for a skin.
     * @param {ImageData|HTMLCanvasElement|HTMLImageElement} bitmapData An image, canvas or other element that the skin
     * @param {boolean} isPremultiplied True if the source bitmap data comes premultiplied (e.g. from readPixels).
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
    }

    /**
     * Test if texture coordinate touches the silhouette using nearest neighbor.
     * @param {twgl.v3} vec A texture coordinate.
     * @return {boolean} If the nearest pixel has an alpha value.
     */
    isTouchingNearest (vec) {
        if (!this._colorData) return;
        return getPoint(
            this,
            Math.floor(vec[0] * (this._width - 1)),
            Math.floor(vec[1] * (this._height - 1))
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
        const x = Math.floor(vec[0] * (this._width - 1));
        const y = Math.floor(vec[1] * (this._height - 1));
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
