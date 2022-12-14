const { writeFileSync, copyFileSync } = require("fs");
const pck = require("../package.json");

const buildPath = `${process.env.PWD}/build`;

pck.scripts = {};
pck.private = false;
pck.type = "module";
pck.files = ["**/*", "!**/tsconfig.tsbuildinfo", "!**/*.tgz"];

writeFileSync(`${buildPath}/package.json`, JSON.stringify(pck, null, 2));
copyFileSync("README.md", `${buildPath}/README.md`);

copyFileSync("./src/index.cjs", `${buildPath}/index.cjs`);