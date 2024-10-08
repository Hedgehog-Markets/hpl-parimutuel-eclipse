//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct CreateMarketV1 {
    /// Program config
    pub config: solana_program::pubkey::Pubkey,
    /// Market
    pub market: solana_program::pubkey::Pubkey,
    /// User
    pub user: solana_program::pubkey::Pubkey,
    /// Deposit token mint
    pub mint: solana_program::pubkey::Pubkey,
    /// Deposit token account
    pub deposit: solana_program::pubkey::Pubkey,
    /// Creator fees account
    pub creator_fees: solana_program::pubkey::Pubkey,
    /// Platform fees account
    pub platform_fees: solana_program::pubkey::Pubkey,
    /// User wallet
    pub wallet: solana_program::pubkey::Pubkey,
    /// Payer
    pub payer: solana_program::pubkey::Pubkey,
    /// SPL token program
    pub token_program: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateMarketV1 {
    pub fn instruction(
        &self,
        args: CreateMarketV1InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateMarketV1InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.config, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.market, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.user, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.mint, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.deposit, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.creator_fees, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.platform_fees, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.wallet, true));
        accounts.push(solana_program::instruction::AccountMeta::new(self.payer, true));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateMarketV1InstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::HPL_PARIMUTUEL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateMarketV1InstructionData {
    discriminator: u8,
}

impl CreateMarketV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 2 }
    }
}

impl Default for CreateMarketV1InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMarketV1InstructionArgs {
    pub resolver: Pubkey,
    pub close_timestamp: i64,
    pub resolve_timestamp: i64,
    pub creator_fee: u16,
    pub options: u8,
    pub uri: String,
}

