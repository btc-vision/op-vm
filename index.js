import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const nativeModule = require('./index.node');

export default nativeModule;

export const ContractManager = nativeModule.ContractManager;

export const BitcoinNetworkRequest = {
    Mainnet: 0,
    Testnet: 1,
    Regtest: 2,

    0: 'Mainnet',
    1: 'Testnet',
    2: 'Regtest',
};

export const NEW_STORAGE_SLOT_GAS_COST = nativeModule.NEW_STORAGE_SLOT_GAS_COST;
export const UPDATED_STORAGE_SLOT_GAS_COST = nativeModule.UPDATED_STORAGE_SLOT_GAS_COST;
