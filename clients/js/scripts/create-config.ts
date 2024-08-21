import type { Amount, PublicKey } from "@metaplex-foundation/umi";

import {
  displayAmount,
  isPublicKey,
  isZeroAmount,
  transactionBuilder,
} from "@metaplex-foundation/umi";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { bold, red } from "colorette";
import prompts from "prompts";

import { createConfigV1 } from "../src";

import {
  MAINNET_URL,
  createAmountFromDecimals,
  createUmi,
  decimalParts,
  formatDuration,
  walletKeypair,
} from "./_utils";

const rpcUrl = process.env.RPC_URL ?? MAINNET_URL;
const umi = createUmi(rpcUrl).use(walletKeypair());

console.log(`${bold("Cluster:")} ${umi.rpc.getCluster()}`);
console.log(`${bold("Endpoint:")} ${umi.rpc.getEndpoint()}`);

{
  const wallet = umi.identity.publicKey;
  const balance = await umi.rpc.getBalance(wallet);

  console.log();
  console.log(bold("Wallet"));
  console.log(`  ${bold("Address:")} ${wallet}`);
  console.log(`  ${bold("Balance:")} ${displayAmount(balance)}`);

  if (isZeroAmount(balance)) {
    console.log();
    console.log("Wallet balance is empty, are you using the correct wallet?");

    process.exit(1);
  }
}

//////////////////////////////////////////////////

type ConfigArgs = {
  authority?: PublicKey | undefined;
  platformFee?: Amount<"%", 2> | undefined;
  inactiveDuration?: number | undefined;
};

console.log();

const config: ConfigArgs = await prompts([
  {
    type: "text",
    name: "authority",
    message: "Authority",
    initial: umi.identity.publicKey,

    format: (value: string) => (value === "" ? null : value),

    validate: (value: string) =>
      value === "" || isPublicKey(value) || "Invalid public key (leave blank to use default)",
  },
  {
    type: "text",
    name: "platformFee",
    message: "Platform fee (%)",
    initial: "0",

    format: (value: string) => (value === "" ? null : createAmountFromDecimals(value, "%", 2)),

    validate: (value: string) => {
      if (value === "") {
        return true;
      }

      const parts = decimalParts(value);
      if (parts === undefined) {
        return "Invalid fee (leave blank to use default)";
      }

      const [intPart, decPart] = parts;
      if (decPart !== undefined && decPart.length > 2) {
        return "Fee can only have 2 decimals";
      }

      let amount = Number(intPart) * 100;
      if (decPart !== undefined) {
        amount += Number(decPart.padEnd(2, "0"));
      }

      if (amount > 10_000) {
        return "Fee cannot exceed 100%";
      }

      return true;
    },
  },
  {
    type: "number",
    name: "inactiveDuration",
    message: "Inactive duration (secs)",

    initial: 45 * 24 * 60 * 60,
    min: 0,
    max: 0xffffffff,
    float: false,
  },
]);

if (
  config.authority === undefined ||
  config.platformFee === undefined ||
  config.inactiveDuration === undefined
) {
  console.log();
  console.log("Cancelled.");

  process.exit(1);
}

console.log();
console.log("Proceeding will create config with the following parameters.");
console.log();
console.log(`${bold("Authority:")} ${config.authority}`);
console.log(`${bold("Platform fee:")} ${displayAmount(config.platformFee)}`);
console.log(`${bold("Inactive duration:")} ${formatDuration(config.inactiveDuration)}`);
console.log();

type ConfirmSend = {
  send?: boolean | undefined;
};

const confirm: ConfirmSend = await prompts({
  type: "confirm",
  name: "send",
  message: "Send transaction?",
  initial: false,
});

if (confirm.send !== true) {
  console.log();
  console.log("Cancelled.");

  process.exit(1);
}

const builder = transactionBuilder().append(
  createConfigV1(umi, {
    authority: config.authority,
    platformFee: config.platformFee,
    inactiveDuration: config.inactiveDuration,
  }),
);

console.log();
console.log("Sending transaction...");

const { signature: signatureBytes, result } = await builder.sendAndConfirm(umi);

const [signature] = base58.deserialize(signatureBytes);

console.log();
console.log(`${bold("Signature:")} ${signature}`);

if (result.value.err !== null) {
  console.log(`${bold(`${red("Error")}:`)} ${JSON.stringify(result.value.err)}`);

  process.exit(1);
}

process.exit(0);
