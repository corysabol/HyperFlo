import { assert } from "https://deno.land/std/testing/asserts.ts";
const tsConfig: any = JSON.parse(await Deno.readTextFile("./src/client/tsconfig.json"));
const compilerOptions: any = tsConfig["compilerOptions"];
const [diagnostics, emit] = await Deno.bundle(
  "./src/client/app.ts",
  undefined,
  compilerOptions
);
if (diagnostics) {
  for (let diag of diagnostics) {
    for (let key in diag) {
      console.error(`${key}: ${(diag as any)[key]}`)
    } 
  }
}

// write the bundle out to disk
// need to emit this to a better build directory...
await Deno.writeTextFile("./src/client/app.bundle.js", emit);
