
import {
  theme as theme_,
  plugins as plugins_,
} from "./../shared/tailwind.config";

/** @type {import('tailwindcss').Config} */
export const content = [
  "index.css",
  "./src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
  "./../shared/src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
  "./../shared/index.css"
];

export const theme = theme_;
export const plugins = plugins_;
