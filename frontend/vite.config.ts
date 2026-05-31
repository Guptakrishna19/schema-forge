import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Proxy API + realtime to the Rust backend during dev so the SPA can call
// same-origin paths.
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      "/api": {
        target: "http://localhost:8080",
        changeOrigin: true,
        ws: true,
      },
      "/health": "http://localhost:8080",
    },
  },
});
