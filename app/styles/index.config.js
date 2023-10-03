/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./app/src/routes/index.rs", "./app/src/components/page.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
  daisyui: {
    logs: false, // Need to disable logs in order for build to succeed. See https://github.com/leptos-rs/cargo-leptos/issues/136
  },
};
