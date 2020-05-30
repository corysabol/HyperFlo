import { assert } from "https://deno.land/std/testing/asserts.ts";
const tsConfig: any = JSON.parse(await Deno.readTextFile("./tsconfig.json"));
const compilerOptions: any = tsConfig["compilerOptions"];
const [diagnostics, emit] = await Deno.bundle(
  "./client/app.ts",
  undefined,
  compilerOptions
);
console.log(diagnostics);
console.log(emit);
//assert(diagnostics === undefined);
// write the bundle out to disk
await Deno.writeTextFile("./client/app.bundle.js", emit);
