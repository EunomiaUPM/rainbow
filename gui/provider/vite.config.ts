import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "path";
import tsconfigPaths from "vite-tsconfig-paths";
import tailwind from "tailwindcss"
import autoprefixer from "autoprefixer"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), tsconfigPaths()],
  publicDir: path.resolve(__dirname, "../shared/public"),
  css: {
    postcss: {
      plugins: [tailwind, autoprefixer],
    },
  },
});
