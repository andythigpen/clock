module.exports = {
  content: ["index.html", "./**/*.rs"],
  theme: {
    extend: {
      backgroundImage: {
        "gradient-45": "linear-gradient(-45deg, var(--tw-gradient-stops))",
      },
      colors: {
        darkbg: {
          800: "#0e002c",
          900: "#000000",
        },
      },
      fontFamily: {
        oswald: ["Oswald", "sans-serif"],
      },
      keyframes: {
        gradient: {
          "0%": { "background-position": "0% 50%" },
          "50%": { "background-position": "100% 50%" },
          "100%": { "background-position": "0% 50%" },
        },
      },
      animation: {
        gradient: "gradient 60s ease infinite",
      },
    },
  },
  plugins: [],
};
