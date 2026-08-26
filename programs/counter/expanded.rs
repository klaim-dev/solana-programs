#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod constants {}
pub mod error {
    use anchor_lang::error_code;
    #[repr(u32)]
    pub enum ProgramError {
        BadInput,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ProgramError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "BadInput")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ProgramError {
        #[inline]
        fn clone(&self) -> ProgramError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ProgramError {}
    impl ProgramError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                ProgramError::BadInput => "BadInput".to_string(),
            }
        }
    }
    impl From<ProgramError> for u32 {
        fn from(e: ProgramError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<ProgramError> for anchor_lang::error::Error {
        fn from(error_code: ProgramError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for ProgramError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                ProgramError::BadInput => <Self as std::fmt::Debug>::fmt(self, fmt),
            }
        }
    }
}
pub mod instructions {
    pub mod initialize {
        use anchor_lang::prelude::*;
        use crate::state::Counter;
        pub struct Initialize<'info> {
            #[account(
                init,
                payer = authority,
                space = Counter::DISCRIMINATOR.len()+Counter::INIT_SPACE
            )]
            pub counter: Account<'info, Counter>,
            #[account(mut)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> Initialize<'info>
        where
            'info: 'info,
        {
            #[doc(hidden)]
            pub const __ANCHOR_IX_PARAM_COUNT: usize = 0;
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_0<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_1<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_2<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_3<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_4<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_5<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_6<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_7<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_8<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_9<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_10<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_11<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_12<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_13<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_14<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_15<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_16<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_17<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_18<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_19<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_20<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_21<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_22<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_23<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_24<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_25<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_26<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_27<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_28<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_29<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_30<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_31<__T>(_arg: &__T) {}
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitializeBumps> for Initialize<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitializeBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let counter = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<
                    'info,
                    System,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let counter = ({
                    #[inline(never)]
                    || {
                        let actual_field = AsRef::<AccountInfo>::as_ref(&counter);
                        let actual_owner = actual_field.owner;
                        let space = Counter::DISCRIMINATOR.len() + Counter::INIT_SPACE;
                        let pa: anchor_lang::accounts::account::Account<Counter> = if !false
                            || actual_owner
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = counter.lamports();
                            if __current_lamports == 0 {
                                let space = space;
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: authority.to_account_info(),
                                    to: counter.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.key(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context.with_signer(&[]),
                                    lamports,
                                    space as u64,
                                    __program_id,
                                )?;
                            } else {
                                if authority.key() == counter.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/counter/src/instructions/initialize.rs",
                                                        line: 3u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((authority.key(), counter.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space)
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: authority.to_account_info(),
                                        to: counter.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.key(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: counter.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.key(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context.with_signer(&[]),
                                    space as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: counter.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.key(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context.with_signer(&[]),
                                    __program_id,
                                )?;
                            }
                            match anchor_lang::accounts::account::Account::try_from_unchecked(
                                &counter,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("counter")),
                            }
                        } else {
                            match anchor_lang::accounts::account::Account::try_from(
                                &counter,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("counter")),
                            }
                        };
                        if false {
                            if space != actual_field.data_len() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintSpace,
                                        )
                                        .with_account_name("counter")
                                        .with_values((space, actual_field.data_len())),
                                );
                            }
                            if actual_owner != __program_id {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintOwner,
                                        )
                                        .with_account_name("counter")
                                        .with_pubkeys((*actual_owner, *__program_id)),
                                );
                            }
                            {
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space);
                                if pa.to_account_info().lamports() < required_lamports {
                                    return Err(
                                        anchor_lang::error::Error::from(
                                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                            )
                                            .with_account_name("counter"),
                                    );
                                }
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&counter).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("counter"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&counter).is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("counter"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        counter.to_account_info().lamports(),
                        counter.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("counter"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(Initialize {
                    counter,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.counter.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.counter.to_account_metas(Some(true)));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Initialize<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.counter, program_id)
                    .map_err(|e| e.with_account_name("counter"))?;
                anchor_lang::AccountsExit::exit(&self.authority, program_id)
                    .map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::DuplicateMutableAccountKeys for Initialize<'info>
        where
            'info: 'info,
        {
            fn duplicate_mutable_account_keys(
                &self,
            ) -> Vec<anchor_lang::solana_program::pubkey::Pubkey> {
                let mut keys = Vec::new();
                keys
            }
        }
        pub struct InitializeBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for InitializeBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "InitializeBumps")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for InitializeBumps {
            #[inline]
            fn clone(&self) -> InitializeBumps {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InitializeBumps {}
        impl Default for InitializeBumps {
            fn default() -> Self {
                InitializeBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for Initialize<'info>
        where
            'info: 'info,
        {
            type Bumps = InitializeBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Initialize`].
            pub struct Initialize {
                pub counter: Pubkey,
                pub authority: Pubkey,
                pub system_program: Pubkey,
            }
            #[automatically_derived]
            impl anchor_lang::prelude::borsh::ser::BorshSerialize for Initialize {
                fn serialize<__W: anchor_lang::prelude::borsh::io::Write>(
                    &self,
                    writer: &mut __W,
                ) -> ::core::result::Result<(), anchor_lang::prelude::borsh::io::Error> {
                    anchor_lang::prelude::borsh::BorshSerialize::serialize(
                        &self.counter,
                        writer,
                    )?;
                    anchor_lang::prelude::borsh::BorshSerialize::serialize(
                        &self.authority,
                        writer,
                    )?;
                    anchor_lang::prelude::borsh::BorshSerialize::serialize(
                        &self.system_program,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Initialize {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Initialize",
                        "counter",
                        &self.counter,
                        "authority",
                        &self.authority,
                        "system_program",
                        &&self.system_program,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for Initialize {
                #[inline]
                fn default() -> Initialize {
                    Initialize {
                        counter: ::core::default::Default::default(),
                        authority: ::core::default::Default::default(),
                        system_program: ::core::default::Default::default(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Initialize {}
            #[automatically_derived]
            impl ::core::clone::Clone for Initialize {
                #[inline]
                fn clone(&self) -> Initialize {
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    *self
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Initialize {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.counter,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize {
            use super::*;
            /// Generated CPI struct of the accounts for [`Initialize`].
            pub struct Initialize<'info> {
                pub counter: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> ::core::fmt::Debug for Initialize<'info> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Initialize",
                        "counter",
                        &self.counter,
                        "authority",
                        &self.authority,
                        "system_program",
                        &&self.system_program,
                    )
                }
            }
            #[automatically_derived]
            impl<'info> ::core::clone::Clone for Initialize<'info> {
                #[inline]
                fn clone(&self) -> Initialize<'info> {
                    Initialize {
                        counter: ::core::clone::Clone::clone(&self.counter),
                        authority: ::core::clone::Clone::clone(&self.authority),
                        system_program: ::core::clone::Clone::clone(&self.system_program),
                    }
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.counter),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.counter),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        impl<'info> Initialize<'info> {
            pub fn initialize_counter(ctx: Context<Initialize>) -> Result<()> {
                ctx.accounts.counter.value = 0;
                Ok(())
            }
        }
    }
    pub mod increment {
        use anchor_lang::prelude::*;
        use crate::state::Counter;
        pub struct Increment<'info> {
            #[account(mut)]
            counter: Account<'info, Counter>,
        }
        #[automatically_derived]
        impl<'info> Increment<'info>
        where
            'info: 'info,
        {
            #[doc(hidden)]
            pub const __ANCHOR_IX_PARAM_COUNT: usize = 0;
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_0<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_1<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_2<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_3<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_4<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_5<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_6<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_7<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_8<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_9<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_10<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_11<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_12<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_13<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_14<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_15<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_16<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_17<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_18<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_19<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_20<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_21<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_22<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_23<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_24<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_25<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_26<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_27<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_28<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_29<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_30<__T>(_arg: &__T) {}
            #[doc(hidden)]
            #[inline(always)]
            #[allow(unused)]
            pub fn __anchor_validate_ix_arg_type_31<__T>(_arg: &__T) {}
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IncrementBumps> for Increment<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IncrementBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let counter: anchor_lang::accounts::account::Account<Counter> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("counter"))?;
                {
                    let mut __mutable_accounts = std::collections::HashSet::new();
                    if let Some(key) = Some(counter.key()) {
                        if !__mutable_accounts.insert(key) {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintDuplicateMutableAccount,
                                    )
                                    .with_account_name("counter"),
                            );
                        }
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&counter).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("counter"),
                    );
                }
                Ok(Increment { counter })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Increment<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.counter.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Increment<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.counter.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Increment<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.counter, program_id)
                    .map_err(|e| e.with_account_name("counter"))?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::DuplicateMutableAccountKeys for Increment<'info>
        where
            'info: 'info,
        {
            fn duplicate_mutable_account_keys(
                &self,
            ) -> Vec<anchor_lang::solana_program::pubkey::Pubkey> {
                let mut keys = Vec::new();
                keys.push(self.counter.key());
                keys
            }
        }
        pub struct IncrementBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IncrementBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IncrementBumps")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IncrementBumps {
            #[inline]
            fn clone(&self) -> IncrementBumps {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for IncrementBumps {}
        impl Default for IncrementBumps {
            fn default() -> Self {
                IncrementBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for Increment<'info>
        where
            'info: 'info,
        {
            type Bumps = IncrementBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_increment {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Increment`].
            pub struct Increment {
                pub counter: Pubkey,
            }
            #[automatically_derived]
            impl anchor_lang::prelude::borsh::ser::BorshSerialize for Increment {
                fn serialize<__W: anchor_lang::prelude::borsh::io::Write>(
                    &self,
                    writer: &mut __W,
                ) -> ::core::result::Result<(), anchor_lang::prelude::borsh::io::Error> {
                    anchor_lang::prelude::borsh::BorshSerialize::serialize(
                        &self.counter,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Increment {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Increment",
                        "counter",
                        &&self.counter,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for Increment {
                #[inline]
                fn default() -> Increment {
                    Increment {
                        counter: ::core::default::Default::default(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Increment {}
            #[automatically_derived]
            impl ::core::clone::Clone for Increment {
                #[inline]
                fn clone(&self) -> Increment {
                    let _: ::core::clone::AssertParamIsClone<Pubkey>;
                    *self
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Increment {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.counter,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_increment {
            use super::*;
            /// Generated CPI struct of the accounts for [`Increment`].
            pub struct Increment<'info> {
                pub counter: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> ::core::fmt::Debug for Increment<'info> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Increment",
                        "counter",
                        &&self.counter,
                    )
                }
            }
            #[automatically_derived]
            impl<'info> ::core::clone::Clone for Increment<'info> {
                #[inline]
                fn clone(&self) -> Increment<'info> {
                    Increment {
                        counter: ::core::clone::Clone::clone(&self.counter),
                    }
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Increment<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.counter),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Increment<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.counter),
                        );
                    account_infos
                }
            }
        }
        impl<'info> Increment<'info> {
            pub fn increment_counter(ctx: Context<Increment>) -> Result<()> {
                ctx.accounts.counter.value += 1;
                Ok(())
            }
        }
    }
    pub use initialize::*;
    pub use increment::*;
}
pub mod state {
    use anchor_lang::prelude::*;
    pub struct Counter {
        pub value: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Space for Counter {
        const INIT_SPACE: usize = 0 + 8;
    }
    #[automatically_derived]
    impl anchor_lang::prelude::borsh::ser::BorshSerialize for Counter {
        fn serialize<__W: anchor_lang::prelude::borsh::io::Write>(
            &self,
            writer: &mut __W,
        ) -> ::core::result::Result<(), anchor_lang::prelude::borsh::io::Error> {
            anchor_lang::prelude::borsh::BorshSerialize::serialize(&self.value, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::prelude::borsh::de::BorshDeserialize for Counter {
        fn deserialize_reader<__R: anchor_lang::prelude::borsh::io::Read>(
            reader: &mut __R,
        ) -> ::core::result::Result<Self, anchor_lang::prelude::borsh::io::Error> {
            Ok(Self {
                value: anchor_lang::prelude::borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Counter {
        #[inline]
        fn clone(&self) -> Counter {
            Counter {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Counter {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(Counter::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Counter {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < Counter::DISCRIMINATOR.len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..Counter::DISCRIMINATOR.len()];
            if Counter::DISCRIMINATOR != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/counter/src/state.rs",
                                    line: 3u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("Counter"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[Counter::DISCRIMINATOR.len()..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Counter {
        const DISCRIMINATOR: &'static [u8] = &[255, 176, 4, 245, 188, 253, 124, 25];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Counter {
        fn owner() -> Pubkey {
            { crate::ID }
        }
    }
}
use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::from_str_const(
    "2WrkWYs4m32JzPnV557x9sPTCo5S8mYibAJ8ehpmBDsi",
);
/// Const version of `ID`
pub const ID_CONST: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::from_str_const(
    "2WrkWYs4m32JzPnV557x9sPTCo5S8mYibAJ8ehpmBDsi",
);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
/// Const version of `ID`
pub const fn id_const() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID_CONST
}
use self::counter::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program_entrypoint::deserialize(input)
    };
    match entry(program_id, &accounts, instruction_data) {
        Ok(()) => ::solana_program_entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one category for now.
///
/// Global methods - regular methods inside of the `#[program]`.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Check whether the declared program id matches the input program
///   id. If it's not, return an error.
/// * Find and invoke the method based on whether the instruction data
///   starts with the method's discriminator.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry<'info>(
    program_id: &'info Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &'info [u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}
fn try_entry<'info>(
    program_id: &'info Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &'info [u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct Counter;
    #[automatically_derived]
    impl ::core::clone::Clone for Counter {
        #[inline]
        fn clone(&self) -> Counter {
            Counter
        }
    }
    impl anchor_lang::Id for Counter {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each instruction's discriminator is checked until the given instruction data starts with
/// the current discriminator.
///
/// If a match is found, the instruction handler is called using the given instruction data
/// excluding the prepended discriminator bytes.
///
/// If no match is found, the fallback function is executed if it exists, or an error is
/// returned if it doesn't exist.
fn dispatch<'info>(
    program_id: &'info Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &'info [u8],
) -> anchor_lang::Result<()> {
    if data.starts_with(instruction::Initialize::DISCRIMINATOR) {
        return __private::__global::initialize(
            program_id,
            accounts,
            &data[instruction::Initialize::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(instruction::Increment::DISCRIMINATOR) {
        return __private::__global::increment(
            program_id,
            accounts,
            &data[instruction::Increment::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(anchor_lang::event::EVENT_IX_TAG_LE) {
        return Err(anchor_lang::error::ErrorCode::EventInstructionStub.into());
    }
    Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into())
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize<'info>(
            __program_id: &'info Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &'info [u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: Initialize");
            const _: () = {
                const EXPECTED_COUNT: usize = Initialize::__ANCHOR_IX_PARAM_COUNT;
                const HANDLER_PARAM_COUNT: usize = 0usize;
                if EXPECTED_COUNT > HANDLER_PARAM_COUNT {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "#[instruction(...)] on Account `Initialize<\'_>` expects MORE args, the ix `initialize(...)` has only 0 args.",
                            ),
                        );
                    };
                }
            };
            let ix = instruction::Initialize::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Initialize = ix;
            let mut __bumps = <Initialize as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts = __accounts;
            let mut __accounts = Initialize::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            unsafe fn __shrink_lifetime<'from, 'to, T>(
                value: &'from mut T,
            ) -> &'to mut T {
                unsafe { ::core::mem::transmute(value) }
            }
            let result = counter::initialize(
                anchor_lang::context::Context::new(
                    __program_id,
                    unsafe { __shrink_lifetime(&mut __accounts) },
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn increment<'info>(
            __program_id: &'info Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &'info [u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: Increment");
            const _: () = {
                const EXPECTED_COUNT: usize = Increment::__ANCHOR_IX_PARAM_COUNT;
                const HANDLER_PARAM_COUNT: usize = 0usize;
                if EXPECTED_COUNT > HANDLER_PARAM_COUNT {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "#[instruction(...)] on Account `Increment<\'_>` expects MORE args, the ix `increment(...)` has only 0 args.",
                            ),
                        );
                    };
                }
            };
            let ix = instruction::Increment::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Increment = ix;
            let mut __bumps = <Increment as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts = __accounts;
            let mut __accounts = Increment::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            unsafe fn __shrink_lifetime<'from, 'to, T>(
                value: &'from mut T,
            ) -> &'to mut T {
                unsafe { ::core::mem::transmute(value) }
            }
            let result = counter::increment(
                anchor_lang::context::Context::new(
                    __program_id,
                    unsafe { __shrink_lifetime(&mut __accounts) },
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Initialize::initialize_counter(ctx)
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        Increment::increment_counter(ctx)
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when specifying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction.
    pub struct Initialize;
    #[automatically_derived]
    impl anchor_lang::prelude::borsh::ser::BorshSerialize for Initialize {
        fn serialize<__W: anchor_lang::prelude::borsh::io::Write>(
            &self,
            writer: &mut __W,
        ) -> ::core::result::Result<(), anchor_lang::prelude::borsh::io::Error> {
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::prelude::borsh::de::BorshDeserialize for Initialize {
        fn deserialize_reader<__R: anchor_lang::prelude::borsh::io::Read>(
            reader: &mut __R,
        ) -> ::core::result::Result<Self, anchor_lang::prelude::borsh::io::Error> {
            Ok(Self {})
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Initialize {
        #[inline]
        fn clone(&self) -> Initialize {
            Initialize
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Initialize {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Initialize")
        }
    }
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: &'static [u8] = &[175, 175, 109, 31, 13, 152, 155, 237];
    }
    impl anchor_lang::InstructionData for Initialize {}
    impl anchor_lang::Owner for Initialize {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Increment;
    #[automatically_derived]
    impl anchor_lang::prelude::borsh::ser::BorshSerialize for Increment {
        fn serialize<__W: anchor_lang::prelude::borsh::io::Write>(
            &self,
            writer: &mut __W,
        ) -> ::core::result::Result<(), anchor_lang::prelude::borsh::io::Error> {
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::prelude::borsh::de::BorshDeserialize for Increment {
        fn deserialize_reader<__R: anchor_lang::prelude::borsh::io::Read>(
            reader: &mut __R,
        ) -> ::core::result::Result<Self, anchor_lang::prelude::borsh::io::Error> {
            Ok(Self {})
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Increment {
        #[inline]
        fn clone(&self) -> Increment {
            Increment
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Increment {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Increment")
        }
    }
    impl anchor_lang::Discriminator for Increment {
        const DISCRIMINATOR: &'static [u8] = &[11, 18, 104, 9, 104, 174, 59, 33];
    }
    impl anchor_lang::InstructionData for Increment {}
    impl anchor_lang::Owner for Increment {
        fn owner() -> Pubkey {
            ID
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_increment::*;
    pub use crate::__client_accounts_initialize::*;
}
