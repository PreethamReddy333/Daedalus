Basic Anatomy of an Applet

Importing the SDK

All contracts will import the Weil SDK (weil_rs), enabling them to access the execution environment, call other contracts, transfer tokens, and much more. You can also use third-party libraries, though some might not work due to the limitations of the contract runtime.

Contract's Main Structure

The contract is described through a structure:

The struct define which data the contract stores
The methods on the struct defines the exposed interface of the contract.
Contract Struct Macro

#[smart_contract] is used to annotate the impl block of generated trait implementation by the contract state. This tell the SDK to capture query and mutate inside the impl block and convert them into WASM exported functions which can be invoked externally.
#[constructor] macro tell the SDK to call the method at the time of deployment. Usually this method is used to define default initial contract state.
#[query] and #[mutate] macros are for annotating the methods which tell the SDK to insert appropriate runtime specific persistence wrappers around the exported methods for example: for mutate annotated methods, there are possible changes to the contract state which should be persisted back to the storage.
Storage: Contract State

We call the data stored in the contract the contract's state. In our counter example, the contract stores a single usize value, and the state starts initialized with the default value 0. Note: We will cover more about the contract's state in the state section.

Read Only Functions

Contract's functions can be read-only, meaning they don't modify the state. They are annotated with macro #[query].

Note: We will cover more about function types in the functions section.

State Mutating Functions

Functions that modify the state or call other contracts are considered state mutating functions. They are annotated with macro #[mutate]. State mutating functions are different because they are submitted to the Blockchain platform and goes through the consensus protocol.


Collections

When deciding on data structures used to an Applet's state, it is important to understand their tradeoffs. Choosing the wrong structure can create a bottleneck as the application scales, and migrating the state to the new data structures will come at a cost. As discussed in State, all the types inside the state must be of WeilType, which mean that they are either basic types, collections provided by us, or which satisfy some conditions on how to be (de)serialized.

Specifically regarding collections, which may be provided by us (WeilCollections), by the language's standard library or by others, you need to understand how the contract stores and loads them to decide which one to use.

TL;DR
Use native collections for small amounts of data that need to be accessed all together and SDK collections for large amounts of data that do not need to be accessed altogether.
State (De)Serialization

Each time the contract call is executed, the first thing it will do is to read the state and deserialize it into memory. Once the call finishes, it will serialize and write the state back to the database. This process has different results for Native Collections (those provided by the language) and Weil-Collections (those provided by the our SDKs).

Rust
Go
AssemblyScript
CPP
You have chosen Rust!
Native Collections

Rust
Go
AssemblyScript
CPP
Those implementing the WeilType trait:

Vec<T>
BTreeMap<K,V>
BTreeSet<V>
All entries in a native collection are serialized into a single value and stored together into the state. This means that every time a function execute, the SDK will read and deserialize all entries in the native collection. This drives to the following conclusion and usage instruction:

Native collections are useful if you are planning to store smalls amounts of data that need to be accessed all together
As the native collection grows, deserializing it from memory will cost more. If the collections grows too large, your state might not be able to fit inside the memory which would result in panicked exit from the function execution.
Weil Collections

The Contract SDK expose collections that have interfaces similar to native collections, but which are optimized for random access of large amounts of data.

Weil-Collections are instantiated using an id of type WeilId, which is used as an index to split the data into chunks.

Rust
Go
AssemblyScript
CPP
let index WeilMap::new(WeilId(0));
let records WeiVec::new(WeilId(1));

The id is combined with a key of the collection (e.g. the index of a vector/slice) to reference to the collection elements individually. This way, Weil-Collections can be read and write only the entries it really needs to, in a deferred (lazy) way.

This drives to the following conclusion and usage instruction:

Weil-Collections are useful when you are planning to store large amounts of data that do not need to be accessed altogether.
CAUTION
One should never ever use the same WeilId for two different collections even if the previous one is deleted! Using same WeilId leads to undefined behavior and can have catastrophic effects on the contract state.
Serialization Example

Rust
Go
AssemblyScript
CPP
Consider a vector with values [1, 2, 3, 4].

If a native collection is used to store it, it will be serialized into the JSON string "[1, 2, 3, 4]" in Rust.

