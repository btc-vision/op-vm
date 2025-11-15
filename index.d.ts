export const enum BitcoinNetworkRequest {
    Mainnet = 0,
    Testnet = 1,
    Regtest = 2,
}
export interface EnvironmentVariablesRequest {
    readonly blockHash: Uint8Array;
    readonly blockNumber: bigint;
    readonly blockMedianTime: bigint;
    readonly txId: Uint8Array;
    readonly txHash: Uint8Array;
    readonly contractAddress: Uint8Array;
    readonly contractDeployer: Uint8Array;
    readonly caller: Uint8Array;
    readonly origin: Uint8Array;
    readonly chainId: Uint8Array;
    readonly protocolId: Uint8Array;
    readonly originTweakedPublicKey: Uint8Array;
    readonly consensusFlags: bigint;
}

export interface VMProof {
    readonly proof: Uint8Array;
    readonly vk: Uint8Array;
}

export interface ExitDataResponse {
    status: number;
    data: Buffer;
    gasUsed: bigint;
    proofs: VMProof[];
}

export interface AccountTypeResponse {
    accountType: number;
    isAddressWarm: boolean;
}

export interface BlockHashRequest {
    blockNumber: bigint;
    contractId: bigint;
}

export interface BlockHashResponse {
    blockHash: Buffer;
    isBlockWarm: boolean;
}

export const NEW_STORAGE_SLOT_GAS_COST: bigint;

export const UPDATED_STORAGE_SLOT_GAS_COST: bigint;

export interface ThreadSafeJsImportResponse {
    buffer: Array<number>;
    contractId: bigint;
}

export declare class ContractManager {
    constructor(
        maxIdlingRuntimes: number,
        storageLoadJsFunction: (
            _: never,
            result: ThreadSafeJsImportResponse,
        ) => Promise<Buffer | Uint8Array>,
        storageStoreJsFunction: (
            _: never,
            result: ThreadSafeJsImportResponse,
        ) => Promise<Buffer | Uint8Array>,
        callOtherContractJsFunction: (
            _: never,
            result: ThreadSafeJsImportResponse,
        ) => Promise<Buffer | Uint8Array>,
        deployFromAddressJsFunction: (
            _: never,
            result: ThreadSafeJsImportResponse,
        ) => Promise<Buffer | Uint8Array>,
        consoleLogJsFunction: (_: never, result: ThreadSafeJsImportResponse) => Promise<void>,
        emitJsFunction: (_: never, result: ThreadSafeJsImportResponse) => Promise<void>,
        inputsJsFunction: (id: bigint) => Promise<Buffer | Uint8Array>,
        outputsJsFunction: (id: bigint) => Promise<Buffer | Uint8Array>,
        accountTypeJsFunction: (
            _: never,
            result: ThreadSafeJsImportResponse,
        ) => Promise<AccountTypeResponse>,
        blockHashJsFunction: (_: never, result: BlockHashRequest) => Promise<BlockHashResponse>,
        mldsaLoadJsFunction: (
            _: never,
            result: ThreadSafeJsImportResponse,
        ) => Promise<Buffer | Uint8Array>,
    );

    reserveId(): bigint;

    instantiate(
        reservedId: bigint,
        address: string,
        bytecode: Buffer | undefined | null,
        usedGas: bigint,
        maxGas: bigint,
        memoryPagesUsed: bigint,
        network: BitcoinNetworkRequest,
        isDebugMode: boolean,
    ): void;

    destroyContract(id: bigint): boolean;

    destroy(): void;

    destroyCache(): void;

    destroyAll(): void;

    useGas(contractId: bigint, gas: bigint): void;

    getExitData(contractId: bigint): ExitDataResponse;

    getUsedGas(id: bigint): bigint;

    writeMemory(id: bigint, offset: bigint, data: Buffer): void;

    readMemory(id: bigint, offset: bigint, length: bigint): Buffer;

    setEnvironmentVariables(id: bigint, environmentVariables: EnvironmentVariablesRequest): void;

    onDeploy(id: bigint, calldata: Buffer): Promise<ExitDataResponse>;

    execute(id: bigint, calldata: Buffer): Promise<ExitDataResponse>;

    callExportByName(id: bigint, functionName: string, params: Array<number>): Promise<number[]>;

    length(): bigint;

    clear(): void;
}
