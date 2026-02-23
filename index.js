import { createRequire } from 'module';
import { existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Helper function to find the binary
function findBinary() {
    const platform = process.platform;
    const arch = process.arch;

    // List of possible binary locations (in order of preference)
    const binaryPaths = [
        // Direct build output
        join(__dirname, 'index.node'),
        // Prebuilt binary location
        join(__dirname, 'prebuilds', `${platform}-${arch}`, 'op-vm.node'),
        // Alternative locations
        join(__dirname, 'build', 'Release', 'op-vm.node'),
        join(__dirname, 'native', 'index.node'),
    ];

    // Find the first existing binary
    for (const binaryPath of binaryPaths) {
        if (existsSync(binaryPath)) {
            return binaryPath;
        }
    }

    throw new Error(
        `Could not find native module for ${platform}-${arch}. ` +
            `Try running 'npm rebuild @btc-vision/op-vm' or 'npm install --build-from-source'`,
    );
}

// Load the native module
let nativeModule;
try {
    const binaryPath = findBinary();
    nativeModule = require(binaryPath);
} catch (error) {
    // Provide helpful error message
    console.error('Failed to load native module:', error.message);

    if (error.code === 'MODULE_NOT_FOUND') {
        console.error('\nPossible solutions:');
        console.error('1. Run: npm rebuild @btc-vision/op-vm');
        console.error('2. Run: npm install --build-from-source');
        console.error('3. Ensure you have Node.js >= 22');
        console.error('4. Ensure you have Rust installed: https://rustup.rs/');
    }

    throw error;
}

export default nativeModule;

export const ContractManager = nativeModule.ContractManager;

export const BitcoinNetworkRequest = {
    Mainnet: 0,
    Testnet: 1,
    Regtest: 2,
    OPNetTestnet: 3,

    0: 'Mainnet',
    1: 'Testnet',
    2: 'Regtest',
    3: 'OPNetTestnet',
};

export const NEW_STORAGE_SLOT_GAS_COST = nativeModule.NEW_STORAGE_SLOT_GAS_COST;
export const UPDATED_STORAGE_SLOT_GAS_COST = nativeModule.UPDATED_STORAGE_SLOT_GAS_COST;