If instead of a native Vec a WeilVec is used, it will be serialized as its WeilId. That is, if it was initialized as WeilVec::new(WeilId(i)), then it is serialized as i. As for the items in the collection, they will be saved as:

i_0: 1
i_1: 2
i_2: 3
i_3: 4

When the collection is deserialized, in the case of native collections, the whole vector is rebuilt. In the case of WeilVec, only the WeilId is loaded and only when some element of the collection is accessed will the actual element, based on its index, be deserialized and loaded.

Usage Instructions

The actual Weil-Collection API may be seen in here. Here we discuss how to use the collections in your Applet.

Generally Weil-Collections are used in the outer most contract state as that's where it spans over the scale of data that might not fit in memory. The inner attributes can use native collections, balancing the trade-off between in-memory space occupied and execution time. For example, a smart contract might have a map containing as key the wallet address and as value another map containing token to balance data. Following are the ways one can implement such data structure:

Rust
Go
AssemblyScript
CPP
// Both outer and inner map using `WeilMap<K, V>`
struct ContractState {
    balances: WeilMap<Address, TokenBalances>
}

struct TokenBalances(WeilMap<Token, uint>);

// Outer map using `WeilMap<K, V>` and inner map using `BTreeMap<K, V>`
struct ContractState {
    balances: WeilMap<Address, TokenBalances>
}

struct TokenBalances(BTreeMap<Token, uint>);

Both approaches are correct, but the second one optimizes the trade-off between in-memory usage and performance. You need to remember that each lazy get or set operation on a Weil-Collection is potentially a call to persistent storage, while a standard collection is loaded all at ones in memory. So the outer map can be Weil-Collection which might scale with the number of wallets the blockchain platform is hosting which could be potentially in millions or billions however the inner map can be stored as standard B-Tree Map as it just stores all the tokens owned by that wallet which might be few hundred or thousand at max.

So by careful inspection about the scale various attributes can attain inside contract state, we can implement quite efficient collection based data-structures.

Client SDK

Once the smart contract is deployed on the WeilChain, developers would be interested in interacting with it and build exciting applications on top of it. This is realized through the Weilliptic Client SDK. We provide client SDK in TypeScript, Rust and Golang. Let's again take example of counter widl interface:

interface Counter {
    query func get_count() -> uint;
    mutate func increment()
}

WARNING
The steps are slightly different depending on the language you are using, so make sure to chose the right language now.

Rust
Go
TypeScript
You have chosen Rust!
Now to generate client bindings from widl, use following command:

Rust
Go
TypeScript
widl generate counter.widl client rust

which will generate a bindings file

Rust
Go
TypeScript
bindings.rs
use serde::{Deserialize, Serialize};
use w_wasmutils::errors::WeilError;
use weil_wallet::{contract::ContractId, wallet::Wallet, WeilClient, WeilContractClient};

struct CounterClient {
    client: WeilContractClient,
}

impl CounterClient {
    pub fn new(contract_id: ContractId, wallet: Wallet) -> Result<Self, anyhow::Error> {
        Ok(CounterClient {
            client: WeilClient::new(wallet)?.to_contract_client(contract_id),
        })
    }

    pub async fn get_count(&self) -> Result<u32, anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
        }

        let args = Args {  };

        let resp = self.client.execute("get_count".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<u32>(&result)?;

        Ok(result)
    }

    pub async fn increment(&self) -> Result<(), anyhow::Error> {
        #[derive(Serialize)]
        struct Args {
        }

        let args = Args {  };

        let resp = self.client.execute("increment".to_string(), serde_json::to_string(&args).unwrap()).await?;

        let txn_result = serde_json::from_str::<Result<String, WeilError>>(&resp.txn_result)?;
        let result = txn_result?;
        let result = serde_json::from_str::<()>(&result)?;

        Ok(result)
    }

}


This CounterClient can be used to interact with the Counter smart contract with the provided ContractId.

Rust
Go
TypeScript
So a very simple main program in rust would look like:


