import '@matterlabs/hardhat-zksync-solc';

export default {
    zksolc: {
        version: '16',
        compilerSource: 'binary',
        settings: {
            isSystem: true
        }
    },
    networks: {
        hardhat: {
            zksync: true
        }
    },
    solidity: {
        version: '0.8.16'
    }
};
