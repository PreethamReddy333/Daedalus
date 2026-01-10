 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

On this page

# Make Cross-Contract Calls

Applets are composable, meaning that any deployed Applet may invoke methods on any other deployed Applet, in a cross-contract call. In this tutorial we'll see how to implement such cross contract calls.

note

Composability of contracts is true even for contracts implemented in different languages. To learn about how specificities of languages reflect on the WIDL spec, read the [Note on WIDL and Language Ergonomics](/docs/explainers/language_ergonomics).

important

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

This tutorial assumes that

  * you have completed the [Deploy and Use an Applet via CLI](/docs/tutorials/deploy_applets), as we will develop an Applet that makes cross-contract calls to the Counter applet you developed and deployed earlier.



To start, create a new project:

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    cargo new cross_counter_rust --lib  
    cd cross_counter_rust  
    
    
    
    mkdir cross_counter_go  
    cd cross_counter_go  
    go mod init main  
    go mod tidy  
    mkdir contract  
    

warning

TODO
    
    
    mkdir crossCounterCPP  
    cd crossCounterCPP  
    

## Definition​

Create file `cross_counter.widl`, with the following contents.

cross_counter.widl
    
    
    interface CrossCounter {  
        query func fetch_counter_from(contract_id: string) -> result<uint,string>;  
        mutate func increment_counter_of(contract_id: string) -> result<(),string>  
    }  
    

## Bindings generation​

Next, use the `WIDL` compiler to generate server-side bindings.

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    widl generate cross_counter.widl server rust  
    

A file named `bindings.rs` should have been created. It contains a skeleton for the Applet being developed.

bindings.rs
    
    
      
    use serde::{Deserialize, Serialize};  
    use weil_macros::{constructor, mutate, query, smart_contract, WeilType};  
      
      
    pub trait CrossCounter {  
        fn new() -> Result<Self, String>  
        where  
            Self: Sized;  
        async fn fetch_counter_from(&self, contract_id: String) -> Result<usize, String>;  
        async fn increment_counter_of(&mut self, contract_id: String) -> Result<(), String>;  
    }  
      
    #[derive(Serialize, Deserialize, WeilType)]  
    pub struct CrossCounterContractState {  
        // define your contract state here!  
    }  
      
    #[smart_contract]  
    impl CrossCounter for CrossCounterContractState {  
        #[constructor]  
        fn new() -> Result<Self, String>  
        where  
            Self: Sized,  
        {  
            unimplemented!();  
        }  
      
      
        #[query]  
        async fn fetch_counter_from(&self, contract_id: String) -> Result<usize, String> {  
            unimplemented!();  
        }  
      
      
        #[mutate]  
        async fn increment_counter_of(&mut self, contract_id: String) -> Result<(), String> {  
            unimplemented!();  
        }  
      
    }  
    
    
    
    widl generate cross_counter.widl server go  
    

Several files should have been generated for you, including `contract.go`, with the skeleton for the Applet.

contract.go
    
    
    package contract  
      
    import (  
        "github.com/weilliptic-inc/wadk/go/weil_go/collections"  
        "github.com/weilliptic-inc/wadk/go/weil_go/types"  
    )  
      
    type CrossCounterContractState struct {  
        // implement your contract state here!  
    }  
      
    func NewCrossCounterContractState() (*CrossCounterContractState, error) {  
        return &CrossCounterContractState {}, nil  
    }  
      
    // query  
    func (obj *CrossCounterContractState) FetchCounterFrom(contractId string) uint32 {  
        // TODO: implement this!  
    }  
      
    // mutate  
    func (obj *CrossCounterContractState) IncrementCounterOf(contractId string) {  
        // TODO: implement this!  
    }  
    

The other files are:

  * `main.go`
  * `exports.go`
  * `types.go`



Files `main.go` and `exports.go` contain the actual bindings, which that will call into your implementation of the contract.

File `types.go` would contain any types defined in the specification, which are none in this example.

warning

TODO
    
    
    widl generate cross_counter.widl server cpp  
    

warning

Server-side bindings generation is underway. For now, copy the following file manually to your project folder.

A file named `bindings.cpp` should have been created. It contains a skeleton for the Applet being developed.

