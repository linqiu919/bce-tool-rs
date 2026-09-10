#!/usr/bin/env node

const { spawn } = require("child_process");
const path = require("path");
const os = require("os");

const PLATFORMS = {
  "darwin-x64": "@bce-tool/darwin-universal",
  "darwin-arm64": "@bce-tool/darwin-universal",
  "linux-x64": "@bce-tool/linux-x64",
  "linux-arm64": "@bce-tool/linux-arm64",
  "win32-x64": "@bce-tool/win32-x64",
  "win32-arm64": "@bce-tool/win32-arm64",
};

function getBinaryPath() {
  const platformKey = `${process.platform}-${process.arch}`;
  const pkgName = PLATFORMS[platformKey];

  if (!pkgName) {
    console.error(`Unsupported platform: ${process.platform}-${process.arch}`);
    console.error("Supported platforms: " + Object.keys(PLATFORMS).join(", "));
    process.exit(1);
  }

  try {
    const pkgPath = require.resolve(`${pkgName}/package.json`);
    const binName = process.platform === "win32" ? "bce-tool.exe" : "bce-tool";
    return path.join(path.dirname(pkgPath), "bin", binName);
  } catch (e) {
    console.error(`Failed to find platform package: ${pkgName}`);
    console.error("This may happen if npm failed to install the optional dependency.");
    console.error("");
    console.error("Try reinstalling:");
    console.error("  npm install bce-tool");
    console.error("");
    console.error("Or install the platform package directly:");
    console.error(`  npm install ${pkgName}`);
    process.exit(1);
  }
}

function run() {
  const binaryPath = getBinaryPath();
  const args = process.argv.slice(2);

  const child = spawn(binaryPath, args, {
    stdio: "inherit",
    env: process.env,
  });

  // Forward signals to child process
  const signals = ["SIGINT", "SIGTERM", "SIGHUP"];
  signals.forEach((signal) => {
    process.on(signal, () => {
      if (!child.killed) {
        child.kill(signal);
      }
    });
  });

  child.on("error", (error) => {
    console.error(`Failed to start bce-tool: ${error.message}`);
    process.exit(1);
  });

  child.on("exit", (code, signal) => {
    if (signal) {
      process.exit(128 + (os.constants.signals[signal] || 0));
    }
    process.exit(code ?? 0);
  });
}

run();