#[tokio::main]
async fn main() {
// put path of the private key here!
    let private_key = PrivateKey::from_file("/root/.weilliptic/private_key.wc").unwrap();
    let wallet = Wallet::new(private_key).unwrap();

    // put your contract id here!
    let contract_id = "0000000279b490f8823ec1ce7e3c6ff2600500afa0e58eb59a1572afd25d0d4d16eb7512"
        .parse::<ContractId>()
        .unwrap();

    let client = CounterClient::new(contract_id, wallet).unwrap();

    let count = client.get_count().await.unwrap();
    println!("count: {:?}", count);

    client.increment().await.unwrap();

    let count = client.get_count().await.unwrap();
    println!("count: {:?}", count);
}


Hence client SDK is a great way to programmatically interact with the smart contract and build cool decentralized application in your favourite language


State

Smart contracts store data in their account's state, which is public on the chain. The storage starts empty until a contract is deployed and the state is initialized.

Defining

Rust
Go
AssemblyScript
The state is defined by the struct which implements the WIDL generated trait and whose trait impl block is annotated with #[smart_contract]. In the Counter Applet example, the state is defined by the CounterContractState struct. The attributes of this struct define the data that will be stored.

use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};

pub trait Counter {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_count(&self) -> usize;
    async fn increment(&mut self);
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct CounterContractState {
    // define your contract state here!
}

#[smart_contract]
impl Counter for CounterContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {... }

    #[query]
    async fn get_count(&self) -> usize {...}

    #[mutate]
    async fn increment(&mut self) {...}
}

Key Aspects

Serializable: The state must be serializable.
Ordered: The state must be ordered on serialization.
Persistent: The state is serialized and stored inside the persistent storage. This means that the state persists across function calls.
To statically ensure these properties, the Contract SDK defines WeilType and only instances of WeilType may be used inside the state. For example, in Rust a HashMap is not ordered but a BTreeMap is, so HashMap cannot be a WeilType, but BTreeMap can. WeilType is detailed in the following section.

WeilType

We call a WeilType those data types that may be used as state by an Applet. In Rust, for example, WeilType is an SDK provided trait which may be derived by the state types.
A type can derive WeilType if all of its fields (in case of structs) or variant (in case of enums) implements WeilType.

In almost all cases you would not implement this derivation manually. In Rust, for example, you will always use the derive macro WeilType to define your state type, as in the previous example.

The following table lists the basic WIDL types, which all implement WeilType, and their equivalent types in the supported languages.

WIDL	Rust	Go	AssemblyScript
u8	u8	uint8	u8
u16	u16	uint16	u16
u32	u32	uint32	u32
u64	u64	`uint64	u64
usize	usize	uint32	u32 or u64
i8	i8	int8	i8
i16	i16	int16	i16
i32	i32	int32	i32
i64	i64	int64	i64
isize	isize	int32	i32 or i64
f32	float32	f32
f64	float64	f64
bool	bool	boolean
char		N/A
string	String	string	string
Some basic collections are also WeilType, and listed in the following table.

WIDL	Rust	Go	AssemblyScript
Vec<_>		`WeilVec
BTreeMap<_, _>		Map<_, _>
BTreeSet<_>		Set<_>
Some language specific types are also WeilType, but these may not have equivalent types in all languages and should not be used as parameter or return values.

WIDL	Rust	Go	AssemblyScript
Box<T>		
Option<T>		
Result<T,E>		
() (unity)		
tuple<T,U,...>	(T,U,...)		
Whenever an exported method is called, the complete serialized state is loaded into WASM memory and deserialized to form the in-memory state object on which the method is then called. This means that a contract containing large amounts of data might not fit in memory. This is where Weil-Collections comes into the picture.

Weil-Collections are light-weight lazy loaded counterparts of the standard collections like Vec and HashMap in Rust.

WeilVec<T>, WeilMap<K, V>, WeilSet<T>, WeilTrieMap<T>, WeilMemory

More details on these collections is given in Section Collections.


Environment

The environment information is accessed using mainly two APIs:

Runtime
Runtime

Following are the runtime specific functions provided by the SDK accessed by Runtime::

API	Description
fn get_contract_id() -> String	Provides the contract address of the executing contract
fn get_sender() -> String	Provides the sender address. This can be either client Wallet address or contract address in case of cross-contract call
fn call_contract(contract_id: String, method_name: String, method_args: Option<String>) -> anyhow::Result<R>	This is used to make a cross-contract call to other contracts. More details are given in next section


Note on WIDL and Language Ergonomics