bindings.cpp
    
    
    struct CrossCounter {  
    };  
      
    void newCrossCounter() {  
    	CrossCounter crossCounter;  
    }  
      
    // query  
    void fetchCounterFrom(std::string contractId){  
    	return;  
    }  
      
    // mutate  
    void IncrementCounterOf(std::string contractId){  
        return;  
    }  
    

## Compiling the contract​

Let's compile the generated skeleton to ensure that everything is correct. In the process, we'll move it to its final location.

  * Rust
  * Go
  * AssemblyScript
  * CPP



Move the skeleton contents to file `src/lib.rs`.
    
    
    cat bindings.rs > src/lib.rs  
    rm bindings.rs  
    

Update the `Cargo.toml` file to be as follows and include needed dependencies.
    
    
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
    

Move the skeleton contents to file `contract/main.go`.
    
    
    mkdir contract  
    mv types.go exports.go contract.go contract  
    

Next download needed dependencies to `go.mod` by executing the following commands
    
    
    go get  
    

warning

While the repository is not public available, use the following `go.mod`
    
    
    module main  
      
    go 1.22.2  
      
    require (  
            github.com/weilliptic-inc/jsonmap   
            github.com/weilliptic-inc/wadk/go/weil_go   
    )  
      
    replace github.com/weilliptic-inc/wadk/go/weil_go => /root/code/wadk/go/weil_go  
    

Finally compile the contract into a WASM module.
    
    
    mkdir target  
    mkdir target/wasi  
    tinygo build -target wasi -o target/wasi/cross_counter_go.wasm  
    

The compilation will fail, but don't worry. Once we have filled in the logic in the contract, all will work.

warning

TODO

Move the skeleton contents to file `contract/main.cpp`.
    
    
    mkdir contract  
    mv bindings.cpp contract/main.cpp  
    

Next include the dependencies to `main.cpp`

main.cpp
    
    
    #include "external/nlohmann.hpp"  
    #include "weilsdk/runtime.h"  
    #include "weilsdk/error.h"  
    

Create a CMakeLists.txt in the root of your project.
    
    
    touch CMakeLists.txt  
    

Fill it as follows.

CMakeLists.txt
    
    
    cmake_minimum_required(VERSION 3.10)  
    project(crossContract)  
      
    # Specify C++ standard  
    set(CMAKE_CXX_STANDARD 17)  
    set(CMAKE_CXX_STANDARD_REQUIRED True)  
      
    set(LIBWEIL_DIR "${CMAKE_SOURCE_DIR}/lib")  
      
    include_directories(${CMAKE_SOURCE_DIR}/include)  
      
    add_executable(crossContract contract/main.cpp)  
      
    target_link_libraries(crossContract "${LIBWEIL_DIR}/libweilsdk_static.a")  
      
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -s STANDALONE_WASM --no-entry -O3 -s ERROR_ON_UNDEFINED_SYMBOLS=0")  
    

Finally compile the contract into a WASM module.
    
    
    mkdir build  
    cd build  
    emcmake cmake ..  
    make  
    

Before this contract becomes useful, we need to fill in the logic of the methods. But to fill in the logic, we must first see how to make cross-contract calls.

## Cross-contract bindings​

Any contract can interact with other contracts, querying information and executing functions on them, using the "call contract" function.

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    fn call_contract(contract_id: String, method_name: String, method_args: Option<String>) -> anyhow::Result<R>  
    

The arguments of the function are:

  * `contract_id`: is contract address of the contract we want to call into
  * `method_name`: is the name of the method which we want to call
  * `method_args`: are the JSON-serialized arguments for the above method



One can either manually call this method using `Runtime::call_contract` or again use the WIDL compiler to generate this cross-contract call bindings. We'll do the latter one; run the following command, assuming that the counter project is in the same folder as the cross_counter one:
    
    
    widl generate ../counter/counter.widl cross-contract rust  
    

The generated cross-contract `bindings.rs` would look like the following:

