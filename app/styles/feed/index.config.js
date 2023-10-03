const defaultConfig = require("../index.config");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./app/src/routes/feed/index.rs", "./app/src/components/page.rs"],
  ...defaultConfig,
};
