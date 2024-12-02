import init, { webp_to_png as wasm_webp_to_png } from "./wasm_webp_to_png/wasm_webp_to_png.js";

/**
 * Convert a webp image to png.
 *
 * using [image](https://github.com/image-rs/image) to convert webp image
 * using [wasm-pack](https://rustwasm.github.io/wasm-pack/) compiled to wasm
 * using [@libs/bundle](https://jsr.io/@libs/bundle) to bundle wasm
 *
 * @param data Webp data Uint8Array
 * @returns PNG data as Promised Uint8Array
 *
 * ## Example
 *
 * ```ts
 * import { webp_to_png } from "@liuxspro/webp-to-png";
 *
 * const webp_data = new Uint8Array([...])
 * console.log(await webp_to_png(webp_data))
 * ```
 */
export async function webp_to_png(data: Uint8Array): Promise<Uint8Array> {
  await init();
  return wasm_webp_to_png(data);
}