While the WIDL compiler can generate bindings for any specification to any of the supported languages, certain choices in the specification will generate code that is more or less ergonomic on different languages.

For example, the following specification will generate bindings whose methods have return values using Rust's standard Result<_,_> type. However, as Go does not have a standard equivalent type, the Weilliptic SDK for Go provides its own Result[_], which is not very ergonomic.

interface CrossCounter {
    query func fetch_counter_from(contract_id: string) -> result<uint,string>;
    mutate func increment_counter_of(contract_id: string) -> result<(),string>
}

If the contract is to be implemented in Go, then the following version of the spec would be more fitting.

interface CrossCounter {
    query func fetch_counter_from(contract_id: string) -> (uint, string)>;
    mutate func increment_counter_of(contract_id: string) -> string
}

The Go Result[_] type is provided as a convenience, for example for when cross-contract calls are made between contracts developed in different languages. Other types are provided in the same spirit and the Applet developer should use its discretion to chose whether to use a convenience type or write a spec that is more befitting of the implementation it foresees.

TIP
WIDL specifications can and should be enriched with comments that will help disambiguate the types used. For example, the following specification excerpt makes it clear that one and exactly one of the values in the returned tuple should be taken into account, for example to convert it into a Result<_,_>.

// Returns a tuple (r,e) where r must be ignored if e is not the empty string ""
query func fetch_counter_from(contract_id: string) -> (uint, error)>;

Token Standards

WeilChain provides standards for creating fungible and non-fungible tokens. The following documentation explains the core principles behind these token types, their components, common functions, and usage.

When we talk about a token, we may be talking about the set of all such tokens or a subset of units of it. For example, we can say that "Bitcoin is a cryptocurrency" or we can say that "Alice's transferred 3 Bitcoins to Bob". To avoid confusion, here we differentiate the two cases by calling the firs t a token class and the latter simply tokens.

What are Fungible and Non-Fungible Tokens?

Fungible Tokens

Fungible tokens are classes of interchangeable assets, with identical value and properties. Each unit of some fungible token class is identical to another, meaning holders can exchange one unit for another without any loss in value or change in functionality. Examples include cryptocurrencies and utility tokens, like Bitcoin, where each coin unit holds the same value and properties.

Non-Fungible Tokens (NFTs)

Non-fungible tokens are classes of unique assets and are not necessarily interchangeable on a 1:1 basis. Each non-fungible token represents a distinct item with unique properties and identifiers, making it ideal for representing digital art, collectibles, and other individual assets. Each token's uniqueness is tracked via an ID.

Structure of Tokens

Fungible Token Overview

In a WeilChain, a fungible token class is defined by an Applet that instantiates the FungibleToken type, defined in the Fungible Token standard and available in the Contract SDK.

A FungibleToken instance is defined by:

Token Name: a descriptive name of the token;
Token Symbol: a short code, e.g., WRC, that represents the token;
Decimals: the number of decimals the token is subdivided into; and,
Total Supply: the number of tokens already minted;
Within the same class, all tokens are identical in type and value and therefore need not be tracked individually, as long as their numbers are tracked. Internally, FungibleToken uses the Ledger to keep balances for every account.

Balances are changed by transferring tokens from a source account to a destination account. Transfers can be performed by the owner of the source account or by an authorized agent.

The following functions are used to query balances and allowances (transfer authorizations), to modify allowances, and to trigger transfers:

balanceOf(address): provides the balance, in fractions, of tokens for a specified address;
transfer(address, uint64): Transfers a specified number of token fractions to a given address.
approve(address, uint64): Authorizes a spender to withdraw a specified amount token fractions from the caller's account.
allowance(address, address): Returns the remaining number of token fractions that a spender is allowed to withdraw from an account.
transferFrom(address, address, uint64): Allows a spender to transfer tokens fractions from one address to another within the approved limit.
When defining a new fungible token class, these methods, exported by the FungibleToken must be implemented and re-exported, without changing their signatures. Most of the time this consists in simply delegating the call to the FungibleToken instance. This ensures that the new tokens can be transferred, approved, and queried with consistency across all contracts, making it easier to integrate the tokens with wallets, exchanges, and dApps.

Method mint, on the other hand, probably shouldn't be re-exported, as minting should be well controlled and accompanied by further logic. This can be accomplished using new methods.

