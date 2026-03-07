/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'bg-primary': '#151515',
        'bg-card': '#1e1e1e',
        'text-primary': '#ffffff',
        'text-secondary': '#cccccc',
        'accent-primary': '#3b82f6',
        'accent-secondary': '#3ce8e2',
        'border-color': '#333333',
      },
    },
  },
  plugins: [],
}
