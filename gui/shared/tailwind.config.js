import {
  theme as theme_,
  plugins as plugins_,
} from "./../shared/tailwind.config";

/** @type {import('tailwindcss').Config} */

module.exports = {
    darkMode: ["class"],
    content: [
    "index.css",
    "./src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
    "./../shared/src/**/*.{ts,tsx,js,jsx,css,sass,scss}",
    "./../shared/index.css",
  ],
  theme: {
  	extend: {
  		fontFamily: {
  			ubuntu: [
  				'Ubuntu',
  				'sans-serif'
  			],
  			source: [
  				'Source Sans Pro',
  				'Source Sans 3',
  				'sans-serif'
  			],
  			mono: [
  				'Source Code Pro"',
  				'monospace'
  			]
  		},
  		fontSize: {
  			'18': [
  				'1.125rem',
  				{
  					lineHeight: '1.5'
  				}
  			],
  			'20': [
  				'1.25rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'24': [
  				'1.5rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'28': [
  				'1.75rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'32': [
  				'2rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'36': [
  				'2.25rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'40': [
  				'2.5rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'48': [
  				'3rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'56': [
  				'3.5rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'64': [
  				'4rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'3xs': [
  				'0.5rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			'2xs': [
  				'0.625rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			xs: [
  				'0.75rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			sm: [
  				'0.875rem',
  				{
  					lineHeight: '1.4'
  				}
  			],
  			base: [
  				'1rem',
  				{
  					lineHeight: '1.5'
  				}
  			]
  		},
  		colors: {
  			foreground: {
  				'50': '#f8f8fa',
  				'100': '#f1f1f6',
  				'200': '#d3d2e0',
  				'300': '#d3d2e0',
  				'400': '#b9b7ce',
  				'500': '#9e9ab8',
  				'600': '#867ea3',
  				'700': '#786f92',
  				'800': '#645d7a',
  				'900': '#524d65',
  				'950': '#353243',
  				DEFAULT: '#d3d2e0'
  			},
  			text: '#f1f1f6',
  			stroke: '#525880',
  			roles: {
  				provider: '#52BFE0',
  				consumer: '#FF852F',
  				bussiness: '#52DBE0',
  				customer: '#FF9E2F'
  			},
  			brand: {
  				snow: '#EFF7FB',
  				sky: '#9DD5F2',
  				purple: '#62388E',
  				blue: '#24234C',
  				black: '#0D0D1C'
  			},
  			base: {
  				main: '#09091B',
  				sidebar: '#1D1C3D',
  				header: '#24234C'
  			},
  			background: {
  				'200': '#2E3356',
  				'300': '#2E3356',
  				'400': '#2E3356',
  				'600': '#09091B',
  				'800': '#07070d',
  				DEFAULT: '#09091B'
  			},
  			primary: {
  				'50': '#f1f5fd',
  				'100': '#dfe8fa',
  				'200': '#c6d7f7',
  				'300': '#9fbef1',
  				'400': '#729be8',
  				'500': '#5178e0',
  				'600': '#3c5cd4',
  				'700': '#3349c2',
  				'800': '#2d3b98',
  				'900': '#2b387d',
  				'950': '#1e244d',
  				DEFAULT: '#2D3B98',
  				foreground: '#FFFFFF'
  			},
  			secondary: {
  				'50': '#f6f4fe',
  				'100': '#eeebfc',
  				'200': '#dfdafa',
  				'300': '#c8bdf5',
  				'400': '#ab97ee',
  				'500': '#8e6de5',
  				'600': '#7e4dda',
  				'700': '#6e3bc6',
  				'800': '#542d98',
  				'900': '#4d2a88',
  				'950': '#2f195c',
  				DEFAULT: '#542D98'
  			},
  			accent: {
  				DEFAULT: '#62388E'
  			},
  			danger: {
  				'50': '#fef2f2',
  				'100': '#fee5e6',
  				'200': '#fccfd3',
  				'300': '#f9a8ae',
  				'400': '#f47883',
  				'500': '#eb485b',
  				'600': '#d42643',
  				'700': '#b61a38',
  				'800': '#981935',
  				'900': '#821933',
  				'950': '#490818',
  				DEFAULT: '#d42643'
  			},
  			warn: {
  				'50': '#fff7ed',
  				'100': '#ffeed4',
  				'200': '#ffd9a8',
  				'300': '#ffbd70',
  				'400': '#ff9537',
  				'500': '#ff7710',
  				'600': '#f05b06',
  				'700': '#c74307',
  				'800': '#9e350e',
  				'900': '#7f2e0f',
  				'950': '#451405',
  				DEFAULT: '#f05b06'
  			},
  			success: {
  				'50': '#d0fee4',
  				'100': '#a3fecd',
  				'200': '#39f3a6',
  				'300': '#33de97',
  				'400': '#2cc586',
  				'500': '#27b077',
  				'600': '#219c69',
  				'700': '#16744d',
  				'800': '#0c4f33',
  				'900': '#032A20',
  				'950': '#021A14',
  				DEFAULT: '#219c69'
  			},
  			process: {
  				'50': '#f8fdff',
  				'100': '#edfcff',
  				'200': '#cefcff',
  				'300': '#98fffc',
  				'400': '#51FFF0',
  				'500': '#00d1cd',
  				'600': '#00a4a9',
  				'700': '#007983',
  				'800': '#004e5a',
  				'900': '#002a35',
  				'950': '#001a25',
  				DEFAULT: '#51FFF0'
  			},
  			pause: {
  				'50': '#f9f9fa',
  				'100': '#f6f6f7',
  				'200': '#ebebed',
  				'300': '#e2e1e5',
  				'400': '#bfbdc6',
  				'500': '#9d99a7',
  				'600': '#7c7789',
  				'700': '#5c5769',
  				'800': '#3f3c49',
  				'900': '#232029',
  				'950': '#141218',
  				DEFAULT: '#7c7789'
  			},
  			border: {
  				DEFAULT: '#121212'
  			}
  		},
  		keyframes: {
  			'accordion-down': {
  				from: {
  					height: '0'
  				},
  				to: {
  					height: 'var(--radix-accordion-content-height)'
  				}
  			},
  			'accordion-up': {
  				from: {
  					height: 'var(--radix-accordion-content-height)'
  				},
  				to: {
  					height: '0'
  				}
  			}
  		},
  		animation: {
  			'accordion-down': 'accordion-down 0.2s ease-out',
  			'accordion-up': 'accordion-up 0.2s ease-out'
  		}
  	},
  	screens: {
  		xs: '420px',
  		sm: '640px',
  		md: '768px',
  		lg: '1024px',
  		xl: '1280px',
  		'2xl': '1536px',
  		'3xl': '1700px',
  		'4xl': '1920px',
  		'5xl': '2120px'
  	}
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
