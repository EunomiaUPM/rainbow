import {defineConfig} from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "path";
import tsconfigPaths from "vite-tsconfig-paths";
import tailwind from "tailwindcss";
import autoprefixer from "autoprefixer";
import {TanStackRouterVite} from "@tanstack/router-plugin/vite";

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
