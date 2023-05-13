import { copyFile } from "fs/promises";
import { validation } from "json-validation";

const main = async (): Promise<void> => {
  await validation({
    schemaDirectory: "./schemas",
    typesOutFile: "./schema_types.ts",
    validationOutFile: "schemas.js",
  });

  await copyFile("schema_types.ts", "../frontend/src/lib/schema_types.ts");
  await copyFile("schemas.js", "../frontend/src/lib/schemas.js");
};

void main();
