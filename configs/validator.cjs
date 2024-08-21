//@ts-check

const path = require("path");

const rootDir = path.dirname(__dirname);
const binDir = path.join(rootDir, ".bin");

/**
 * @param {string} binary
 * @returns {string}
 */
function getProgram(binary) {
  return path.join(binDir, binary);
}

/** @type {import("@metaplex-foundation/amman").AmmanConfig} */
module.exports = {
  validator: {
    matchFeatures: "mainnet-beta",
    commitment: "confirmed",
    accountsCluster: "https://mainnetbeta-rpc.eclipse.xyz",
    programs: [
      {
        label: "Parimutuel Program",
        programId: "PARrVs6F5egaNuz8g6pKJyU4ze3eX5xGZCFb3GLiVvu",
        deployPath: getProgram("parimutuel.so"),
      },
    ],
  },
};
