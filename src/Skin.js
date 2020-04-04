const EventEmitter = require('events');

const twgl = require('twgl.js');

const RenderConstants = require('./RenderConstants');

/**
 * Truncate a number into what could be stored in a 32 bit floating point value.
 * @param {number} num Number to truncate.
 * @return {number} Truncated value.
 */
const toFloat32 = (function () {
    const memory = new Float32Array(1);
    return function (num) {
        memory[0] = num;
        return memory[0];
    };
}());

class Skin extends EventEmitter {
    /**
     * Create a Skin, which stores and/or generates textures for use in rendering.
     * @param {int} id - The unique ID for this Skin.
     * @param {!RenderWebGL} renderer - The renderer which will use this skin.
     * @constructor
     */
    constructor (id, renderer) {
        super();

        /** @type {RenderWebGL} */
        this._renderer = renderer;

        /** @type {int} */
        this._id = id;

        /** @type {Vec3} */
        this._rotationCenter = twgl.v3.create(0, 0);

        /** @type {WebGLTexture} */
        this._texture = null;

        /**
         * The uniforms to be used by the vertex and pixel shaders.
         * Some of these are used by other parts of the renderer as well.
         * @type {Object.<string,*>}
         * @private
         */
        this._uniforms = {
            /**
             * The nominal (not necessarily current) size of the current skin.
             * @type {Array<number>}
             */
            u_skinSize: [0, 0],

            /**
             * The actual WebGL texture object for the skin.
             * @type {WebGLTexture}
             */
            u_skin: null
        };

        /**
         * A silhouette to store touching data, skins are responsible for keeping it up to date.
         * @private
         */

        this.silhouetteSize = [0, 0];

        renderer.softwareRenderer.set_silhouette(id, 0, 0, new Uint8Array(0), 1, 1, true);

        this.setMaxListeners(RenderConstants.SKIN_SHARE_SOFT_LIMIT);
    }

    /**
     * Dispose of this object. Do not use it after calling this method.
     */
    dispose () {
        this._id = RenderConstants.ID_NONE;
        if (this._newSilhouette) this._newSilhouette.free();
    }

    /**
     * @returns {boolean} true for a raster-style skin (like a BitmapSkin), false for vector-style (like SVGSkin).
     */
    get isRaster () {
        return false;
    }

    /**
     * @return {int} the unique ID for this Skin.
     */
    get id () {
        return this._id;
    }

    /**
     * @returns {Vec3} the origin, in object space, about which this Skin should rotate.
     */
    get rotationCenter () {
        return this._rotationCenter;
    }

    /**
     * @abstract
     * @return {Array<number>} the "native" size, in texels, of this skin.
     */
    get size () {
        return [0, 0];
    }

    /**
     * Set the origin, in object space, about which this Skin should rotate.
     * @param {number} x - The x coordinate of the new rotation center.
     * @param {number} y - The y coordinate of the new rotation center.
     * @fires Skin.event:WasAltered
     */
    setRotationCenter (x, y) {
        const emptySkin = this.size[0] === 0 && this.size[1] === 0;
        // Compare a 32 bit x and y value against the stored 32 bit center
        // values.
        const changed = (
            toFloat32(x) !== this._rotationCenter[0] ||
            toFloat32(y) !== this._rotationCenter[1]);
        if (!emptySkin && changed) {
            this._rotationCenter[0] = x;
            this._rotationCenter[1] = y;
            this.emit(Skin.Events.WasAltered);
        }
    }

    /**
     * Get the center of the current bounding box
     * @return {Array<number>} the center of the current bounding box
     */
    calculateRotationCenter () {
        return [this.size[0] / 2, this.size[1] / 2];
    }

    /**
     * @abstract
     * @param {Array<number>} scale - The scaling factors to be used.
     * @return {WebGLTexture} The GL texture representation of this skin when drawing at the given size.
     */
    // eslint-disable-next-line no-unused-vars
    getTexture (scale) {
        return this._emptyImageTexture;
    }

    /**
     * Get the bounds of the drawable for determining its fenced position.
     * @param {Array<number>} drawable - The Drawable instance this skin is using.
     * @param {?Rectangle} result - Optional destination for bounds calculation.
     * @return {!Rectangle} The drawable's bounds.
     */
    getFenceBounds (drawable, result) {
        return drawable.getFastBounds(result);
    }

    /**
     * Update and returns the uniforms for this skin.
     * @param {Array<number>} scale - The scaling factors to be used.
     * @returns {object.<string, *>} the shader uniforms to be used when rendering with this Skin.
     */
    getUniforms (scale) {
        this._uniforms.u_skin = this.getTexture(scale);
        this._uniforms.u_skinSize = this.size;
        return this._uniforms;
    }

    /**
     * If the skin defers silhouette operations until the last possible minute,
     * this will be called before isTouching uses the silhouette.
     * @abstract
     */
    updateSilhouette () {}

    /**
     * Set this skin's texture to the given image.
     * @param {ImageData|HTMLCanvasElement} textureData - The canvas or image data to set the texture to.
     */
    _setTexture (textureData) {
        const gl = this._renderer.gl;

        gl.bindTexture(gl.TEXTURE_2D, this._texture);
        // Premultiplied alpha is necessary for proper blending.
        // See http://www.realtimerendering.com/blog/gpus-prefer-premultiplication/
        gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, textureData);
        gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, false);

        this._setSilhouetteFromData(textureData);
    }

    /**
     * Set the contents of this skin to an empty skin.
     * @fires Skin.event:WasAltered
     */
    setEmptyImageData () {
        // Free up the current reference to the _texture
        this._texture = null;

        if (!this._emptyImageData) {
            // Create a transparent pixel
            this._emptyImageData = new ImageData(1, 1);

            // Create a new texture and update the silhouette
            const gl = this._renderer.gl;

            const textureOptions = {
                auto: true,
                wrap: gl.CLAMP_TO_EDGE,
                src: this._emptyImageData
            };

            // Note: we're using _emptyImageTexture here instead of _texture
            // so that we can cache this empty texture for later use as needed.
            // this._texture can get modified by other skins (e.g. BitmapSkin
            // and SVGSkin, so we can't use that same field for caching)
            this._emptyImageTexture = twgl.createTexture(gl, textureOptions);
        }

        this._rotationCenter[0] = 0;
        this._rotationCenter[1] = 0;

        this._setSilhouetteFromData(this._emptyImageData);
        this.emit(Skin.Events.WasAltered);
    }

    _setSilhouetteFromData (data, premultiplied = false) {
        const size = this.size;
        this._renderer.softwareRenderer.set_silhouette(
            this._id,
            data.width,
            data.height,
            data.data,

            size[0],
            size[1],

            premultiplied
        );

        this.silhouetteSize[0] = data.width;
        this.silhouetteSize[1] = data.height;

        this.emit(Skin.Events.SilhouetteUpdated);
    }

}

/**
 * These are the events which can be emitted by instances of this class.
 * @enum {string}
 */
Skin.Events = {
    /**
     * Emitted when anything about the Skin has been altered, such as the appearance or rotation center.
     * @event Skin.event:WasAltered
     */
    WasAltered: 'WasAltered',

    /**
     * Emitted whenever this skin's silhouette changes.
     * @event Skin.event:SilhouetteUpdated
     */
    SilhouetteUpdated: 'SilhouetteUpdated'
};

module.exports = Skin;