bindings.rs
    
    
    use serde::{Deserialize, Serialize};  
    use anyhow::Result;  
    use weil_rs::runtime::Runtime;  
      
      
    pub struct Counter {  
        contract_id: String,  
    }  
      
    impl Counter {  
        pub fn new(contract_id: String) -> Self {  
            Counter {  
                contract_id,  
            }  
        }  
    }  
      
    impl Counter {  
        pub async fn get_count(&self) -> Result<usize> {  
            let serialized_args = None;  
      
            let resp = Runtime::call_contract::<usize>(  
                self.contract_id.clone(),  
                "get_count".to_string(),  
                serialized_args,  
            )?;  
      
            Ok(resp)  
        }  
      
        pub async fn increment(&self) -> Result<()> {  
            let serialized_args = None;  
      
            let resp = Runtime::call_contract::<()>(  
                self.contract_id.clone(),  
                "increment".to_string(),  
                serialized_args,  
            )?;  
      
            Ok(resp)  
        }  
      
        pub async fn set_value(&self, val: usize) -> Result<()> {  
      
            #[derive(Debug, Serialize)]  
            struct set_valueArgs {  
                val: usize,  
            }  
      
            let serialized_args = Some(serde_json::to_string(&set_valueArgs { val })?);  
      
            let resp = Runtime::call_contract::<()>(  
                self.contract_id.clone(),  
                "set_value".to_string(),  
                serialized_args,  
            )?;  
      
            Ok(resp)  
        }  
    }  
    

Now copy the `binding.rs` file to the src folder.
    
    
    cp bindings.rs src/counter.rs  
    
    
    
    func CallContract[T any](contractId string, methodName string, methodArgs string) (*T, error)  
    

The arguments of the function are:

  * `contractId`: is contract address of the contract we want to call into
  * `methodName`: is the name of the method which we want to call
  * `methodArgs`: are the JSON-serialized arguments for the above method



One can either manually call this method using `runtime.CallContract` or again use the WIDL compiler to generate this cross-contract call bindings. We'll do the latter one; run the following command, assuming that the `counter` project is in the same folder as the `cross_counter_go` one:
    
    
    widl generate ../counter_go/counter.widl cross-contract go  
    

warning

Cross-contract bindings generation is underway. For now, copy the following file manually to your project folder.

The generated cross-contract `counter.go` should look like the following. Observe that it imports `runtime` and uses it to make call `CallContract()`.

counter.go
    
    
    package contract  
      
    import (  
        "github.com/weilliptic-inc/wadk/go/weil_go/runtime"  
    )  
      
    type Counter struct {  
        ContractId string `json:"contract_id"`  
    }  
      
    func newCounter(contractId string) *Counter {  
        return &Counter{  
            contractId,  
        }  
    }  
      
    func (c *Counter) getCount() (*uint32, error) {  
        resp, err := runtime.CallContract[uint32](c.ContractId, "get_count", "")  
      
        if err != nil {  
            return nil, err  
        } else {  
            return resp, nil  
        }  
    }  
      
    func (c *Counter) increment() error {  
        _, err := runtime.CallContract[interface{}](c.ContractId, "increment", "")  
      
        if err != nil {  
            return err  
        } else {  
            return nil  
        }  
    }  
    

Now move the `counter.go` file to the `contract` folder.
    
    
    mv counter.go contract/counter.go  
    

warning

TODO
    
    
    std::pair<int,std::string> callContract(std::string contract_id,std::string method_name, std::string method_args)  
    

The arguments of the function are:

  * `contract_id`: is contract address of the contract we want to call into
  * `method_name`: is the name of the method which we want to call
  * `method_args`: are the JSON-serialized arguments for the above method



One can either manually call this method using `Runtime::call_contract` or again use the WIDL compiler to generate this cross-contract call bindings. We'll do the latter one; run the following command, assuming that the counter project is in the same folder as the cross_counter one:
    
    
    widl generate ../counter/counter.widl crossContract cpp  
    

warning

Cross-contract bindings generation is underway. For now, copy the following file manually to your project folder.

The generated cross-contract `bindings.cpp` would look like the following:

