import { ContractManager } from '../index.js';

const test = new ContractManager(16, () => {
    console.log('storage_load_js_function');
}, () => {
    console.log('storage_store_js_function');
}, () => {
    console.log('call_other_contract_js_function');
}, () => {
    console.log('deploy_from_address_js_function');
}, () => {
    console.log('console_log_js_function');
}, () => {
    console.log('emit_js_function');
}, () => {
    console.log('inputs_js_function');
}, () => {
    console.log('outputs_js_function');
}, () => {
    console.log('account_type_js_function');
}, () => {
    console.log('block_hash_js_function');
});

console.log('from prototype:', test.reserveId);
