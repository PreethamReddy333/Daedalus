 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

# Weilliptic Documentation

Our Applet and MCP server development kit, including examples, is available in our [GitHub repo](https://github.com/weilliptic-public/wadk).

Our documentation has been divided into four sections:

  * [Tutorials](/docs/tutorials/): to learn how WeilChains works.
  * [How-To's](/docs/howtos/): to execute specific tasks, for those already familiar with WeilChain.
  * [Explanations](/docs/explainers/): to read in-depth discussions of design and workings of WeilChains.
  * [Reference](/docs/reference/): to quickly find out or reviewing concepts.



Copyright © 2026 Weilliptic, Inc.

                
 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

On this page

# Create a Basic Applet

Generally the life-cycle of a Weilliptic Applet, a WeilChain smart-contract, consists of the following steps:

  1. Specifying the Applet's interface using WIDL.
  2. Generate server bindings for the language of your choice using the WIDL compiler.
  3. Fill in the bindings with business logic.
  4. Compile the applet into a WASM module and deploy on the WeilChain platform.
  5. Use the exposed methods of the Applet using either the [Weilliptic CLI](/docs/reference/cli), or Wallet, or DApp.



In this tutorial we'll implement a basic Counter Applet.

If you prefer a video explanation, Here is a complete end-to-end walkthrough of writing, deploying and interacting smart contracts on WeilChain.

warning

The steps are slightly different depending on the language you are using, so make sure to chose the right language now.

  * Rust
  * Go
  * AssemblyScript
  * CPP



You have chosen Rust!

You have chosen Go!

You have chosen AssemblyScript!

You have chosen C++!

## Preparations​

  * Rust
  * Go
  * AssemblyScript
  * C++



This tutorial assumes that

  * you have completed [How-To install the WIDL compiler and extension](/docs/howtos/install_widl).
  * you have installed the tools for the supported language, i.e., Rust, Golang, AssemblyScript, or C++.



To start, create a new project:

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    cargo new counter_rust --lib  
    cd counter_rust  
    
    
    
    mkdir counter_go  
    cd counter_go  
    go mod init main  
    go mod tidy  
    mkdir contract  
    
    
    
    mkdir counter_as  
    cd counter_as  
    npm init  
    npm install --save-dev assemblyscript  
    npx asinit .  
    npm install assemblyscript-json json-as visitor-as --force  
    

After that, add the following to your `asconfig.json`.
    
    
    {  
      "options": {  
        "transform": ["json-as/transform"]  
      }  
    }  
    

Refer to the `json-as` [documentation](https://www.npmjs.com/package/json-as) for more information on what this does.
    
    
    mkdir counterCpp  
    cd CounterCPP  
    touch CMakeLists.txt  
    

important

Prerequisites:

  * Add an _include_ folder in the root of your project, which contains all the required headers to work with the C++ SDK.
  * Add the statically compiled library code (libweilsdk_static.a) in your project in the _lib_ folder.
  * You need to have [emscripten](https://emscripten.org/docs/getting_started/downloads.html) installed.



## Contract Specification​

Create file `counter.widl`, with the following contents.

counter.widl
    
    
    interface Counter {  
        query func get_count() -> uint;  
        mutate func increment()  
    }  
    

This file defines a service called `Counter` with two methods, `get_count` and `increment()`.

Observe that the definition of `get_count()` starts with `query` but the definition of `increment()` starts with `mutate`. These indicate if the operation will merely consult the state or can potentially update it.

`query`

`query` methods will not update the state of the Applet. Even if your method does update the state, the change will not be persisted.

`mutate`

`mutate` methods may update the Applet's state. Any changes to the state will be made durable at the end of the execution, atomically.

Observe as well that the definition of the last method is not terminated by a `;`.

## Server-Side Bindings​

Server-side bindings act as a proxy between the host and the Applet. Server-side bindings generation may be performed manually, but this approach is time-consuming and error prone. Hence we use the `WIDL` compiler to generate the bindings.

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    widl generate counter.widl server rust  
    

A file named `bindings.rs` should have been created. It contains a skeleton for the Applet being developed, annotated with macros that will be expanded to the actual bindings during the compilation.

bindings.rs
    
    
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
        {  
            unimplemented!();  
        }  
      
      
        #[query]  
        async fn get_count(&self) -> usize {  
            unimplemented!();  
        }  
      
      
        #[mutate]  
        async fn increment(&mut self) {  
            unimplemented!();  
        }  
    }  
    

Observe that a trait with the name of the service, `Counter`, has been defined and that it defines three methods, two of which match the methods defined in the WIDL file, `get_count` and `increment`. The third method is a constructor for the service.

We'll fill in the logic for these methods later. For now, let's compile the contract.
    
    
    widl generate counter.widl server go  
    

You will notice that a file named `contract.go` got created. It contains a skeleton for the Applet being developed.

contract.go
    
    
    package contract  
      
    import (  
        "github.com/weilliptic-inc/wadk/go/weil_go/collections"  
        "github.com/weilliptic-inc/wadk/go/weil_go/types"  
    )  
      
    type CounterContractState struct {  
            // implement your contract state here!  
    }  
      
    func NewCounterContractState() (*CounterContractState, error) {  
            return &CounterContractState {}, nil  
    }  
      
    // query  
    func (obj *CounterContractState) GetCount() uint32 {  
        // TODO: implement this!  
    }  
      
    // mutate  
    func (obj *CounterContractState) Increment() {  
        // TODO: implement this!  
    }  
      
    

Observe that a struct with the name of the service, appended with `ContractState`, that is, `CounterContractState`, has been defined and that it implements three methods, two of which match the methods defined in the WIDL file, `get_count` and `increment` (except for the casing). The third method, `NewCounterContractState` is a constructor for the struct.

We'll fill in the logic for these methods later. For now, let's compile the contract.

You will notice that other files were also created:

  * `main.go`
  * `exports.go`
  * `types.go`



Files `main.go` and `exports.go` contain the actual bindings, which that will call into your implementation of the contract.

File `types.go` would contain any types defined in the specification, which are none in this example.

In the `assembly` folder create a new file `counter.ts`
    
    
    touch assembly/counter.ts  
    

Write a contract state class in this file
    
    
    @json  
    export class CounterContractState {  
      counter: u64  
      
      constructor() {  
        this.counter = 0  
      }  
      
      increment(): void {  
        this.counter += 1  
      }  
      
      getCount(): u64 {  
        return this.counter  
      }  
    }  
    

`@json` decorator above is important if you want to benefit from state serialization and deserialization provided by `json-as` library.

Next, you update `assembly/index.ts` to write your code that will interact with runtime as well as use `CounterContractState` methods.
    
    
    export function init(): void {  
    }  
      
    export function get_count(): void {  
    }  
      
    export function increment(): void {  
    }  
    

warning

Server-side binding generation is not yet supported. Copy the following file manually for now.
    
    
    widl generate counter.widl server cpp  
    

A file named `bindings.hpp` should have been created. It contain a skeleton for the Applet being developed.

bindings.hpp
    
    
    #include "external/nlohmann.hpp"  
      
    //define your counter state  
    class Counter {  
    public:  
        int value;  
      
        Counter(int initialValue) {}  
        Counter() : {}  
      
        int getCount() const {  
            return 0;  
        }  
      
        void increment() {  
            return;  
        }  
    };  
      
    // Serialization functions for Counter  
    inline void to_json(nlohmann::json& j, const Counter& c) {  
        return;  
    }  
    inline void from_json(const nlohmann::json& j, Counter& c) {  
        return;  
    }  
      
    

Notice that a class with the name of the service, `Counter`, has been defined; it will hold the state of the applet. The class defines two methods, which retrieve or manipulate the data inside it, `getCount` and `increment`; these match the methods in the specification. The other two methods are default and parameterized constructors for the service.

important

The **to_json** and **from_json** methods are extremely important here. They will be called to serialize and deserialize your contract state. Make sure the logic to convert your state to and from JSON is correctly implemented to not risk losing data while serializing and deserializing.

You will notice that another file, `main.cpp`, was also created. This file contains the stubs that will call into your implementation of the contracts

main.cpp
    
    
    #include "bindings.hpp"  
    #include "weilsdk/runtime.h"  
    #include "external/nlohmann.hpp"  
    #include "weilsdk/error.h"  
    #include <map>  
      
    extern "C" int __new(size_t len, unsigned char _id)  __attribute__((export_name("__new")));  
    extern "C" void init() __attribute__((export_name("init")));  
    extern "C" void get_value() __attribute__((export_name("get_value")));  
    extern "C" void increment() __attribute__((export_name("increment")));  
      
    Counter smart_contract_state;  
      
    extern "C" {  
      
        //export __new  
        int __new(size_t len, unsigned char _id) {  
                void* ptr = weilsdk::Runtime::allocate(len);  
                return reinterpret_cast<int>(ptr);  // Return the pointer as an integer to track the memory location  
        }  
      
        //export method_kind_data  
        void method_kind_data() {  
            std::map<std::string, std::string> method_kind_mapping;  
      
            method_kind_mapping["get_count"]= "query";  
            method_kind_mapping["increment"]= "mutate";  
      
            nlohmann::json json_object = method_kind_mapping;  
            std::string serialized_string = json_object.dump();  
            weilsdk::Runtime::setResult(serialized_string,0);  
        }  
      
        //export init  
        void init() {  
                nlohmann::json j;  
                to_json(j,smart_contract_state);  
                std::string serializedPayload = j.dump();  
       
                weilsdk::WeilValue wv;  
                wv.new_with_state_and_ok_value(serializedPayload, "Ok");  
                  
                weilsdk::Runtime::setStateAndResult(std::variant<weilsdk::WeilValue,weilsdk::WeilError> {wv});  
        }  
      
      
        void get_count() {  
      
                std::string serializedState = weilsdk::Runtime::state();  
                nlohmann::json j = nlohmann::json::parse(serializedState);  
                from_json(j,smart_contract_state);  
      
                int result = smart_contract_state.getCount();  
                  
                std::string serialized_result = std::to_string(result);  
                weilsdk::Runtime::setResult(serialized_result, 0);  
        }  
      
        void increment() {  
                std::string serializedState = weilsdk::Runtime::state();  
                nlohmann::json j = nlohmann::json::parse(serializedState);  
                from_json(j,smart_contract_state);  
                smart_contract_state.increment();  
      
                int incremented_count = smart_contract_state.getCount();  
                  
                nlohmann::json j1;  
                to_json(j1,smart_contract_state);  
                std::string serializedPayload = j1.dump();   
      
                weilsdk::WeilValue wv;  
                wv.new_with_state_and_ok_value(serializedPayload,std::to_string(incremented_count));  
                weilsdk::Runtime::setStateAndResult(std::variant<weilsdk::WeilValue,weilsdk::WeilError> {wv});  
        }  
    }  
      
    int main(){  
            return 0;  
    }  
      
    

danger

Make sure your handle errors in the implementations of functions "getCount" , "increment", etc. gracefully and deterministically. You should **not** use `try`, `catch` as they may lead to unexpected behaviour due to cross-language compatibility issues. We suggest using `std::pair<int, type>` as return types to check errors at intermediate steps.

## Compiling the contract​

  * Rust
  * Go
  * AssemblyScript
  * CPP



In order to compile the skeleton, first, copy `bindings.rs` its contents to file `src/lib.rs`.
    
    
    cat bindings.rs > src/lib.rc  
    rm bindings.rs  
    

Next, update the `Cargo.toml` file to be as follows and include needed dependencies.

Cargo.toml
    
    
    [package]  
    name = "counter"  
    version = "0.1.0"  
    edition = "2021"  
      
    [dependencies]  
    anyhow = "1.0.97"  
    serde = "1.0.219"  
    serde_json = "1.0.140"  
    weil_rs = { path = "../../../../adk/rust/weil_rs/" }  
    weil_macros = { path = "../../../../adk/rust/weil_rs/weil_macros" }  
    weil_contracts = { path = "../../../../adk/rust/weil_rs/weil_contracts" }  
      
    [lib]  
    crate-type = ["cdylib"]  
    

Finally compile the contract into a WASM module.
    
    
    cargo build --target wasm32-unknown-unknown --release  
    

You should now have file `target/wasm32-unknown-unknown/release/counter.wasm`, which is the body of the Applet. However, it is not useful in its current state and until we fill in the logic of the methods.

To compile, create folder `contract` and move `contract.go`, `exports.go` and `types.go` to the `contract` folder.
    
    
    mkdir contract  
    mv types.go exports.go contract.go contract  
    

Next download needed dependencies to `go.mod` by executing the following commands
    
    
    go get  
    

warning

While the repository is not publicly available, use the following `go.mod`

go.mod
    
    
    module main  
      
    go 1.22.2  
      
    require (  
            github.com/weilliptic-inc/jsonmap   
            github.com/weilliptic-inc/wadk/go/weil_go   
    )  
      
    replace github.com/weilliptic-inc/wadk/go/weil_go => /root/code/wadk/go/weil_go  
    

Finally try to compile the contract into a WASM module.
    
    
    mkdir target  
    mkdir target/wasi  
    tinygo build -target wasi -o target/wasi/counter_go.wasm  
    

You will see a few error messages, but don't worry. These errors will go away once we fill in the logic of the Applet methods.
    
    
    npm run asbuild  
    

Add the dependencies to `include` folder

Create a CMakeLists.txt file in the root of your project.
    
    
    touch CMakeLists.txt  
    

Here's a working file.

CMakeLists.txt
    
    
    cmake_minimum_required(VERSION 3.10)  
    project(counter)  
      
    # Specify C++ standard  
    set(CMAKE_CXX_STANDARD 17)  
    set(CMAKE_CXX_STANDARD_REQUIRED True)  
      
    set(LIBWEIL_DIR "${CMAKE_SOURCE_DIR}/lib")  
      
    include_directories(${CMAKE_SOURCE_DIR}/include)  
      
    add_executable(counter src/main.cpp)  
      
    target_link_libraries(counter "${LIBWEIL_DIR}/libweilsdk_static.a")  
      
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -s STANDALONE_WASM --no-entry -O3 -s ERROR_ON_UNDEFINED_SYMBOLS=0")  
    

Finally compile the contract into a WASM module.
    
    
    mkdir build  
    cd build  
    emcmake cmake ..  
    make  
    

You should now have file `build/counter.wasm`, which is the body of the Applet. However, it is not useful in its current state and until we fill in the logic of the methods.

## Filling in the logic​

Open contract file and go to the definition of struct/class with the state. This is struct/class, which will contain the Applet's state, is still empty. Modify it to include a single field with name `count` and integer type.

  * Rust
  * Go
  * AssemblyScript
  * CPP



src/lib.rs
    
    
    ...  
    #[derive(Serialize, Deserialize, WeilType)]  
    pub struct CounterContractState {  
        count: usize;  
    }  
    ...  
    

contract/contract.go
    
    
    ...  
    type CounterContractState struct {  
        Val uint32 `json:"inner"`  
    }  
    ...  
    

It is paramount that the fields of the state be named with an initial capital letter, so the state will be properly (de)serialized.

main.cpp
    
    
    class Counter {  
    public:  
        int value;  
    }  
    

Also fill the (de)serialization methods, which will convert the state to and from JSON.

main.cpp
    
    
    // Serialization functions for Counter  
    inline void to_json(nlohmann::json& j, const Counter& c) {  
        j = nlohmann::json{{"value", c.value}};  
    }  
    inline void from_json(const nlohmann::json& j, Counter& c) {  
        int val = j.at("value");  
        c.value = val;  
    }  
      
    

Now locate the constructor and change its contents as follows, so it sets the initial state of the Applet, when it is deployed.

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    ...  
        #[constructor]  
        fn new() -> Result<Self, String>  
        where  
            Self: Sized,  
        {  
            Ok(CounterContractState { counter: 0 })  
        }  
    ...  
    

contract/contract.go
    
    
    func NewCounterContractState() (*CounterContractState, error) {  
        return &CounterContractState {  
            Val: 0,  
        }, nil  
    }  
    
    
    
    export function init(): void {  
      const state = new CounterContractState();  
      const resultValue = "true";  
      
      const weilValue = WeilValue.newWithStateAndOkValue(state, resultValue);  
      const result = Result.Ok<WeilValue<CounterContractState,string>, WeilError>(weilValue);  
      Runtime.setStateAndResult(result);  
    }  
    
    
    
    Counter(int initialValue) : value(initialValue) {}  
    Counter() : value(0) {}  
    

Finally, locate methods to get the counter's value and to increment it, and add the corresponding logic :

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    ...  
        #[query]  
        async fn get_count(&self) -> usize {  
            self.count  
        }  
      
      
        #[mutate]  
        async fn increment(&mut self) {  
            self.count += 1  
        }  
    ...  
    

Observe that the `query` and `mutate` keywords used in the WIDL file became macros in the Rust definition and that the `query` method receives a reference to the Applet state, while the `mutate` method receives a mutable reference.
    
    
    // query  
    func (obj *CounterContractState) GetCount() uint32 {  
         return obj.Val  
    }  
      
    // mutate  
    func (obj *CounterContractState) Increment() {  
        s.Val++  
    }  
    

Observe that the `query` and `mutate` keywords used in the WIDL file are mere comments here, that only help you identify the methods that implement the WIDL service.
    
    
    export function get_count(): void {  
      const state = Runtime.state<CounterContractState>()  
      const result = state.getCount()  
      
      Runtime.setOkResult(result)  
    }  
      
    export function increment(): void {  
      const state = Runtime.state<CounterContractState>()  
      state.increment()  
      const resultValue = "ok"  
      
      const weilvalue = WeilValue.newWithStateAndOkValue(state, resultValue)  
      const result = Result.Ok<WeilValue<CounterContractState,string>, WeilError>(weilvalue);  
      
      Runtime.setStateAndResult(result);  
    }  
    
    
    
      
    int getCount() const {  
         return value;  
    }  
      
    void increment() {  
        value += 1;  
    }  
    

Now compile the Applet again to ensure that your file is correct.

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    cargo build --target wasm32-unknown-unknown --release  
    

warning

Edit `contract/contract.go` and remove the import clauses, so it looks exactly like the following.

contract/contract.go
    
    
    package contract  
      
    type CounterContractState struct {  
        Val uint32 `json:"inner"`  
    }  
      
    func NewCounterContractState() (*CounterContractState, error) {  
        return &CounterContractState {  
            Val: 0,  
        }, nil  
    }  
      
    // query  
    func (obj *CounterContractState) GetCount() uint32 {  
         return obj.Val  
    }  
      
    // mutate  
    func (obj *CounterContractState) Increment() {  
        obj.Val++  
    }  
    

Edit `contract/types.go` and remove the import clauses, so it looks exactly like the following.

contract/types.go
    
    
    package contract  
    
    
    
    tinygo build -target wasi -o target/wasi/counter_go.wasm  
    
    
    
    npx run asbuld  
    
    
    
    make  
    

## Next steps​

Congratulations! You should have your Applet ready to be deployed, which you can do by following the tutorial [Deploy and use a WeilChain Applet](/docs/tutorials/deploy_applets).

  * Preparations
  * Contract Specification
  * Server-Side Bindings
  * Compiling the contract
  * Filling in the logic
  * Next steps



Copyright © 2026 Weilliptic, Inc.

                
 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

On this page

# Deploy and Use an Applet via CLI

You two options to deploy applets in WeilChain. Here we explain how to do so using the CLI and in we show how to [deploy it directly the Explorer](/docs/tutorials/view_applets)

Once an Applet is deployed to the WeilChain, its methods may invoked by any user of the chain, using the CLI, a DApp, in some cases, the Wallet, or even through the [Explorer](/docs/tutorials/view_applets)

In this tutorial we'll see how to upload an Applet to the chain and invoke its methods using the Weilliptic CLI.

## Preparations​

This tutorial assumes that

  * You have access to the testnet WeilChain.
  * You have completed the [Basic Applet Creation](/docs/tutorials/counter_applet) tutorial, as we will use the Applet created there.
  * You have installed the Weilliptic CLI binary.



## Start the cli​

Assuming that the Weilliptic CLI binary is in the `PATH`, execute the following:
    
    
    WC_PATH=~/.weilliptic WC_PRIVATE_KEY=~/.weilliptic cli  
    

The Weilliptic CLI is a very simple Wallet and to use it you will need a private key. If you already have one setup, it will be automatically loaded. Otherwise, to generate a new one, use the following command.
    
    
    wallet setup -g  
    

Simply hit ⮐ or execute command `help` to see a list of all commands available in the CLI.
    
    
    Weilliptic$$$>  
      quit                  Quit the Weilliptic CLI  
      connect               Connect.  
      deploy                Deploy Weil Applet.  
      execute               Execute Weil Applet.  
      get_contract_details  Get WIDL Details.  
      get_txn_status        Retrieve Transaction Status given a ticket.  
      list_weilpods         List all the commands in Weilliptic  
      wallet                Sets up a Wallet  
      help                  Print this message or the help of the given subcommand(s)  
    

To learn about each command, execute `help <COMMAND>`, e.g.
    
    
    Weilliptic$$$> help help  
    Print this message or the help of the given subcommand(s)  
      
    Usage: help [COMMAND]...  
      
    Arguments:  
      [COMMAND]...  Print help for the subcommand(s)  
    

The first command in the list, `connect`, is used to connect to the Sentinel node, where the CLI will learn about other nodes. Connecting is a requirement for most other commands.
    
    
    Weilliptic$$$> help connect  
    Connect.  
      
    Usage: connect --host <host> --port <port>  
      
    Options:  
      -h, --host <host>  Hostname to which we wish to connect.  
      -p, --port <port>  Port number to which we wish to connect.  
      -H, --help-all     Print help information  
    

The `host` refers to the Sentinel node, whose name or address will be published based on whether it is connected to the test-net or main-net. For example, for the mainnet, use

Execute command `connect -h <sentinel-node>` The following response should be seen.
    
    
    {"message":"Connected successfully to <sentinel-node>.","status":"Ok"}  
    

## Deploying Smart Contracts​

Once successfully connected to the Sentinel host, one can deploy their Applet using the deploy command by providing the contract body (path to the .wasm file) and its definition (path to the .widl file).

To deploy the [Counter Applet](/docs/tutorials/counter_applet), execute the following in the CLI:

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    deploy --file-path path-to-project/target/wasm32-unknown-unknown/release/counter.wasm --widl-file path-to-widl-file   
    
    
    
    deploy --file-path path-to-project/target/wasi/counter_go.wasm --widl-file path-to-widl-file   
    
    
    
    deploy --file-path path-to-project/target/wasi/counter.wasm --widl-file path-to-widl-file  
    
    
    
    deploy --file-path path-to-project/build/counter.wasm --widl-file path-to-widl-file  
    

A response similar to the following should be returned, here shortened and pretty-printed to facilitate the reading:
    
    
    {  
      "batch_author": "<pod-id>",  
      "batch_id": "7618487ecb89ea5b11f0bb9fa5878e0ed0210106c8558796c826f62fd0b81674",  
      "block_height": 330,  
      "contract_address": "7b2...27d",  
      "creation_time": "2025-03-17T05:39:32Z",  
      "status": "Finalized",  
      "tx_idx": 0,  
      "txn_result": "{\"Ok\":\"null\"}",  
      "txn_ticket": "000000029eb1827e922c39afbbeab36a35d3750afe4d49505d91a67d24f2b05b46f193e6"  
    }  
    

Note the `contract_address` field. You will need to use its value in the next call, which retrieves the value of the counter using the `execute` command.
    
    
    execute --name 7b2...27d  --method get_count  
    

note

The address is given in the `--name` parameter, in its full form, not shortened as shown above.

The result should be the following
    
    
    {  
        "batch_author":"<pod-id>",  
        "batch_id":"",  
        "block_height":0,  
        "creation_time":"",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"0\"}"  
    }  
    

The field `txn_result` indicates that the value of the counter is 0. Let's increment it using the `increment` method:
    
    
    execute -n 7b2...27d --method increment  
    
    
    
    {  
        "batch_author":"<pod-id>",  
        "batch_id":"4f50b5754765124e077e352b8f27fec83f349a88581c91cb7e2c2469b8796fe8",  
        "block_height":11739,  
        "creation_time":"2024-09-30T20:48:28Z",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"null\"}"  
    }  
    

followed by another call to `get_count`
    
    
    execute -n 7b2...27d --method get_count  
    
    
    
    {  
        "batch_author":"<pod-id>",  
        "batch_id":"",  
        "block_height":0,  
        "creation_time":"",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"1\"}"  
    }  
    

Observe that this time `txn_result` has value `1`.

## Next steps​

Congratulations! You should have deployed an Applet and used it. Next you should modify the Applet to include a method that receives parameters, as in [Pass parameters to Applets methods](/docs/tutorials/applet_with_parameters).

  * Preparations
  * Start the cli
  * Deploying Smart Contracts
  * Next steps



Copyright © 2026 Weilliptic, Inc.

                
 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

On this page

# Define and Execute Methods with Parameters via the CLI

Useful Applets will invariably have methods that accept arguments, such as "to whom some token should be transferred?", "which domain to register?", and "how many tokens to transfer?".

In this tutorial we'll see how to define methods with parameters and how to pass arguments when invoking such methods using the Weilliptic CLI.

## Preparations​

This tutorial assumes that

  * you have access to the WeilChain.
  * you have completed the [Basic Weil Applet Creation](/docs/tutorials/counter_applet) tutorial, as we will improve the Applet created there, and have a shell opened at the root of that project;
  * you have completed the [Weil Applet Deployment using cli](/docs/tutorials/deploy_applets) tutorial, as we will use cli in a similar way.



## Updating the Counter Applet​

We will add a method to set the counter to an arbitrary value, bigger or equal to 0. To add the method to the Counter Applet, edit the `counter.widl` file to match the following.

counter.widl
    
    
    interface Counter {  
        query func get_count() -> uint;  
        mutate func increment();  
        mutate func set_value(val: uint)  
    }  
    

Observe that since the method will update the counter, it is defined as `mutate`.

Observe also that the only the last method definition is not ended by `;`.

Now regenerate the server-side bindings file using the WIDL command:

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    widl generate counter.widl server rust  
    

The updated `bindings.rs` should include the new method `set_value`, in the `Counter` trait, and its skeleton.

bindings.rs
    
    
    use serde::{Deserialize, Serialize};  
    use weil_macros::{constructor, mutate, query, smart_contract, WeilType};  
      
      
    pub trait Counter {  
        fn new() -> Result<Self, String>  
        where  
            Self: Sized;  
        async fn get_count(&self) -> usize;  
        async fn increment(&mut self);  
        async fn set_value(&mut self, val: usize);  
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
        {  
            unimplemented!();  
        }  
      
        #[query]  
        async fn get_count(&self) -> usize {  
            unimplemented!();  
        }  
      
        #[mutate]  
        async fn increment(&mut self) {  
            unimplemented!();  
        }  
      
        #[mutate]  
        async fn set_value(&mut self, val: usize) {  
            unimplemented!();  
        }  
    }  
    
    
    
    widl generate counter.widl server go  
    

The updated `contract.go` should include the new method, `setValue` and its skeleton.

contract.go
    
    
    ...  
      
    // mutate  
    func (s *Counter) SetValue(uint) error {  
            return nil  
    }  
    

warning

Not supported yet.  
Copy the following file manually for now.
    
    
    widl generate counter.widl server cpp  
    

The updated `bindings.hpp` should include the new method, `setValue` and its skeleton.

bindings.hpp
    
    
    #include "external/nlohmann.hpp"  
      
    //define your counter state  
    class Counter {  
    public:  
        int value;  
      
        Counter(int initialValue) {}  
        Counter() : {}  
      
        int getCount() const {  
            return 0;  
        }  
      
        void increment() {  
            return;  
        }  
      
        void setValue(){  
            return;  
        }  
    };  
      
    // Serialization functions for Counter  
    inline void to_json(nlohmann::json& j, const Counter& c) {  
        return;  
    }  
    inline void from_json(const nlohmann::json& j, Counter& c) {  
        return;  
    }  
      
    

Next, copy the new pieces of code to the contract implementation to the corresponding locations and fill in the logic. You need to do this because the compiler only updates the skeleton, not the implementation of the Applet.

  * Rust
  * Go
  * AssemblyScript
  * CPP



src/lib.rs
    
    
    ...  
    pub trait Counter {  
    ...  
        async fn set_value(&mut self, val: usize);  
    }  
    ...  
        #[mutate]  
        fn set_value(&mut self, val: usize) {  
            self.count = val  
        }  
    ...  
    

contract/contract.go
    
    
    ...  
        // mutate  
        func (obj *CounterContractState) SetValue(val uint32) {  
            obj.Val = val  
        }  
    ...  
    
    
    
      setValue(value: u64): void{  
        this.counter = value  
      }  
    

counter.ts
    
    
    @json  
    export class setValueArgs {  
      val: u64;  
      
      constructor() {  
        this.val = 0  
      }  
    }  
    

index.ts
    
    
      
    ...  
    export function set_value(): void {  
      const stateAndArgs = Runtime.stateAndArgs<CounterContractState, setValueArgs>()  
        
      const state = (stateAndArgs.elements[0] as JSONWrapper<CounterContractState>).inner;  
      const args = (stateAndArgs.elements[1] as JSONWrapper<setValueArgs>).inner;  
      
      state.setValue(args.val)  
      
      const resultValue = "ok"  
      const weilvalue = WeilValue.newWithStateAndOkValue(state, resultValue)  
      const result = Result.Ok<WeilValue<CounterContractState,string>, WeilError>(weilvalue);  
      
      Runtime.setStateAndResult(result);  
    }  
      
    

src/main.cpp
    
    
    ...  
        // mutate  
        void setValue(int newValue) {  
            value = newValue;  
        }  
    ...  
    

warning

Server side binding generation isn't supported yet.

You need to update `method_kind_data` and add the and `set_value()` manually.

main.cpp
    
    
      
    ...  
    //export method_kind_data  
    void method_kind_data() {  
        std::map<std::string, std::string> method_kind_mapping;  
      
        method_kind_mapping["get_count"]= "query";  
        method_kind_mapping["increment"]= "mutate";  
        method_kind_mapping["set_value"]= "mutate";  
      
        nlohmann::json json_object = method_kind_mapping;  
        std::string serialized_string = json_object.dump();  
        weilsdk::Runtime::setResult(serialized_string,0);  
    }  
      
    struct setValueArgs{  
        int val;  
    };  
      
    inline void to_json(nlohmann::json& j, const setValueArgs& s) {  
        j = nlohmann::json{{"val", s.val}};  
    }  
    inline void from_json(const nlohmann::json& j, setValueArgs& s) {  
        int _val = j.at("val");  
        c.val = _val;  
    }  
      
    ...  
    void set_value() {  
      
            std::pair<std::string,std::string> serializedStateAndArgs = weilsdk::Runtime::stateAndArgs();  
            weilsdk::StateArgsValue sav;  
      
            nlohmann::json stateJson = nlohmann::json::parse(serializedStateAndArgs.first);  
            from_json(stateJson,smart_contract_state);  
      
            nlohmann::json argsJson = nlohmann::json::parse(serializedStateAndArgs.second);  
            if(argsJson.is_discarded()){  
                weilsdk::MethodError me = weilsdk::MethodError("set_value", "invalid_args");  
                std::string err = weilsdk::WeilError::MethodArgumentDeserializationError(me);  
                weilsdk::Runtime::setStateAndResult(std::variant<weilsdk::WeilValue,weilsdk::WeilError> {err});  
                return;  
            }  
            setValueArgs s;  
            from_json(argsJson,s);  
      
            smart_contract_state.setValue(s.val);  
      
            nlohmann::json j2;  
            to_json(j2,smart_contract_state);  
            std::string serializedSmartContractState = j2.dump();   
      
            weilsdk::WeilValue wv;  
            wv.new_with_state_and_ok_value(serializedSmartContractState, "Ok");  
            weilsdk::Runtime::setStateAndResult(std::variant<weilsdk::WeilValue,weilsdk::WeilError> {wv});  
    }  
    

## Execute the CLI​

Assuming that the Weilliptic CLI binary is in the `PATH`, execute the following:
    
    
    WC_PATH=~/.weilliptic WC_PRIVATE_KEY=~/.weilliptic cli  
    

Execute command `connect -h <sentinel-node>` The following response should be seen.
    
    
    {"message":"Connected successfully to <sentinel-node>.","status":"Ok"}  
    

## Deploying the Applet​

To deploy the updated Counter Applet, execute the following in the CLI:
    
    
    deploy --file-path /root/code/counter/target/wasm32-unknown-unknown/release/counter.wasm --widl-file /root/code/counter/counter.widl  
    

The contract is deployed an new `contract_address` is returned, different from any previously deployed.
    
    
    {  
        "batch_author":"pod-75a62520-0.weilliptic.default.svc.cluster.local-8000",  
        "batch_id":"21d39243c5f1abdf3b530963ba5d0209afa66df9f6dcf902583fe0f6e4eb7e3b",  
        "block_height":953,  
        "contract_address":"7b226...227d",  
        "creation_time":"2024-09-30T20:30:26Z",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"null\"}",  
        "txn_ticket":{"Ok":"7b226...227d"}  
    }  
    

Run the `execute` command to see the current counter value
    
    
    execute --name  7b226...227d --method get_count  
    

The result should be the following
    
    
    {  
        "batch_author":"",  
        "batch_id":"",  
        "block_height":0,  
        "creation_time":"",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"0\"}"  
    }  
    

Let's set the value of the counter to 10 using the `set_value` method:
    
    
    execute -n 7b226...227d --method set_value --method-args '{"val": 10}'  
    
    
    
    {  
        "batch_author":"pod-7bad3bac-0.weilliptic.default.svc.cluster.local-8000",  
        "batch_id":"f8d22d5400f5b00e6fb9d934ce20b0a64ac10eecdfbe173ef63ffbfa0ed96789",  
        "block_height":39486,  
        "creation_time":"2024-09-30T21:54:06Z",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"null\"}"  
    }  
    

followed by another call to `get_count`
    
    
    execute -n  7b226...227d --method get_count  
    
    
    
    {  
        "batch_author":"",  
        "batch_id":"",  
        "block_height":0,  
        "creation_time":"",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"10\"}"  
    }  
    

Observe that this time `txn_result` has value `10`.

## Next steps​

Congratulations! You have seen how to define and deploy simple Applets. Next you should understand how to [Make Cross-Contract Calls](/docs/tutorials/cross-contract).

  * Preparations
  * Updating the Counter Applet
  * Execute the CLI
  * Deploying the Applet
  * Next steps



Copyright © 2026 Weilliptic, Inc.

                