bindings.cpp
    
    
    #include "external/nlohmann.hpp"  
    #include "weilsdk/runtime.h"  
    #include <string>  
      
      
    struct setValueArgs{  
        int val;  
    };  
      
    inline void to_json(nlohmann::json& j, const setValueArgs& s) {  
        j = nlohmann::json{{"val", s.val}};  
    }  
    inline void from_json(const nlohmann::json& j, setValueArgs& s) {  
        int _val = j.at("val");  
        s.val = _val;  
    }  
      
      
    class Counter {  
        std::string contractId;  
      
        Counter(): contract_id("") {};  
        Counter(std::string _contractId): contractId(_contractId) {};  
      
        std::pair<int,int> get_count(){  
            std::string serializedArgs = "";  
      
            std::pair<int,std::string> resp = weilsdk::Runtime::callContract(  
                contract_id,  
                "getCount",  
                serializedArgs  
            );  
      
            if(resp.first){  
                return {1,-1};  
            }  
            else{  
                return {0,std::stoi(resp.second)};  
            }  
        }  
      
      
        std::pair<int,std::string> increment(){  
      
            std::string serializedArgs = "";  
      
            std::pair<int,std::string> resp = weilsdk::Runtime::callContract(  
                contract_id,  
                "increment",  
                serializedArgs  
            );  
            return resp;  
        }  
    };  
      
    // Serialization functions for Counter  
    /*  
    Note:  
    We define the to_json, from_json function as "inline" so as to guard against naming collisions.  
    */  
      
    inline void to_json(nlohmann::json& j, const Counter& c) {  
        j = nlohmann::json{{"contract_id", c.contract_id}};  
    }  
    inline void from_json(const nlohmann::json& j, Counter& c) {  
        std::string contractId = j.at("contract_id");  
        c.contract_id = contractId;  
    }  
    

Now copy the `bindings.cpp` file to the src folder.
    
    
    cp bindings.hpp src/counter.hpp  
    

## Filling in the logic​

  * Rust
  * Go
  * AssemblyScript
  * CPP



The `Counter` type in `src/counter.rs` acts like a proxy for a `Counter` Applet instance. The exact instance will be determined by the `contract_id` parameter passed to the implementation of `CrossCounter::fetch_from_counter` and `CrossCounter::set_counter_of`. To do so, implement methods in `CrossCounter` in `src/lib.rs` as follows:
    
    
    mod counter;  
      
    use serde::{Deserialize, Serialize};  
    use weil_macros::{constructor, mutate, query, smart_contract, WeilType};  
    use crate::counter::Counter;  
      
    ...  
      
    #[smart_contract]  
    impl CrossCounter for CrossCounterContractState {  
        #[constructor]  
        fn new() -> Result<Self, String>  
        where  
            Self: Sized,  
        {  
            Ok(CrossCounterContractState {})  
        }  
      
        #[query]  
        async fn fetch_counter_from(&self, contract_id: String) -> Result<usize,String> {  
            let counter = Counter::new(contract_id);  
            counter.get_count().map_err(|err| err.to_string())  
        }  
      
        #[mutate]  
        async fn increment_counter_of(&mut self, contract_id: String) -> Result<(),String> {  
            let counter = Counter::new(contract_id);  
            counter.increment().await.map_err(|err| err.to_string())  
        }  
    }  
    

The `Counter` type in `package/counter.go` acts like a proxy for a `Counter` Applet instance. The exact instance will be determined by the `contractId` parameter passed to the implementation of `CrossCounter.FetchCounterFrom` and `CrossCounter.IncrementCounterOf`. To do so, implement methods for `CrossCounter` in `package/main.go` as follows:
    
    
    package contract  
      
    type CrossCounterContractState struct {  
    }  
      
    func NewCrossCounterContractState() (*CrossCounterContractState, error) {  
        return &CrossCounterContractState {}, nil  
    }  
      
    // query  
    func (obj *CrossCounterContractState) FetchCounterFrom(contractId string) uint32 {  
        counter := newCounter(contractId)  
        c, err := counter.getCount()  
        if err != nil {  
            return 0  
        }  
      
        return *c  
    }  
      
    // mutate  
    func (obj *CrossCounterContractState) IncrementCounterOf(contractId string) {  
        counter := newCounter(contractId)  
        counter.increment()  
    }  
    

warning

TODO

The `Counter` type in `src/counter.hpp` acts like a proxy for a `Counter` Applet instance. The exact instance will be determined by the `contract_id` parameter passed to the implementation of `CrossCounter::fetchCounterFrom` and `CrossCounter::incrementCounterOf`. To do so, implement methods in `CrossCounter` in `src/main.cpp` as follows:
    
    
    #include "counter.hpp"  
    #include "external/nlohammn.hpp"  
    #include <string>  
      
    class CrossCounterContractState{  
        CrossCounterContractState(){}  
      
        void fetchCounterFrom (std::string counterId){  
            Counter counterstate(contractId);  
            std::pair<int,int> res= counter.get_count();  
            if(res.first){  
                //some error  
            }  
            else std::cout<<res.second<<std::endl;  
        }  
      
        void IncrementCounterOf (std::string contractId){  
            Counter counter(contractId);  
            counter.increment();  
        }  
    };  
    

