import {defineConfig} from "vite";
import react from "@vitejs/plugin-react-swc";
import tsconfigPaths from "vite-tsconfig-paths";
import {TanStackRouterVite} from "@tanstack/router-plugin/vite";
import path from "path";
import tailwind from "tailwindcss";
import autoprefixer from "autoprefixer";

export default defineConfig(() => {
  return {
    plugins: [react(), tsconfigPaths(), TanStackRouterVite({autoCodeSplitting: true})],
    publicDir: path.resolve(__dirname, "../shared/public"),
    css: {
      postcss: {
        plugins: [tailwind, autoprefixer],
      },
    },
  };
});