Non-Fungible Token Overview

In a WeilChain, a non-fungible token class is defined by an Applet that instantiates the NonFungibleToken type, defined in the Non-Fungible Token standard and available in the Contract SDK.

A NonFungibleToken instance is defined solely by its name.

The creation of tokens in a class, or minting, may be performed all at once, during the class instantiation (e.g. a non-modifiable collection of monkey drawings) or at a later moment (e.g., to represent real world assets as they get "digitized").

Each NFT is defined by:

Token Name: (string) a short name for this NFT;
Token Title: (string) a title for the asset which this NFT represents;
Token Description: (string) additional data associated with each token (e.g., a human readable description of the NFT or a resource locator to where a a description lies, off-chain);
Token Payload: (string) the actual contents of the NFT or a resource locator to where the contents lie.
Since every token is unique inside the class, they are tracked individually. This is done by NonFungibleToken using the Ledger.

Tokens are created using method

mint(token {name:string, title:string, description:string, payload:string}, tokenId: string)

Minted tokens are transferred individually from a source account to a destination account. Transfers may be performed by the owner of the source account or by an authorized agent.

The following functions are used to query balances and allowances (transfer authorizations), to modify allowances, and to trigger transfers:

balanceOf(address: string): provides the number of NFTs owned by a given address.
ownerOf(tokenId: string): returns the owner of a specified token Id.
details(tokenId: string): returns the name, title, description and payload of the specified token Id.
transfer(toAdd: string, tokenId: string): transfers the specified token to the specified address, if owned by the caller;
approve(spender: string, tokenId: string): approves address spender to transfer token tokenId on behalf of the owner caller;
transferFrom(fromAddr: string, toAddr: string, tokenId: string): transfers ownership of token tokenId from one address fromAddr to address toAddr, checking for ownership and authorization;
setApproveForAll(spender: string, approval: bool): approves or revokes the approval for the address spender to transfer all tokens of caller/owner.
get_approved(tokenId: string): lists the addresses that may "spend" token tokenId
isApprovedForAll(owner: string, spender: String): checks if address owner has given address spender an approve for all.
When defining a new non-fungible token class, these methods, exported by NonFungibleToken must be implemented and re-exported, without changing their signatures. Most of the time this consists in simply delegating the call to the FungibleToken instance. This ensures that the new tokens can be transferred, approved, and queried with consistency across all contracts, making it easier to integrate the tokens with wallets, exchanges, and dApps.

Method mint, on the other hand, probably shouldn't be re-exported, as minting should be well controlled and accompanied by further logic. This can be accomplished using new methods.

Usage of Fungible and Non-Fungible tokens

Fungible Token Use Cases

Stablecoins: Pegged tokens that hold value against an external asset, often used in payments or value preservation.
Utility Tokens: Used within a specific platform, granting holders access to services, products, or rewards.
Governance Tokens: Enable token holders to vote on decisions within a decentralized ecosystem.
Incentives and Rewards: tokens can be used as rewards within games or platforms, adding economic incentives.
Non-Fungible Token Use Cases

Digital Art and Collectibles: Each NFT can represent a unique piece of art or collectible, allowing creators to sell unique digital assets.
Gaming Assets: In-game items like weapons, characters, or plots of land are often represented as NFTs, allowing users to trade, sell, or use them across compatible games.
Real Estate and Property Ownership: WRC721 tokens can represent real estate deeds, property ownership, and other tangible assets.
Identity and Certification: NFTs can represent digital certificates, licenses, or achievements, proving ownership and authenticity of various credentials.
Summary

WeilChains offer standards and base implementations to create, manage, and integrate fungible and non-fungible tokens into Web3.0 applications. These tokens open possibilities for innovative applications across finance, gaming, art, and real-world asset representation, enabling secure and efficient digital asset management.


Atomic Token Swap via Escrow Contracts

This document describes a protocol by which two parties, Alice and Bob, can atomically swap tokens across two distinct smart contract platforms using Hash Time-Locked Contracts (HTLCs). The protocol ensures that either both token transfers happen or neither does, eliminating counterparty risk.

In the scenario considered, Alice wishes to trade 10 units of token A for 20 units of Bob's token B.

