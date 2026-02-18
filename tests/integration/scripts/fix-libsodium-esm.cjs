/**
 * Fix broken ESM entry in libsodium-wrappers-sumo 0.7.x
 *
 * The ESM module at libsodium-wrappers-sumo/dist/modules-sumo-esm/libsodium-wrappers.mjs
 * does `import e from "./libsodium-sumo.mjs"` but the file doesn't exist — it lives
 * in the separate libsodium-sumo package. This script copies it into place.
 */
const fs = require("fs");
const path = require("path");

const src = path.join(
  __dirname,
  "..",
  "node_modules",
  "libsodium-sumo",
  "dist",
  "modules-sumo-esm",
  "libsodium-sumo.mjs"
);
const dst = path.join(
  __dirname,
  "..",
  "node_modules",
  "libsodium-wrappers-sumo",
  "dist",
  "modules-sumo-esm",
  "libsodium-sumo.mjs"
);

if (fs.existsSync(src) && !fs.existsSync(dst)) {
  fs.copyFileSync(src, dst);
  console.log("Fixed libsodium-wrappers-sumo ESM entry");
}
