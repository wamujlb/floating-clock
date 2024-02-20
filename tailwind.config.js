/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {},
	},
	plugins: ['tailwindcss/nesting', require('daisyui')],
	daisyui: {
		themes: ['emerald', 'dim'],
	},
};
