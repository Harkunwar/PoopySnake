import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
  build: {
    minify: "esbuild",

    commonjsOptions: {
      include: [/poopy_snake_wasm/, /node_modules/],
    },
  },
  plugins: [wasmPack(["./poopy_snake_wasm"]), topLevelAwait()],
  optimizeDeps: {
    include: ["poopy_snake_wasm"],
  },
});
