// @ts-check

import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

import * as kIdl from "@kinobi-so/nodes-from-anchor";
import * as kJs from "@kinobi-so/renderers-js-umi";
import * as kRust from "@kinobi-so/renderers-rust";
import { bold } from "colorette";
import { ESLint } from "eslint";
import * as k from "kinobi";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

const rootDir = path.dirname(__dirname);
const idlDir = path.join(rootDir, "idls");
const clientDir = path.join(rootDir, "clients");

const idlJson = await fs.readFile(path.join(idlDir, "hpl_parimutuel.json"), "utf8");

const start = Date.now();

console.log("generating clients...");

const idl = kIdl.rootNodeFromAnchor(JSON.parse(idlJson));
const kinobi = k.createFromRoot(idl);

// Add PDA seeds for accounts.
kinobi.update(
  k.updateAccountsVisitor({
    configV1: {
      seeds: [k.constantPdaSeedNodeFromString("utf8", "config")],
    },
    userV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "user"),
        k.variablePdaSeedNode("wallet", k.publicKeyTypeNode(), "The address of the user wallet."),
      ],
    },
    marketV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "market"),
        k.variablePdaSeedNode("wallet", k.publicKeyTypeNode(), "The address of the user wallet."),
        k.variablePdaSeedNode(
          "index",
          k.numberTypeNode("u32"),
          "The index of the market in the users markets.",
        ),
      ],
    },
    userPositionV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "user_position"),
        k.variablePdaSeedNode("market", k.publicKeyTypeNode(), "The address of the market."),
        k.variablePdaSeedNode("wallet", k.publicKeyTypeNode(), "The address of the user wallet."),
      ],
    },
  }),
);

// Set default values for instruction accounts.
kinobi.update(
  k.setInstructionAccountDefaultValuesVisitor([
    {
      account: "config",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("configV1"),
    },
    {
      account: "wallet",
      ignoreIfOptional: true,
      defaultValue: k.identityValueNode(),
    },
    {
      account: "user",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("userV1", [
        k.pdaSeedValueNode("wallet", k.accountValueNode("wallet")),
      ]),
    },
    {
      account: "userPosition",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("userPositionV1", [
        k.pdaSeedValueNode("market", k.accountValueNode("market")),
        k.pdaSeedValueNode("wallet", k.accountValueNode("wallet")),
      ]),
    },
    {
      account: "deposit",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode(k.pdaLinkNode("deposit", "hooked"), [
        k.pdaSeedValueNode("market", k.accountValueNode("market")),
      ]),
    },
    {
      account: "tokenAccount",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode(k.pdaLinkNode("associatedToken", "mplToolbox"), [
        k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
        k.pdaSeedValueNode("owner", k.accountValueNode("wallet")),
      ]),
    },
    {
      account: "platformFees",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode(k.pdaLinkNode("platformFees", "hooked"), [
        k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
      ]),
    },
  ]),
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    createConfigV1: {
      arguments: {
        platformFee: { type: k.definedTypeLinkNode("bps") },
      },
    },
    createMarketV1: {
      accounts: {
        creatorFees: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("creatorFees", "hooked"), [
            k.pdaSeedValueNode("wallet", k.accountValueNode("wallet")),
            k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
          ]),
        },
      },
      arguments: {
        creatorFee: { type: k.definedTypeLinkNode("bps") },
      },
    },
    withdrawCreatorFeesV1: {
      accounts: {
        creatorFees: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("creatorFees", "hooked"), [
            k.pdaSeedValueNode("wallet", k.accountValueNode("wallet")),
            k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
          ]),
        },
      },
    },
    withdrawPlatformFeesV1: {
      accounts: {
        tokenAccount: { defaultValue: null },
      },
    },
  }),
);

// Flatten Bps.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: ["[definedTypeNode]bps"],
      transform: (node) => {
        k.assertIsNode(node, "definedTypeNode");

        return k.definedTypeNode({
          ...node,
          type: k.amountTypeNode(k.numberTypeNode("u16"), 2, "%"),
        });
      },
    },
  ]),
);
kinobi.update(k.unwrapDefinedTypesVisitor(["bps"]));

// Replace defined type links to SmallU64Array.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: ["[definedTypeLinkNode]smallU64Array"],
      transform: (node) => {
        k.assertIsNode(node, "definedTypeLinkNode");

        return k.arrayTypeNode(
          k.numberTypeNode("u64"),
          k.prefixedCountNode(k.numberTypeNode("u8")),
        );
      },
    },
  ]),
);

/** @param {string} name */
const accountType = (name) => ({
  field: "accountType",
  value: k.enumValueNode("AccountType", name),
});

// Set account discriminators.
kinobi.update(
  k.setAccountDiscriminatorFromFieldVisitor({
    ConfigV1: accountType("ConfigV1"),
    UserV1: accountType("UserV1"),
    MarketV1: accountType("MarketV1"),
    UserPositionV1: accountType("UserPositionV1"),
  }),
);

// Write kinobi IDL JSON.
await fs.writeFile(path.join(idlDir, "hpl_parimutuel_kinobi.json"), kinobi.getJson(), "utf8");

// Render Rust.
{
  const crateDir = path.join(clientDir, "rust");
  const rustDir = path.join(crateDir, "src", "generated");

  console.log(`writing rust client to ${bold(path.relative(rootDir, rustDir))}...`);

  kinobi.accept(
    kRust.renderVisitor(rustDir, {
      crateFolder: crateDir,
      formatCode: true,
      toolchain: "+nightly",
    }),
  );
}

// Render JavaScript.
{
  const jsDir = path.join(clientDir, "js", "src", "generated");

  console.log(`writing js client to ${bold(path.relative(rootDir, jsDir))}...`);

  await kinobi.accept(
    kJs.renderVisitor(jsDir, {
      formatCode: true,
    }),
  );

  console.log("cleaning up generated js client...");

  const eslint = new ESLint({
    cache: true,
    cacheLocation: path.join(rootDir, "node_modules", ".cache", "eslint-kinobi"),
    cacheStrategy: "content",
    fix: true,
  });
  const lintResults = await eslint.lintFiles(jsDir);

  await ESLint.outputFixes(lintResults);

  const eslintFormatter = await eslint.loadFormatter();
  const lintOutput = await eslintFormatter.format(lintResults);

  if (lintOutput) {
    console.error(lintOutput);
  }
}

console.log(`done in ${bold(`${Date.now() - start}ms`)}`);
