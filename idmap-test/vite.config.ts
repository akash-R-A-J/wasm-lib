import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm"; // this one

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm()],
});
