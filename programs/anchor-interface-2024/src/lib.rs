use anchor_lang::prelude::*;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};
use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};
use spl_transfer_hook_interface::{
    error::TransferHookError,
    instruction::{ExecuteInstruction, TransferHookInstruction},
};

declare_id!("CgfCat6x2Rqhh3o5oKHQZXSK8N2c15EbAYrg9HjzGh5o");

fn check_account_for_hooking(account_data: &[u8]) -> Result<()> {
    //let source_account = unpack()...

    // if bool::from(source.is_hooking()) {
    //     Ok(())
    // } else {
    //     Err(InterfaceError::NotHooking)?
    // }
    Ok(())
}

#[program]
pub mod anchor_interface_2024 {
    use super::*;

    #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        metas: Vec<AnchorExtraAccountMeta>,
    ) -> Result<()> {
        let extra_metas_account = &ctx.accounts.extra_metas_account;
        let metas: Vec<ExtraAccountMeta> = metas.into_iter().map(|meta| meta.into()).collect();
        let mut data = extra_metas_account.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &metas)?;

        Ok(())
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn execute(ctx: Context<Execute>) -> Result<()> {
        //let source_account = &ctx.accounts.source_account;
        //let destination_account = &ctx.accounts.destination_account;

        //todo:
        //any source account should flip a flag before it hooks to this execute to
        //show that is it currently hooking to a interface.

        // check_account_for_hooking(&source_account.to_account_info().try_borrow_data()?)?;
        // check_account_for_hooking(&destination_account.to_account_info().try_borrow_data()?)?;
        let amount: u64 = 0;

        let data = ctx.accounts.extra_metas_account.try_borrow_data()?;
        ExtraAccountMetaList::check_account_infos::<PlaceholderInstruction>(
            &ctx.accounts.to_account_infos(),
            &TransferHookInstruction::Execute { amount }.pack(),
            &ctx.program_id,
            &data,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed:u64, metas: Vec<AnchorExtraAccountMeta>)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This account's data is a buffer of TLV data
    #[account(
        init,
        space = ExtraAccountMetaList::size_of(metas.len()).unwrap(),
        // space = 8 + 4 + 2 * 35,
        seeds = [b"extra-account-metas", seed.to_le_bytes().as_ref()],
        bump,
        payer = payer,
    )]
    pub extra_metas_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Execute<'info> {
    /// CHECK: This account's data is a buffer of TLV data
    #[account(
        seeds = [b"extra-account-metas", seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub extra_metas_account: UncheckedAccount<'info>,

    /// CHECK: Example extra PDA for transfer #1
    pub secondary_authority_1: UncheckedAccount<'info>,

    /// CHECK: Example extra PDA for transfer #2
    pub secondary_authority_2: UncheckedAccount<'info>,
}

#[derive(SplDiscriminate)]
#[discriminator_hash_input("spl-transfer-hook-interface:execute")]
pub struct PlaceholderInstruction {}

/// TLV instruction type used to initialize extra account metas
/// for the transfer hook
#[derive(SplDiscriminate)]
#[discriminator_hash_input("spl-transfer-hook-interface:initialize-extra-account-metas")]
pub struct InitializeExtraAccountMetaListInstruction {}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AnchorExtraAccountMeta {
    pub discriminator: u8,
    pub address_config: [u8; 32],
    pub is_signer: bool,
    pub is_writable: bool,
}
impl From<AnchorExtraAccountMeta> for ExtraAccountMeta {
    fn from(meta: AnchorExtraAccountMeta) -> Self {
        Self {
            discriminator: meta.discriminator,
            address_config: meta.address_config,
            is_signer: meta.is_signer.into(),
            is_writable: meta.is_writable.into(),
        }
    }
}
