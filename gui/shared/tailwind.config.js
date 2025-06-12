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
        source: [ "Source Sans Pro",  "Source Sans 3", "sans-serif"],
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
        text: "#fefefe",
        stroke: '#525880',
        brand: {          
          snow: "#EFF7FB", // white
          sky: "#9DD5F2", // light blue
          purple: "#62388E",
          blue: "#24234C", // dark blue
          black: "#0D0D1C",
        },
        base: {
          main: '#191934',
          sidebar: '#1D1C3D',
          header: '#24234C',
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
        background: {
          600: "#2E3356",
          DEFAULT: "#2E3356",
          400: "#2E3356",
          300: "#2E3356",
          200: "#2E3356",
        },
        ring: {
          DEFAULT: "#cbe0ed",
        },
        shadow: {
          DEFAULT: "#cbd5e1",
        },
        title: {
          DEFAULT: "#323232",
        },
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

// TAILWIND CONFIG POR DEFECTO - Quiza se pueda rescatar algo de esto.

// import { fontFamily } from "tailwindcss/defaultTheme"

// /** @type {import('tailwindcss').Config} */
// export const content = [
//   "index.css",
//   "./src/**/*.{ts,tsx,js,jsx,css,sass,scss}"
// ]
// export const theme = {
//   container: {
//     center: true,
//     padding: "2rem",
//     screens: {
//       "2xl": "1400px",
//     },
//   },
//   extend: {
//     colors: {
//       border: "hsl(var(--border))",
//       input: "hsl(var(--input))",
//       ring: "hsl(var(--ring))",
//       background: "hsl(var(--background))",
//       foreground: "hsl(var(--foreground))",
//       primary: {
//         DEFAULT: "hsl(var(--primary))",
//         foreground: "hsl(var(--primary-foreground))",
//       },
//       secondary: {
//         DEFAULT: "hsl(var(--secondary))",
//         foreground: "hsl(var(--secondary-foreground))",
//       },
//       destructive: {
//         DEFAULT: "hsl(var(--destructive))",
//         foreground: "hsl(var(--destructive-foreground))",
//       },
//       muted: {
//         DEFAULT: "hsl(var(--muted))",
//         foreground: "hsl(var(--muted-foreground))",
//       },
//       accent: {
//         DEFAULT: "hsl(var(--accent))",
//         foreground: "hsl(var(--accent-foreground))",
//       },
//       popover: {
//         DEFAULT: "hsl(var(--popover))",
//         foreground: "hsl(var(--popover-foreground))",
//       },
//       card: {
//         DEFAULT: "hsl(var(--card))",
//         foreground: "hsl(var(--card-foreground))",
//       },
//     },
//     borderRadius: {
//       lg: `var(--radius)`,
//       md: `calc(var(--radius) - 2px)`,
//       sm: "calc(var(--radius) - 4px)",
//     },
//     fontFamily: {
//       sans: ["var(--font-sans)", ...fontFamily.sans],
//     },
//     keyframes: {
//       "accordion-down": {
//         from: { height: "0" },
//         to: { height: "var(--radix-accordion-content-height)" },
//       },
//       "accordion-up": {
//         from: { height: "var(--radix-accordion-content-height)" },
//         to: { height: "0" },
//       },
//     },
//     animation: {
//       "accordion-down": "accordion-down 0.2s ease-out",
//       "accordion-up": "accordion-up 0.2s ease-out",
//     },
//   },
// }
// // eslint-disable-next-line no-undef
// export const plugins = [require("tailwindcss-animate")]
