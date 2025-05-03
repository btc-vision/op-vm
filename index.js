import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const nativeModule = require('./index.node');

export default nativeModule;

export const ContractManager = nativeModule.ContractManager;

export const BitcoinNetworkRequest = {
    MAINNET: 0,
    TESTNET: 1,
    REGTEST: 2,

    0: 'MAINNET',
    1: 'TESTNET',
    2: 'REGTEST',
}

export const NEW_STORAGE_SLOT_GAS_COST = nativeModule.NEW_STORAGE_SLOT_GAS_COST;
export const UPDATED_STORAGE_SLOT_GAS_COST = nativeModule.UPDATED_STORAGE_SLOT_GAS_COST;
