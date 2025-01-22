/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./src/**/*.{html,rs}"],
	theme: {
		extend: {
			brightness: {
			  25: '.25',
			  175: '1.75',
			}
		}
	},
	plugins: [],
}

