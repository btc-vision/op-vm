#!/usr/bin/env node

import { existsSync } from 'fs';
import { dirname, join } from 'path';
import { spawnSync } from 'child_process';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Determine platform and architecture
const platform = process.platform;
const arch = process.arch;

// Map Node ABI versions to Node versions (for neon compatibility)
const nodeVersion = process.version.match(/^v(\d+)\./)[1];

// Check if we should build from source
const buildFromSource =
    process.env.npm_config_build_from_source === 'true' ||
    process.env.npm_package_config_build_from_source === 'true';

// Path to prebuilt binary
const prebuildPath = join(__dirname, 'prebuilds', `${platform}-${arch}`, 'op-vm.node');

// Check if this is during npm publish (skip in that case)
if (
    process.env.npm_lifecycle_event === 'prepublishOnly' ||
    process.env.npm_lifecycle_event === 'prepublish'
) {
    console.log('Skipping install during publish');
    process.exit(0);
}

// Check if prebuilt binary exists
if (!buildFromSource && existsSync(prebuildPath)) {
    console.log(`Using prebuilt binary for ${platform}-${arch} (Node ${nodeVersion})`);
    // Copy prebuild to index.node
    const fs = await import('fs');
    fs.copyFileSync(prebuildPath, join(__dirname, 'index.node'));
    process.exit(0);
}

// Check if index.node already exists (might be from a previous build)
if (!buildFromSource && existsSync(join(__dirname, 'index.node'))) {
    console.log('Native module already exists, skipping build');
    process.exit(0);
}

// Otherwise, build from source
console.log('Building from source...');

// Check Node version
if (parseInt(nodeVersion) < 22) {
    console.error(
        `Error: Node.js ${nodeVersion} is not supported. This package requires Node.js >= 22`,
    );
    process.exit(1);
}

// Check if Rust is installed
const rustCheck = spawnSync('rustc', ['--version'], { shell: true });
if (rustCheck.error || rustCheck.status !== 0) {
    console.error('Error: Rust is not installed. Please install Rust from https://rustup.rs/');
    console.error('After installing Rust, run: npm install');
    process.exit(1);
}

// Check if @neon-rs/cli is available
const neonCheck = spawnSync('neon', ['--version'], { shell: true });
if (neonCheck.error || neonCheck.status !== 0) {
    console.log('@neon-rs/cli not found globally, using local version');
}

// Build the native module
console.log('Building native module with neon...');
const build = spawnSync('npm', ['run', 'build'], {
    stdio: 'inherit',
    shell: true,
    cwd: __dirname,
});

if (build.status !== 0) {
    console.error('Build failed');
    console.error('You may need to install build tools:');
    console.error('- Windows: npm install -g windows-build-tools');
    console.error('- macOS: Install Xcode Command Line Tools');
    console.error('- Linux: sudo apt-get install build-essential');
    process.exit(1);
}

console.log('Build successful!');