Assumptions

Tokens & Contracts

Token A
Defined by contract A, deployed to pod PA
Contract address: A.addr
Token B
Defined by contract B, deployed to pod PB
Contract address: B.addr
HTLC_A
Escrow contract for token A, deployed to pod PA
Address: HTLC_A.addr
HTLC_B
Escrow contract for token B, deployed to pod PB
Address: HTLC_B.addr
Parties

Alice
Holds at least 10 A tokens
Address: Alice.addr
Bob
Holds at least 20 B tokens
Address: Bob.addr
Happy path Steps

PodB
PodA
Ledger_B
HTLC_B
HTLC_A
Ledger_A
Ledger_B
HTLC_B
HTLC_A
Ledger_A
Bob observes S
Alice
Bob
Offline agreement
1
allow(HTLC_A.addr, A, 10)
2
new_contract(A,Bob,t+D, hash(S))
3
escrow_id_A
4
transfer_from(Alice, HTLC_A, A)
5
Use hash(S) and escrow_id_A
6
allow(HTLC_B, B, 20)
7
new_contract(B,Alice,t, hash(S))
8
escrow_id_B
9
transfer_from(Bob, HTLC_B, B)
10
Use escrow_id_B
11
withdraw(escrow_id_B, B, S)
12
transfer(Alice, B)
13
withdraw(escrow_id_A, A,S)
14
transfer(Bob, A)
15
Alice
Bob
Step-by-step Description

Agreement: Alice and Bob agree to swap 10 A tokens for 20 B tokens.
This agreement may be reached in different forms, e.g., in person or using a matching application.
Approval: Alice allows HTLC_A to spend 10 A tokens on her behalf.
Contract Creation: Alice creates a contract via HTLC_A.new_contract, specifying Bob's address, the hash of a secret S, a deadline t+D, the token address A.addr, and the amount 10.
HTLC_A stores the escrow details and returns escrow_id_A.
HTLC_A transfers 10 A tokens from Alice to itself.
Alice sends hash(S) and escrow_id_A to Bob.
Bob allows HTLC_B to spend 20 B tokens.
Bob creates a contract via HTLC_B.new_contract, specifying Alice's address, hash(S), a deadline t, token B.addr, and the amount 20.
HTLC_B stores the escrow details and returns escrow_id_B.
HTLC_B transfers 20 B tokens from Bob to itself.
Bob informs Alice of escrow_id_B.
Alice calls HTLC_B.withdraw(escrow_id_B, S) to claim the 20 B tokens.
HTLC_B transfers 20 B tokens to Alice.
Bob, having learned S from Alice's transaction, calls HTLC_A.withdraw(escrow_id_A, S).
HTLC_A transfers 10 A tokens to Bob.
Fallback Scenario: Bob Disappears

If Bob disappears after step 6 (e.g., to stall the trade or attempt sabotage), Alice waits for the t+D deadline and recovers her tokens.

PodB
PodA
Ledger_B
HTLC_B
HTLC_A
Ledger_A
Ledger_B
HTLC_B
HTLC_A
Ledger_A
Wait until t+D
Alice
Bob
refund(escrow_id_A)
16
transfer(Alice.addr, 10)
17
Alice
Bob
Alice waits until time t+D.
Alice calls HTLC_A.refund(escrow_id_A) to recover her 10 A tokens.
HTLC_A refunds the tokens to Alice.
Choosing the Delay Parameter D

If D is too small, Alice might wait until just before time t to withdraw the B tokens, leaving Bob insufficient time to claim the A tokens before t+D. In that case, Alice could also reclaim her A tokens, unfairly acquiring both tokens.
If D is too large, and Bob disappears, Alice must wait unnecessarily long to reclaim her A tokens.
Thus, D must be chosen to provide Bob with a fair time window, while not overly delaying refunds for Alice.

Choosing the Timeout t

If Bob colludes with a validator, they may see Alice's withdrawal and intercept the secret S without broadcasting her transaction. This would let Bob use S to claim the A tokens and later refund the B tokens, effectively stealing from Alice.
To prevent this, t must be long enough to allow Alice to retry her transaction via multiple validators.
A sufficiently large value of t reduces the likelihood that a censoring validator could cause the swap to fail unfairly.


