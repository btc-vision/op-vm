import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const nativeModule = require('./index.node');

export default nativeModule;

export const ContractManager = nativeModule.ContractManager;
