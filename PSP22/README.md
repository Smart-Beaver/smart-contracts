## Build

https://github.com/paritytech/cargo-contract

cargo contract build --release --features "contract"

## About

### Module Overview
- `lib.rs`: The main module that includes all other modules and re-exports key components.
- `data.rs`: Defines structs and enums, which are used to manage token data and events.
- `errors.rs`: Contains  enums for error handling.
- `traits.rs`: Declares several traits like `PSP22`, `PSP22Metadata`, `PSP22Burnable`, `PSP22Mintable`, `PSP22Pausable`, `PSP22Wrapper`, and `Ownable`, which define the standard functionalities of PSP22 tokens.

### `lib.rs`
- `Token`: Main struct representing a PSP22 token. It includes methods for token creation (`new`) and for emitting events (`emit_events`).
- `Approval` and `Transfer`: Event structs used for emitting events related to token transfer and approval.

### `data.rs`
- `PSP22Data`: A struct that maintains the state of all account balances and allowances. Includes methods for token supply management, balance queries, and allowance management.
- `PSP22Event`: An enum representing events that occur during state changes of `PSP22Data`.

### `errors.rs`
- `PSP22Error`: An enum for PSP22-specific errors, such as insufficient balance or allowance.
- `OwnableError`: An enum for errors related to ownership management.

### `traits.rs`
- `PSP22`: A trait defining the core functionalities of a PSP22 token, including methods for querying supply, balance, and allowance, and for executing transfers and approvals.
- `PSP22Metadata`: A trait for accessing token metadata like name, symbol, and decimals.
- `PSP22Burnable`: A trait for token burning functionalities.
- `PSP22Mintable`: A trait for token minting functionalities.
- `PSP22Pausable`: A trait for pausing and unpausing token transfers.
- `PSP22Wrapper`: A trait for deposit and withdrawal functionalities.
- `Ownable`: A trait for ownership management, including querying owner, transferring ownership, and renouncing ownership.

### General Notes
- The library adheres to the PSP22 standard for fungible tokens.
- Most methods in `PSP22Data` and the traits in `traits.rs` return `Result` types, indicating that they can fail with a `PSP22Error`.
- Events are an essential part of the library, used for signaling state changes like transfers and approvals.

This documentation provides an overview of your Rust library's structure and functionalities. For a complete understanding, users should refer to the detailed comments and implementations within each module.