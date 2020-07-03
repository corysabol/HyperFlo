const tsConfig = JSON.parse(await Deno.readTextFile("./src/client/tsconfig.json"));
const compilerOptions = tsConfig["compilerOptions"];
const [diagnostics, emit] = await Deno.bundle(
  "./src/client/app.js",
  undefined,
  compilerOptions
);
if (diagnostics) {
  for (let diag of diagnostics) {
    for (let key in diag) {
      console.error(`${key}: ${diag[key]}`)
    } 
  }
}

// write the bundle out to disk
// need to emit this to a better build directory...
await Deno.writeTextFile("./src/client/app.bundle.js", emit);