/// Instruction builder for `CreateMarketV1`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` market
///   2. `[writable]` user
///   3. `[]` mint
///   4. `[writable]` deposit
///   5. `[writable]` creator_fees
///   6. `[writable]` platform_fees
///   7. `[signer]` wallet
///   8. `[writable, signer]` payer
///   9. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   10. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct CreateMarketV1Builder {
    config: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    deposit: Option<solana_program::pubkey::Pubkey>,
    creator_fees: Option<solana_program::pubkey::Pubkey>,
    platform_fees: Option<solana_program::pubkey::Pubkey>,
    wallet: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    resolver: Option<Pubkey>,
    close_timestamp: Option<i64>,
    resolve_timestamp: Option<i64>,
    creator_fee: Option<u16>,
    options: Option<u8>,
    uri: Option<String>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateMarketV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Program config
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    /// Market
    #[inline(always)]
    pub fn market(&mut self, market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market = Some(market);
        self
    }
    /// User
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    /// Deposit token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    /// Deposit token account
    #[inline(always)]
    pub fn deposit(&mut self, deposit: solana_program::pubkey::Pubkey) -> &mut Self {
        self.deposit = Some(deposit);
        self
    }
    /// Creator fees account
    #[inline(always)]
    pub fn creator_fees(&mut self, creator_fees: solana_program::pubkey::Pubkey) -> &mut Self {
        self.creator_fees = Some(creator_fees);
        self
    }
    /// Platform fees account
    #[inline(always)]
    pub fn platform_fees(&mut self, platform_fees: solana_program::pubkey::Pubkey) -> &mut Self {
        self.platform_fees = Some(platform_fees);
        self
    }
    /// User wallet
    #[inline(always)]
    pub fn wallet(&mut self, wallet: solana_program::pubkey::Pubkey) -> &mut Self {
        self.wallet = Some(wallet);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// SPL token program
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn resolver(&mut self, resolver: Pubkey) -> &mut Self {
        self.resolver = Some(resolver);
        self
    }
    #[inline(always)]
    pub fn close_timestamp(&mut self, close_timestamp: i64) -> &mut Self {
        self.close_timestamp = Some(close_timestamp);
        self
    }
    #[inline(always)]
    pub fn resolve_timestamp(&mut self, resolve_timestamp: i64) -> &mut Self {
        self.resolve_timestamp = Some(resolve_timestamp);
        self
    }
    #[inline(always)]
    pub fn creator_fee(&mut self, creator_fee: u16) -> &mut Self {
        self.creator_fee = Some(creator_fee);
        self
    }
    #[inline(always)]
    pub fn options(&mut self, options: u8) -> &mut Self {
        self.options = Some(options);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CreateMarketV1 {
            config: self.config.expect("config is not set"),
            market: self.market.expect("market is not set"),
            user: self.user.expect("user is not set"),
            mint: self.mint.expect("mint is not set"),
            deposit: self.deposit.expect("deposit is not set"),
            creator_fees: self.creator_fees.expect("creator_fees is not set"),
            platform_fees: self.platform_fees.expect("platform_fees is not set"),
            wallet: self.wallet.expect("wallet is not set"),
            payer: self.payer.expect("payer is not set"),
            token_program: self
                .token_program
                .unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateMarketV1InstructionArgs {
            resolver: self.resolver.clone().expect("resolver is not set"),
            close_timestamp: self.close_timestamp.clone().expect("close_timestamp is not set"),
            resolve_timestamp: self
                .resolve_timestamp
                .clone()
                .expect("resolve_timestamp is not set"),
            creator_fee: self.creator_fee.clone().expect("creator_fee is not set"),
            options: self.options.clone().expect("options is not set"),
            uri: self.uri.clone().expect("uri is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_market_v1` CPI accounts.
pub struct CreateMarketV1CpiAccounts<'a, 'b> {
    /// Program config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// User
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token account
    pub deposit: &'b solana_program::account_info::AccountInfo<'a>,
    /// Creator fees account
    pub creator_fees: &'b solana_program::account_info::AccountInfo<'a>,
    /// Platform fees account
    pub platform_fees: &'b solana_program::account_info::AccountInfo<'a>,
    /// User wallet
    pub wallet: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_market_v1` CPI instruction.
pub struct CreateMarketV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// User
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token account
    pub deposit: &'b solana_program::account_info::AccountInfo<'a>,
    /// Creator fees account
    pub creator_fees: &'b solana_program::account_info::AccountInfo<'a>,
    /// Platform fees account
    pub platform_fees: &'b solana_program::account_info::AccountInfo<'a>,
    /// User wallet
    pub wallet: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateMarketV1InstructionArgs,
}

impl<'a, 'b> CreateMarketV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateMarketV1CpiAccounts<'a, 'b>,
        args: CreateMarketV1InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            market: accounts.market,
            user: accounts.user,
            mint: accounts.mint,
            deposit: accounts.deposit,
            creator_fees: accounts.creator_fees,
            platform_fees: accounts.platform_fees,
            wallet: accounts.wallet,
            payer: accounts.payer,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.config.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.market.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.user.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.mint.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.deposit.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.creator_fees.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new(*self.platform_fees.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.wallet.key, true));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.payer.key, true));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateMarketV1InstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::HPL_PARIMUTUEL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.deposit.clone());
        account_infos.push(self.creator_fees.clone());
        account_infos.push(self.platform_fees.clone());
        account_infos.push(self.wallet.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CreateMarketV1` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` market
///   2. `[writable]` user
///   3. `[]` mint
///   4. `[writable]` deposit
///   5. `[writable]` creator_fees
///   6. `[writable]` platform_fees
///   7. `[signer]` wallet
///   8. `[writable, signer]` payer
///   9. `[]` token_program
///   10. `[]` system_program
#[derive(Clone, Debug)]
pub struct CreateMarketV1CpiBuilder<'a, 'b> {
    instruction: Box<CreateMarketV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateMarketV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateMarketV1CpiBuilderInstruction {
            __program: program,
            config: None,
            market: None,
            user: None,
            mint: None,
            deposit: None,
            creator_fees: None,
            platform_fees: None,
            wallet: None,
            payer: None,
            token_program: None,
            system_program: None,
            resolver: None,
            close_timestamp: None,
            resolve_timestamp: None,
            creator_fee: None,
            options: None,
            uri: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Program config
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    /// Market
    #[inline(always)]
    pub fn market(
        &mut self,
        market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market = Some(market);
        self
    }
    /// User
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    /// Deposit token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    /// Deposit token account
    #[inline(always)]
    pub fn deposit(
        &mut self,
        deposit: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.deposit = Some(deposit);
        self
    }
    /// Creator fees account
    #[inline(always)]
    pub fn creator_fees(
        &mut self,
        creator_fees: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.creator_fees = Some(creator_fees);
        self
    }
    /// Platform fees account
    #[inline(always)]
    pub fn platform_fees(
        &mut self,
        platform_fees: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.platform_fees = Some(platform_fees);
        self
    }
    /// User wallet
    #[inline(always)]
    pub fn wallet(
        &mut self,
        wallet: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.wallet = Some(wallet);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// SPL token program
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn resolver(&mut self, resolver: Pubkey) -> &mut Self {
        self.instruction.resolver = Some(resolver);
        self
    }
    #[inline(always)]
    pub fn close_timestamp(&mut self, close_timestamp: i64) -> &mut Self {
        self.instruction.close_timestamp = Some(close_timestamp);
        self
    }
    #[inline(always)]
    pub fn resolve_timestamp(&mut self, resolve_timestamp: i64) -> &mut Self {
        self.instruction.resolve_timestamp = Some(resolve_timestamp);
        self
    }
    #[inline(always)]
    pub fn creator_fee(&mut self, creator_fee: u16) -> &mut Self {
        self.instruction.creator_fee = Some(creator_fee);
        self
    }
    #[inline(always)]
    pub fn options(&mut self, options: u8) -> &mut Self {
        self.instruction.options = Some(options);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.instruction.uri = Some(uri);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CreateMarketV1InstructionArgs {
            resolver: self.instruction.resolver.clone().expect("resolver is not set"),
            close_timestamp: self
                .instruction
                .close_timestamp
                .clone()
                .expect("close_timestamp is not set"),
            resolve_timestamp: self
                .instruction
                .resolve_timestamp
                .clone()
                .expect("resolve_timestamp is not set"),
            creator_fee: self.instruction.creator_fee.clone().expect("creator_fee is not set"),
            options: self.instruction.options.clone().expect("options is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
        };
        let instruction = CreateMarketV1Cpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            market: self.instruction.market.expect("market is not set"),

            user: self.instruction.user.expect("user is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            deposit: self.instruction.deposit.expect("deposit is not set"),

            creator_fees: self.instruction.creator_fees.expect("creator_fees is not set"),

            platform_fees: self.instruction.platform_fees.expect("platform_fees is not set"),

            wallet: self.instruction.wallet.expect("wallet is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            token_program: self.instruction.token_program.expect("token_program is not set"),

            system_program: self.instruction.system_program.expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateMarketV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    deposit: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    creator_fees: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    platform_fees: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    wallet: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    resolver: Option<Pubkey>,
    close_timestamp: Option<i64>,
    resolve_timestamp: Option<i64>,
    creator_fee: Option<u16>,
    options: Option<u8>,
    uri: Option<String>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
