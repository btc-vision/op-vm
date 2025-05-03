const native = require('./index.node');
const BitcoinNetworkRequest = Object.freeze({
    Mainnet: 0,
    Testnet: 1,
    Regtest: 2
  });

const EnvironmentVariablesRequest = {
    blockHash: new Uint8Array(),
    blockNumber: 0n,
    blockMedianTime: 0n,
    txId: new Uint8Array(),
    txHash: new Uint8Array(),
    contractAddress: new Uint8Array(),
    contractDeployer: new Uint8Array(),
    caller: new Uint8Array(),
    origin: new Uint8Array()
};

module.exports = {
    BitcoinNetworkRequest: BitcoinNetworkRequest,
    EnvironmentVariablesRequest: EnvironmentVariablesRequest,
    ContractManager: native.ContractManager,
    NEW_STORAGE_SLOT_GAS_COST: native.NEW_STORAGE_SLOT_GAS_COST,
    UPDATED_STORAGE_SLOT_GAS_COST: native.UPDATED_STORAGE_SLOT_GAS_COST
};

console.log(module.exports)
console.log(module.exports)