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

import { findCreatorFeesPda } from "../../hooked";
import { findUserV1Pda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";

// Accounts.
export type WithdrawCreatorFeesV1InstructionAccounts = {
  /** User */
  user?: PublicKey | Pda;
  /** Token mint */
  mint: PublicKey | Pda;
  /** Creator fees account */
  creatorFees?: PublicKey | Pda;
  /** User token account */
  tokenAccount?: PublicKey | Pda;
  /** User wallet */
  wallet?: Signer;
  /** SPL token program */
  tokenProgram?: PublicKey | Pda;
};

// Data.
export type WithdrawCreatorFeesV1InstructionData = { discriminator: number };

export type WithdrawCreatorFeesV1InstructionDataArgs = {};

export function getWithdrawCreatorFeesV1InstructionDataSerializer(): Serializer<
  WithdrawCreatorFeesV1InstructionDataArgs,
  WithdrawCreatorFeesV1InstructionData
> {
  return mapSerializer<
    WithdrawCreatorFeesV1InstructionDataArgs,
    any,
    WithdrawCreatorFeesV1InstructionData
  >(
    struct<WithdrawCreatorFeesV1InstructionData>([["discriminator", u8()]], {
      description: "WithdrawCreatorFeesV1InstructionData",
    }),
    (value) => ({ ...value, discriminator: 8 }),
  );
}

// Instruction.
export function withdrawCreatorFeesV1(
  context: Pick<Context, "eddsa" | "identity" | "programs">,
  input: WithdrawCreatorFeesV1InstructionAccounts,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "hplParimutuel",
    "PARrVs6F5egaNuz8g6pKJyU4ze3eX5xGZCFb3GLiVvu",
  );

  // Accounts.
  const resolvedAccounts = {
    user: { index: 0, isWritable: false as boolean, value: input.user ?? null },
    mint: { index: 1, isWritable: false as boolean, value: input.mint ?? null },
    creatorFees: {
      index: 2,
      isWritable: true as boolean,
      value: input.creatorFees ?? null,
    },
    tokenAccount: {
      index: 3,
      isWritable: true as boolean,
      value: input.tokenAccount ?? null,
    },
    wallet: {
      index: 4,
      isWritable: false as boolean,
      value: input.wallet ?? null,
    },
    tokenProgram: {
      index: 5,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.wallet.value) {
    resolvedAccounts.wallet.value = context.identity;
  }
  if (!resolvedAccounts.user.value) {
    resolvedAccounts.user.value = findUserV1Pda(context, {
      wallet: expectPublicKey(resolvedAccounts.wallet.value),
    });
  }
  if (!resolvedAccounts.creatorFees.value) {
    resolvedAccounts.creatorFees.value = findCreatorFeesPda(context, {
      wallet: expectPublicKey(resolvedAccounts.wallet.value),
      mint: expectPublicKey(resolvedAccounts.mint.value),
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
  const data = getWithdrawCreatorFeesV1InstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
