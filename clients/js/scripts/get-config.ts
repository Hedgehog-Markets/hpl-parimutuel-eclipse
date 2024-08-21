import { displayAmount } from "@metaplex-foundation/umi";
import { bold } from "colorette";

import { findConfigV1Pda, safeFetchConfigV1 } from "../src";

import { MAINNET_URL, createUmi, formatDuration } from "./_utils";

const rpcUrl = process.env.RPC_URL ?? MAINNET_URL;
const umi = createUmi(rpcUrl);

console.log(`${bold("Cluster:")} ${umi.rpc.getCluster()}`);
console.log(`${bold("Endpoint:")} ${umi.rpc.getEndpoint()}`);

const [configPda] = findConfigV1Pda(umi);

console.log();
console.log(`${bold("Config PDA:")} ${configPda}`);

const config = await safeFetchConfigV1(umi, configPda);

if (config === null) {
  console.log();
  console.log("Program config account does not exist");

  process.exit(1);
}

console.log();
console.log(bold("Config"));
console.log(`  ${bold("Authority:")} ${config.authority}`);
console.log(`  ${bold("Platform fee:")} ${displayAmount(config.platformFee)}`);
console.log(`  ${bold("Inactive duration:")} ${formatDuration(config.inactiveDuration)}`);
