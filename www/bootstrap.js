// Bootstraps index.js and checks for errors
import("./index")
    .catch(e => console.error("Error importing index.ts: ", e));