/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { findAssociatedTokenPda } from "@metaplex-foundation/mpl-toolbox";
import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { findDepositPda } from "../../hooked";
import { findUserPositionV1Pda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";

// Accounts.
export type WithdrawV1InstructionAccounts = {
  /** Market */
  market: PublicKey | Pda;
  /** User position */
  userPosition?: PublicKey | Pda;
  /** Deposit token mint */
  mint: PublicKey | Pda;
  /** Deposit token account */
  deposit?: PublicKey | Pda;
  /** User token account */
  tokenAccount?: PublicKey | Pda;
  /** User wallet */
  wallet?: Signer;
  /** SPL token program */
  tokenProgram?: PublicKey | Pda;
};

// Data.
export type WithdrawV1InstructionData = { discriminator: number };

export type WithdrawV1InstructionDataArgs = {};

export function getWithdrawV1InstructionDataSerializer(): Serializer<
  WithdrawV1InstructionDataArgs,
  WithdrawV1InstructionData
> {
  return mapSerializer<WithdrawV1InstructionDataArgs, any, WithdrawV1InstructionData>(
    struct<WithdrawV1InstructionData>([["discriminator", u8()]], {
      description: "WithdrawV1InstructionData",
    }),
    (value) => ({ ...value, discriminator: 6 }),
  );
}

// Instruction.
export function withdrawV1(
  context: Pick<Context, "eddsa" | "identity" | "programs">,
  input: WithdrawV1InstructionAccounts,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "hplParimutuel",
    "PARrVs6F5egaNuz8g6pKJyU4ze3eX5xGZCFb3GLiVvu",
  );

  // Accounts.
  const resolvedAccounts = {
    market: {
      index: 0,
      isWritable: false as boolean,
      value: input.market ?? null,
    },
    userPosition: {
      index: 1,
      isWritable: true as boolean,
      value: input.userPosition ?? null,
    },
    mint: { index: 2, isWritable: false as boolean, value: input.mint ?? null },
    deposit: {
      index: 3,
      isWritable: true as boolean,
      value: input.deposit ?? null,
    },
    tokenAccount: {
      index: 4,
      isWritable: true as boolean,
      value: input.tokenAccount ?? null,
    },
    wallet: {
      index: 5,
      isWritable: false as boolean,
      value: input.wallet ?? null,
    },
    tokenProgram: {
      index: 6,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.wallet.value) {
    resolvedAccounts.wallet.value = context.identity;
  }
  if (!resolvedAccounts.userPosition.value) {
    resolvedAccounts.userPosition.value = findUserPositionV1Pda(context, {
      market: expectPublicKey(resolvedAccounts.market.value),
      wallet: expectPublicKey(resolvedAccounts.wallet.value),
    });
  }
  if (!resolvedAccounts.deposit.value) {
    resolvedAccounts.deposit.value = findDepositPda(context, {
      market: expectPublicKey(resolvedAccounts.market.value),
    });
  }
  if (!resolvedAccounts.tokenAccount.value) {
    resolvedAccounts.tokenAccount.value = findAssociatedTokenPda(context, {
      mint: expectPublicKey(resolvedAccounts.mint.value),
      owner: expectPublicKey(resolvedAccounts.wallet.value),
    });
  }
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      "splToken",
      "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    );
    resolvedAccounts.tokenProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: Array<ResolvedAccount> = Object.values(resolvedAccounts).sort(
    (a, b) => a.index - b.index,
  );

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
  const data = getWithdrawV1InstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
