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
        foreground: "#ffffff",
        text: "#fafafa",
        //foreground: "#a2adc4",
        //text: "#aec0e8",
        stroke: '#525880',
        brand: {
          snow: "#EFF7FB", // white
          sky: "#9DD5F2", // light blue
          purple: "#62388E",
          blue: "#24234C", // dark blue
          black: "#0D0D1C",
        },
        base: {
          main: "#191934",
          //sidebar: "#1D1C3D",
          sidebar: "#1f1f33",
          header: "#24234C",
        },
        background: {
          //DEFAULT:"#191934",
          DEFAULT:"#151123",
        },
        primary: {
          DEFAULT: "#056dc1",
          900: "#121212",
          800: "#121212",
          700: "#121212",
          600: "#121212",
          500: "#121212",
          400: "#121212",
          300: "#121212",
          200: "#121212",
          100: "#121212",
          50: "#121212",
          foreground: "#FFFFFF",
        },
        secondary: {
          DEFAULT: "#121212",
          700: "#121212",
          600: "#121212",
          500: "#121212",
          400: "#121212",
          300: "#121212",
          200: "#121212",
          100: "#121212",
          50: "#121212",
          foreground: "#ffffff",
        },
        accent: {
          DEFAULT: "#2D3B98", // azul, igual termina siendo el primary
        },
        border: {
          DEFAULT: "#121212",
        },
        // background: {
        //   600: "#2E3356",
        //   DEFAULT:"#151123",
        //   400:"#2E3356",
        //   300: "#2E3356", 
        //   200:  "#2E3356"
      
        // },
        ring: {
          DEFAULT: "#cbe0ed",
        },
        shadow: {
          DEFAULT: "#cbd5e1",
        },
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
