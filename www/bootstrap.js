// Bootstraps index.js and checks for errors
import("./index.js")
    .catch(e => console.error("Error importing index.js: ", e));