import {
  theme as theme_,
  plugins as plugins_,
} from "./../shared/tailwind.config";

/** @type {import('tailwindcss').Config} */

module.exports = {
  content: [
    "index.css",
    "./src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
    "./../shared/src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
    "./../shared/index.css",
  ],
  theme: {
    extend: {
      fontFamily: {
        ubuntu: ["Ubuntu", "sans-serif"],
        source: ["Source Sans Pro", "Source Sans 3", "sans-serif"],
        mono: ['"Source Code Pro"', "monospace"],
      },
      fontSize: {
        "2xs": ["0.625rem", { lineHeight: "1.4" }] /* 10px */,
        xs: ["0.75rem", { lineHeight: "1.4" }] /* 12px */,
        sm: ["0.875rem", { lineHeight: "1.4" }] /* 14px */,
        base: ["1rem", { lineHeight: "1.5" }] /* 16px body */,
        18: ["1.125rem", { lineHeight: "1.5" }],
        20: ["1.25rem", { lineHeight: "1.4" }],
        24: ["1.5rem", { lineHeight: "1.4" }],
        28: ["1.75rem", { lineHeight: "1.4" }],
        32: ["2rem", { lineHeight: "1.4" }],
        36: ["2.25rem", { lineHeight: "1.4" }],
        40: ["2.5rem", { lineHeight: "1.4" }],
        48: ["3rem", { lineHeight: "1.4" }],
        56: ["3.5rem", { lineHeight: "1.4" }],
        64: ["4rem", { lineHeight: "1.4" }],
      },
      colors: {
        foreground: "#f8f8fa",
        text: "#f1f1f6",
        stroke: "#645d7a",
        roles: {
          provider: "#52BFE0",
          consumer: "#FF852F",
          bussiness: "#52DBE0",
          customer: "#FF9E2F",
        },
        brand: {
          snow: "#EFF7FB", // white
          sky: "#9DD5F2", // light blue
          purple: "#62388E",
          blue: "#24234C", // dark blue
          black: "#0D0D1C",
        },
        base: {
          // TO DO?
          main: "#09091B",
          sidebar: "#191930",
          header: "#24234C",
        },
        background: {
          DEFAULT: "#09091B",
          800: "#07070d",
          600: "#09091B",
          400: "#2E3356",
          300: "#2E3356",
          200: "#2E3356",
        },

        foreground: {
          // LIGHT BASE color palette
          DEFAULT: "#d3d2e0",
          950: "#353243",
          900: "#524d65",
          800: "#645d7a",
          700: "#786f92",
          600: "#867ea3",
          500: "#9e9ab8",
          400: "#b9b7ce",
          300: "#d3d2e0", // default, que sino el texto es muy oscuro
          200: "#d3d2e0",
          100: "#f1f1f6",
          50: "#f8f8fa",
        },
        primary: {
          // blue-ish
          DEFAULT: "#2D3B98",
          950: "#1e244d",
          900: "#2b387d",
          800: "#2d3b98", // default
          700: "#3349c2",
          600: "#3c5cd4",
          500: "#5178e0",
          400: "#729be8",
          300: "#9fbef1",
          200: "#c6d7f7",
          100: "#dfe8fa",
          50: "#f1f5fd",
          foreground: "#FFFFFF",
        },
        secondary: {
          // purple-ish
          DEFAULT: "#542D98",
          950: "#2f195c",
          900: "#4d2a88",
          800: "#542d98", // default
          700: "#6e3bc6",
          600: "#7e4dda",
          500: "#8e6de5",
          400: "#ab97ee",
          300: "#c8bdf5",
          200: "#dfdafa",
          100: "#eeebfc",
          50: "#f6f4fe",
        },
        accent: {
          DEFAULT: "#62388E",
        },

        danger: {
          // RED
          DEFAULT: "#d42643",
          950: "#490818",
          900: "#821933",
          800: "#981935",
          700: "#b61a38",
          600: "#d42643", // default
          500: "#eb485b",
          400: "#f47883",
          300: "#f9a8ae",
          200: "#fccfd3",
          100: "#fee5e6",
          50: "#fef2f2",
        },
        warn: {
          // ORANGE/YELLOW
          DEFAULT: "#f05b06",
          950: "#451405",
          900: "#7f2e0f",
          800: "#9e350e",
          700: "#c74307",
          600: "#f05b06", // default
          500: "#ff7710",
          400: "#ff9537",
          300: "#ffbd70",
          200: "#ffd9a8",
          100: "#ffeed4",
          50: "#fff7ed",
        },
        success: {
          // GREEN
          DEFAULT: "#219c69",
          950: "#021A14",
          900: "#032A20",
          800: "#0c4f33",
          700: "#16744d",
          600: "#219c69", // default
          500: "#27b077",
          400: "#2cc586",
          300: "#33de97",
          200: "#39f3a6",
          100: "#a3fecd",
          50: "#d0fee4",
        },
        process: {
          // BLUE/TURQUOISE
          DEFAULT: "#51FFF0",
          950: "#001a25",
          900: "#002a35",
          800: "#004e5a",
          700: "#007983",
          600: "#00a4a9",
          500: "#00d1cd",
          400: "#51FFF0", // default
          300: "#98fffc",
          200: "#cefcff",
          100: "#edfcff",
          50: "#f8fdff",
        },
        pause: {
          // GREY (NEUTRAL)
          DEFAULT: "#7c7789",
          950: "#141218",
          900: "#232029",
          800: "#3f3c49",
          700: "#5c5769",
          600: "#7c7789", // default
          500: "#9d99a7",
          400: "#bfbdc6",
          300: "#e2e1e5",
          200: "#ebebed",
          100: "#f6f6f7",
          50: "#f9f9fa",
        },
        // Revisiones ----------------------------------
        border: {
          DEFAULT: "#121212",
        },
        // ring: {
        //   DEFAULT: "#cbe0ed",
        // },
        // shadow: {
        //   DEFAULT: "#cbd5e1",
        // },
        // title: {
        //   DEFAULT: "#323232",
        // },
      },
    },
    screens: {
      xs: "420px",
      sm: "640px",
      md: "768px",
      lg: "1024px",
      xl: "1280px",
      "2xl": "1536px",
      "3xl": "1700px",
      "4xl": "1920px",
      "5xl": "2120px",
    },
  },
};
// export const content = [
//   "index.css",
//   "./src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
//   "./../shared/src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
//   "./../shared/index.css"
// ];

export const theme = theme_;
export const plugins = plugins_;
