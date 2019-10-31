const twgl = require('twgl.js');

const Skin = require('./Skin');
const SvgRenderer = require('scratch-svg-renderer').SVGRenderer;

const MAX_TEXTURE_DIMENSION = 2048;

class SVGSkin extends Skin {
    /**
     * Create a new SVG skin.
     * @param {!int} id - The ID for this Skin.
     * @param {!RenderWebGL} renderer - The renderer which will use this skin.
     * @constructor
     * @extends Skin
     */
    constructor (id, renderer) {
        super(id);

        /** @type {RenderWebGL} */
        this._renderer = renderer;

        /** @type {SvgRenderer} */
        this._svgRenderer = new SvgRenderer();

        /** @type {boolean} */
        this._svgDirty = false;

        /** @type {Array<number>} */
        this._newRotationCenter = null;

        /** @type {WebGLTexture} */
        this._texture = null;

        /** @type {number} */
        this._textureScale = 1;

        /** @type {Number} */
        this._maxTextureScale = 0;
    }

    /**
     * Dispose of this object. Do not use it after calling this method.
     */
    dispose () {
        if (this._texture) {
            this._renderer.gl.deleteTexture(this._texture);
            this._texture = null;
        }
        super.dispose();
    }

    /**
     * @return {Array<number>} the natural size, in Scratch units, of this skin.
     */
    get size () {
        return this._svgRenderer.size;
    }

    /**
     * Set the origin, in object space, about which this Skin should rotate.
     * @param {number} x - The x coordinate of the new rotation center.
     * @param {number} y - The y coordinate of the new rotation center.
     */
    setRotationCenter (x, y) {
        const viewOffset = this._svgRenderer.viewOffset;
        super.setRotationCenter(x - viewOffset[0], y - viewOffset[1]);
    }

    /**
     * @param {Array<number>} scale - The scaling factors to be used, each in the [0,100] range.
     * @return {WebGLTexture} The GL texture representation of this skin when drawing at the given scale.
     */
    // eslint-disable-next-line no-unused-vars
    getTexture (scale) {
        if (!this._svgRenderer.canvas.width || !this._svgRenderer.canvas.height) {
            return super.getTexture();
        }

        // The texture only ever gets uniform scale. Take the larger of the two axes.
        const scaleMax = scale ? Math.max(Math.abs(scale[0]), Math.abs(scale[1])) : 100;
        const requestedScale = Math.min(scaleMax / 100, this._maxTextureScale);
        let newScale = this._textureScale;
        while ((newScale < this._maxTextureScale) && (requestedScale >= 1.5 * newScale)) {
            newScale *= 2;
        }
        if (this._svgDirty || this._textureScale !== newScale) {
            this._textureScale = newScale;

            const gl = this._renderer.gl;

            this._svgRenderer._draw(this._textureScale, () => {
                if (this._textureScale === newScale) {
                    // Create the texture after the SVG has been drawn to prevent a black texture from showing up
                    // during the frames for which the SVG is still being drawn.
                    if (this._texture === null) {
                        const textureOptions = {
                            auto: false,
                            wrap: gl.CLAMP_TO_EDGE
                        };

                        this._texture = twgl.createTexture(gl, textureOptions);
                    }

                    const canvas = this._svgRenderer.canvas;
                    const context = canvas.getContext('2d');
                    const textureData = context.getImageData(0, 0, canvas.width, canvas.height);

                    gl.bindTexture(gl.TEXTURE_2D, this._texture);
                    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, textureData);
                    this._silhouette.update(textureData);

                    // Defer `Drawable` updates until the new SVG has been rendered for the first time.
                    // See `setSVG` for more details on why this needs to be done.
                    if (this._svgDirty) {
                        this.setRotationCenter.apply(this, this._newRotationCenter);
                        this.emit(Skin.Events.WasAltered);
                        this._svgDirty = false;
                    }
                }
            });
        }

        return this._texture;
    }

    /**
     * Set the contents of this skin to a snapshot of the provided SVG data.
     * @param {string} svgData - new SVG to use.
     * @param {Array<number>} [rotationCenter] - Optional rotation center for the SVG. If not supplied, it will be
     * calculated from the bounding box
     * @fires Skin.event:WasAltered
     */
    setSVG (svgData, rotationCenter) {
        this._svgRenderer.loadString(svgData);

        const maxDimension = Math.max(this._svgRenderer.canvas.width, this._svgRenderer.canvas.height);
        let testScale = 2;
        for (testScale; maxDimension * testScale <= MAX_TEXTURE_DIMENSION; testScale *= 2) {
            this._maxTextureScale = testScale;
        }

        // When `setSVG` is called, a new rotation center is set.
        // However, calling `setRotationCenter` emits a `WasAltered` event, which causes all `Drawable`s with this skin
        // to recalculate their transforms. This is unwanted behavior because `setSVG` updates the texture's size, but
        // not the texture itself. That means that for 1-2 frames when the new SVG is rendering, the `Drawable`s will
        // display the old SVG, stretched to the new SVG's size. To prevent that, we emit the event in `getTexture`,
        // once the SVG is first rendered. We track whether this is the *first* render via the `_svgDirty` flag.
        if (typeof rotationCenter === 'undefined') rotationCenter = this.calculateRotationCenter();
        this._newRotationCenter = rotationCenter;

        this._svgDirty = true;
    }

}

module.exports = SVGSkin;