## Deploy Contracts​

Once the contract is compiled and assuming your `Counter` contract is also compiled, let's deploy them.

First, start cli.
    
    
    WC_PATH=~/.weilliptic   
    WC_PRIVATE_KEY=~/.weilliptic cli  
    

Execute command `connect -h <sentinel-node>` The following response should be seen.
    
    
    {"message":"Connected successfully to <sentinel-node>.","status":"Ok"}  
    

Deploy the `Counter` applet and note the `contract_address` in the response:

  * Rust
  * Go
  * AssemblyScript
  * CPP


    
    
    deploy --file-path path-to-counter-project/target/wasm32-unknown-unknown/release/counter.wasm --widl-file path-to-counter-widl-file   
    

And deploy the `CrossCounter` applet and note the `contract_address` in the response:
    
    
    deploy --file-path path-to-cross-counter-project/target/wasm32-unknown-unknown/release/cross_counter.wasm --widl-file path-to-cross_counter-widl-file   
    
    
    
    deploy --file-path path-to-counter-project/target/wasi/counter_go.wasm --widl-file path-to-counter-widl-file   
    

And deploy the `CrossCounter` applet and note the `contract_address` in the response:
    
    
    deploy --file-path path-to-cross_counter-project/target/wasi/counter_go.wasm --widl-file path-to-cross_counter-widl-file   
    
    
    
    deploy --file-path path-to-counter-project/target/wasi/counter.wasm --widl-file path-to-counter-widl-file  
    

And deploy the `CrossCounter` applet and note the `contract_address` in the response:
    
    
    deploy --file-path path-to-cross_counter-project/target/wasi/counter.wasm --widl-file path-to-cross_counter-widl-file  
    
    
    
    deploy --file-path path-to-counter-project/build/counter.wasm --widl-file path-to-counter-widl-file  
    

And deploy the `CrossCounter` applet and note the `contract_address` in the response:
    
    
    deploy --file-path path-to-cross_counter-project/build/counter.wasm --widl-file path-to-cross_counter-widl-file  
    

Now let's retrieve the counter of `Counter` Applet using the `CrossCounter` contract, using `fetch_counter_from` as in the following example.
    
    
    execute --name 6d1...37b --method fetch_counter_from --method-args '{"contract_id":"7b2...27d"}'  
    

note

In `--name` we pass the `contract_address` of `CrossCounter` and in `--method-args` we pass in the `contract_address` of `Counter`.

note

The name of the method is as defined in the WIDL, that is, in snake-case, even if the implementation had it in Camel-case

The result should inform that the counter has value 0.
    
    
    {  
        "batch_author":"",  
        "batch_id":"",  
        "block_height":0,  
        "creation_time":"",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"0\"}"  
    }  
    

Next, let's increment the counter through `CrossCounter`.
    
    
    execute --name 6d1...37b --method increment_counter_of --method-args '{"contract_id":"7b2...27d"}'  
    

The result should show the absence of errors
    
    
    {  
        "batch_author":"<pod-id>",  
        "batch_id":"853acf02df133929ff364115f123bf0730b993e58f82822a32b19c09cd416d77",  
        "block_height":358893,  
        "creation_time":"2024-10-02T10:35:15Z",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"null\"}"}  
    

Finally, let's repeat the query:
    
    
    execute --name 6d1...37b --method fetch_counter_from --method-args '{"contract_id":"7b2...27d"}'  
    

And confirm that the counter got incremented to 1.
    
    
    {  
        "batch_author":"",  
        "batch_id":"",  
        "block_height":0,  
        "creation_time":"",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"1\"}"  
    }  
    

Now query the `Counter` Applet directly and you will see the same result.
    
    
    execute --name 7b2...27d --method get_count  
    
    
    
    {  
        "batch_author":"",  
        "batch_id":"",  
        "block_height":0,  
        "creation_time":"",  
        "status":"Finalized",  
        "tx_idx":0,  
        "txn_result":"{\"Ok\":\"1\"}"  
    }  
    

  * Preparations
  * Definition
  * Bindings generation
  * Compiling the contract
  * Cross-contract bindings
  * Filling in the logic
  * Deploy Contracts



Copyright © 2026 Weilliptic, Inc.

                
