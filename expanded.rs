#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod constants {
    use anchor_lang::prelude::*;
    pub const SEED: &str = "anchor";
}
pub mod error {
    use anchor_lang::prelude::*;
    #[repr(u32)]
    pub enum ErrorCode {
        CustomError,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorCode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "CustomError")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorCode {
        #[inline]
        fn clone(&self) -> ErrorCode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorCode {}
    impl ErrorCode {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                ErrorCode::CustomError => "CustomError".to_string(),
            }
        }
    }
    impl From<ErrorCode> for u32 {
        fn from(e: ErrorCode) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<ErrorCode> for anchor_lang::error::Error {
        fn from(error_code: ErrorCode) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for ErrorCode {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                ErrorCode::CustomError => {
                    fmt.write_fmt(format_args!("Custom error message"))
                }
            }
        }
    }
}
pub mod instructions {
    pub mod initialize {
        use anchor_lang::prelude::*;
        pub struct Initialize {}
        #[automatically_derived]
        impl<'info> Initialize {
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
        impl<'info> anchor_lang::Accounts<'info, InitializeBumps> for Initialize {
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
                Ok(Initialize {})
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
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
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Initialize {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                Ok(())
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::DuplicateMutableAccountKeys for Initialize {
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
        impl<'info> anchor_lang::Bumps for Initialize {
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
            pub struct Initialize {}
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
            impl ::core::fmt::Debug for Initialize {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Initialize")
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for Initialize {
                #[inline]
                fn default() -> Initialize {
                    Initialize {}
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Initialize {}
            #[automatically_derived]
            impl ::core::clone::Clone for Initialize {
                #[inline]
                fn clone(&self) -> Initialize {
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
            pub struct Initialize {}
            #[automatically_derived]
            impl ::core::fmt::Debug for Initialize {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Initialize")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Initialize {
                #[inline]
                fn clone(&self) -> Initialize {
                    Initialize {}
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
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                }
            }
        }
        pub fn handler(ctx: Context<Initialize>) -> Result<()> {
            ::solana_msg::sol_log(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("Greetings from: {0:?}", ctx.program_id),
                    )
                }),
            );
            Ok(())
        }
    }
    pub use initialize::*;
}
pub mod state {}
use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::from_str_const(
    "BzDa1WDtCVVhy3PasxqZk4L4BSdxhc68ji9qnw5B42bA",
);
/// Const version of `ID`
pub const ID_CONST: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::from_str_const(
    "BzDa1WDtCVVhy3PasxqZk4L4BSdxhc68ji9qnw5B42bA",
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
    }
}
pub mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
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
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_initialize::*;
}
