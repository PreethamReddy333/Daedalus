 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

On this page

# Create your first MCP Server

Model Context Protocol (MCP) servers enable AI models to interact with external tools and data sources in a standardized way. On the WeilChain platform, you can create MCP servers as applets that expose functions as tools for AI models to use.

note

Currently only Rust and Golang are supported for MCP server development. C++ and Typescript are supported for non-MCP server applet development.

This tutorial will guide you through creating an MCP server applet that provides arithmetic operations for AI models.

## What is an MCP Server?​

An MCP server is a standardized interface that allows AI models to access external functionality through well-defined tools. In the WeilChain ecosystem, MCP servers are deployed as applets that:

  * Expose functions as AI-callable tools
  * Provide structured descriptions for each tool
  * Handle parameter validation and execution
  * Return results in a format AI models can understand



The key advantage of MCP servers on WeilChain is that they run as decentralized applets, ensuring reliability, transparency, and auditability.

## Prerequisites​

This tutorial assumes that:

  * You have completed the [How-to install the WIDL compiler and extension](/docs/howtos/install_widl)
  * You have Rust installed and configured for WebAssembly development
  * You are familiar with basic Rust programming concepts



## Project Setup​

First, create a new Rust project for your MCP server:
    
    
    cargo new --lib arithmetic  
    cd arithmetic  
    

## Interface Specification with @mcp Annotation​

Create a file `arithmetic.widl` with the following contents:

arithmetic.widl
    
    
    @mcp  
    interface Arithmetic {  
        // adds two numbers  
        query func add(  
            // the first number  
            x: int,   
            // the number to add with the first one  
            y: int) -> int;  
        // multiply two numbers  
        query func multiply(  
            // the first number  
            x: int,  
            // the number to multiply with the first one  
            y: int) -> int  
    }  
    

The `@mcp` annotation is crucial here - it tells the WIDL compiler to generate MCP-compatible bindings. Notice several important aspects:

### Function Comments as Tool Descriptions​

The comments above each function and parameter become the tool descriptions that AI models use to understand when and how to call your tools. These descriptions are critical because:

  * **AI Tool Selection** : The AI model reads these descriptions to determine which tool to use for a given task
  * **Parameter Understanding** : Parameter descriptions help the AI provide correct arguments
  * **Context Awareness** : Well-written descriptions help the AI understand the tool's purpose and limitations



## Generate Server Bindings​

Generate the MCP server bindings using the WIDL compiler:
    
    
    widl generate arithmetic.widl server rust  
    

This creates a `bindings.rs` file. The generated code includes a special `tools()` method that returns a JSON description of available tools:
    
    
    fn tools(&self) -> String {  
        r#"[  
      {  
        "type": "function",  
        "function": {  
          "name": "add",  
          "description": "adds two numbers\n",  
          "parameters": {  
            "type": "object",  
            "properties": {  
              "x": {  
                "type": "number",  
                "description": "the first number\n"  
              },  
              "y": {  
                "type": "number",  
                "description": "the number to add with the first one\n"  
              }  
            },  
            "required": [  
              "x",  
              "y"  
            ]  
          }  
        }  
      },  
      {  
        "type": "function",  
        "function": {  
          "name": "multiply",  
          "description": "multiply two numbers\n",  
          "parameters": {  
            "type": "object",  
            "properties": {  
              "x": {  
                "type": "number",  
                "description": "the first number\n"  
              },  
              "y": {  
                "type": "number",  
                "description": "the number to multiply with the first one\n"  
              }  
            },  
            "required": [  
              "x",  
              "y"  
            ]  
          }  
        }  
      }  
    ]"#.to_string()  
    }  
    

This JSON follows the standard function calling format, making it compatible with various AI models and frameworks.

## Implement the Applet Logic​

Copy the generated bindings to your main library file:
    
    
    cat bindings.rs > src/lib.rs  
    rm bindings.rs  
    

Update your `Cargo.toml` to include the necessary dependencies:

Cargo.toml
    
    
    [package]  
    name = "arithmetic"  
    version = "0.1.0"  
    edition = "2025"  
      
      
    [dependencies]  
    weil_rs = { path = "../../../wadk/adk/rust/weil_rs/" }  
    weil_macros = { path = "../../../wadk/adk/rust/weil_rs/weil_macros" }  
    weil_contracts = { path = "../../../wadk/adk/rust/weil_rs/weil_contracts" }  
    anyhow = "1.0.97"  
    serde = { version = "1.0.219", features = ["derive", "rc"] }  
    serde_json = { version = "1.0.140", features = ["raw_value"] }  
      
    [lib]  
    crate-type = ["cdylib"]  
    

Now implement the applet logic in `src/lib.rs`. Replace the unimplemented functions with:

src/lib.rs
    
    
        #[query]  
        async fn add(&self, x: i32, y: i32) -> i32 {  
            x + y  
        }  
      
        #[query]  
        async fn multiply(&self, x: i32, y: i32) -> i32 {  
            x * y  
        }  
    

## Compile the MCP Server​

Build your MCP server applet:
    
    
    cargo build --target wasm32-unknown-unknown --release  
    

You should now have a file `target/wasm32-unknown-unknown/release/arithmetic_mcp.wasm` containing your MCP server applet.

## Deploy Your MCP Server​

To deploy your MCP server applet to the WeilChain platform, follow the detailed instructions in the [Deploy an Applet on Weilchain using the CLI](/docs/tutorials/deploy_applets) or [Deploy an Applet on Weilchain using the Explorer](/docs/tutorials/view_applets) tutorials.

## Additional​

In most cases, you would want to write an MCP applet that interacts with external services through rest apis. To make deterministic http outcalls from an mcp server, the `weil_rs` provides a neat abstraction called the `HttpClient` which you can leverage in your applet code.
    
    
    let response = HttpClient::request(url, HttpMethod::Get)  
        .send()  
        .map_err(|e| format!("HTTP request failed: {}", e))?;  
      
    let response_text = response.text();  
    

Internally it uses a system api called `make_outcall` which makes the final call in an idempotent manner from the distributed system.

To enable usage of this idempotency service by your applet, deploy the applet with an additional `-e` flag.
    
    
    deploy -e -f path/to/file.wasm -p path/to/file.widl  
    

Also, if you want to deploy your mcp server in a specific weilpod (ideally the one geographically closest to your prime consumers), you can specify that with a `-w` flag
    
    
    deploy -e -f path/to/file.wasm -p path/to/file.widl -w POD_ID_OF_REQUISITE_POD  
    

## Next Steps​

Once your MCP server is deployed, you can:

  * Add it to Icarus, the onchain AI-chatbot
  * Interact with the applet using natural language



We'll show you how to do so in the next tutorial, [Integrate MCP Server with Icarus AI Chatbot](/docs/tutorials/register_mcp).

  * What is an MCP Server?
  * Prerequisites
  * Project Setup
  * Interface Specification with @mcp Annotation
    * Function Comments as Tool Descriptions
  * Generate Server Bindings
  * Implement the Applet Logic
  * Compile the MCP Server
  * Deploy Your MCP Server
  * Additional
  * Next Steps



Copyright © 2026 Weilliptic, Inc.

                


 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

On this page

# Integrate MCP Server with Icarus AI Chatbot

Icarus is WeilChain's onchain AI chatbot that can seamlessly integrate with MCP servers deployed as applets. This tutorial will show you how to register your deployed MCP server with Icarus and start using it.

## Prerequisites​

Before starting this tutorial, ensure you have:

  * A deployed MCP server applet on WeilChain (see [Create an MCP Server](/docs/tutorials/mcp_basic)),
  * The applet id of your deployed MCP server,
  * A Web browser,
  * Access to the WAuth extension, Weilliptic's browser Wallet.



## Accessing Icarus​

Navigate to Icarus, the onchain AI chatbot, at: **<https://icarus.weilliptic.ai>**

If you don't have the WAuth extension yet, the screen you will see upon visiting the website will be something like this  You can setup your weil wallet browser extension using this tutorial: [Setup Weil-Wallet extension](/docs/howtos/install_webwallet)

Once you have the wallet configured, you'll be prompted to connect the chatbot to the Weilchain using the wallet  Go ahead and select your account that you want to connect with.

Once done, you'll be presented with the chat interface where you can interact with the AI. 

## Opening MCP Server Settings​

To register your MCP server with Icarus, you need to access the settings panel:

  1. Click on the **Agentic** button, located on top of the chat input panel.
  2. Look for the **settings button** located near the bottom right of the chat interface
  3. Click on the settings button to open the MCP server configuration form



## Registering Your MCP Server​

When you click the settings button, a form will appear allowing you to add your MCP server:

### Fill in the Registration Details​

The form will require two pieces of information:

**Name** : Give your MCP server a descriptive name

  * This name will help you identify the server in Icarus
  * For our arithmetic example, you might use: "Arithmetic"



**Applet Id** : Enter your deployed applet address

  * This is the WeilChain applet address where your MCP server is deployed
  * The address should look like: `0x1234567890abcdef...`



### Example Registration​

For the arithmetic MCP server we created in the previous tutorial:
    
    
    Name: arithmetic  
    Applet Id: 0x742d35Cc6538C0532925a3b8D0b22d98b4C98b4E5d  
    

After filling in the details, click the **Add Server** button to complete the registration.

## Confirming Registration​

Once registered successfully, you should see confirmation that your MCP server has been added to Icarus. The AI chatbot will now have access to the tools provided by your MCP server.

## Using Your MCP Server Tools​

Now that your MCP server is registered, you can start asking questions that will utilize your custom tools. The AI will automatically determine when to use your tools based on the descriptions you provided in your WIDL file.

### Example Interactions​

Here are some example questions you can ask to test your arithmetic MCP server:

"can u add 34 and 89 using tools?"

At this point, you will notice a hint of the tool call that the chatbot makes

And then the response will be delievered 

## Next Steps​

Now that you have successfully integrated your MCP server with Icarus, you can:

  * **Create More Complex Tools** : Add additional functions to your MCP server
  * **Integrate Multiple Servers** : Register additional MCP servers for different functionalities



[Register an MCP server with configurations](/docs/tutorials/mcp_config)

  * Prerequisites
  * Accessing Icarus
  * Opening MCP Server Settings
  * Registering Your MCP Server
    * Fill in the Registration Details
    * Example Registration
  * Confirming Registration
  * Using Your MCP Server Tools
    * Example Interactions
  * Next Steps



Copyright © 2026 Weilliptic, Inc.

                


 Output   Browser Output

                    
Skip to main content

[](/)[Docs](/docs)

On this page

# Deploy Applets with Configuration

When building MCP servers or other applets that need to connect to external services, you often require configuration values like API endpoints, hostnames, ports, or other settings that shouldn't be hardcoded into your applet. WeilChain provides a secure configuration system that allows you to deploy applets with custom configurations while keeping sensitive information separate from your code.

## Why Use External Configuration?​

Hardcoding configuration values in your applet has several drawbacks:

  * **Security Concerns** : Sensitive information like API keys or internal hostnames become visible in the applet code
  * **Flexibility** : Different deployments might need different configurations (development vs production)
  * **Reusability** : Other users can't easily adapt your applet for their own infrastructure
  * **Maintenance** : Changing configuration requires recompiling and redeploying the entire applet



WeilChain's configuration system solves these problems by allowing you to specify configuration externally at deployment time.

## Defining Configuration Structure​

First, define your configuration structure in your WIDL file using a `record` type. This creates a structured schema for your configuration values.

### Example: Postgres Service Configuration​

postgres.widl
    
    
    record PostgresConfig {  
        host: string,  
        port: string  
    }  
      
    @mcp  
    interface Postgres {  
        config -> PostgresConfig;  
          
        // This returns the schema of the database with name given by argument `db_name`.  
        query func schema(db_name: string) -> result<string, string>;  
        // This runs a query provided in argument `query_str` on the database with name given by argument `db_name`.  
        query func run_query(db_name: string, query_str: string) -> result<list<string>, string>;  
        // This executes the statement provided in argument `statement` potentially mutating the rows of the database with name given by argument `db_name`.  
        query func execute(db_name: string, statement: string) -> result<u64, string>  
    }  
    

### Configuration Schema Definition​

The `record` type defines the structure of your configuration:

  * **Field Types** : Use appropriate WIDL types (`string` for now)
  * **Required Fields** : All fields in a record are required by default
  * **Naming** : Use clear, descriptive field names that indicate their purpose



## Creating Configuration Files​

Create a YAML configuration file that matches your defined schema. This file contains the actual values for your deployment.

### Example Configuration File​

config.yaml
    
    
    host: "1.2.3.4"  
    port: "1234"  
    

## Deploying with Configuration​

Use the enhanced deploy command to include your configuration file:
    
    
    deploy -f path/to/applet.wasm -p path/to/applet.widl -c path/to/config.yaml  
    

### Deploy Command Parameters​

  * `-f`: Path to your compiled WASM applet
  * `-p`: Path to your WIDL interface definition
  * `-c`: Path to your YAML configuration file



### Deployment Process​

When you deploy with configuration:

  1. **Validation** : The deploy tool validates your configuration against the WIDL schema
  2. **Encryption** : Configuration values are securely encrypted before storage
  3. **Registration** : The applet is registered with its associated configuration
  4. **Access Control** : Only your applet instance can access its configuration



## Implementing Configuration in Rust​

### Applet State Setup​

Import the necessary types and set up your applet state to use the configuration system:

src/lib.rs
    
    
    use serde::{Deserialize, Serialize};  
    use weil_macros::{constructor, mutate, query, smart_contract, WeilType};  
    use weil_rs::config::Secrets;  
      
    // Your configuration structure (generated from WIDL)  
    #[derive(Serialize, Deserialize, Clone)]  
    pub struct PostgresConfig {  
        pub host: String,  
        pub port: String,  
    }  
      
    #[derive(Serialize, Deserialize, WeilType)]  
    pub struct PostgresContractState {  
        secrets: Secrets<PostgresConfig>,  
    }  
    

### Constructor Implementation​

Initialize your applet state with the configuration system:
    
    
    #[smart_contract]  
    impl Postgres for PostgresContractState {  
        #[constructor]  
        fn new() -> Result<Self, String>  
        where  
            Self: Sized,  
        {  
            Ok(PostgresContractState {  
                secrets: Secrets::new(),  
            })  
        }  
          
        // Implementation methods...  
    }  
    

### Using Configuration Values​

Access configuration values through the `secrets` field in your applet methods:
    
    
    impl PostgresContractState {  
        fn url(&self, endpoint: &str) -> String {  
            format!(  
                "http://{}:{}/{}",  
                self.secrets.config().host,  
                self.secrets.config().port,  
                endpoint  
            )  
        }  
          
        async fn make_api_call(&self, endpoint: &str) -> Result<String, String> {  
            let url = self.url(endpoint);  
              
            // Use the constructed URL for your API call  
            todo!("Implement request to postgres url {}", url)  
        }  
    }  
    

The configuration system enables you to build flexible, secure, and reusable applets that can adapt to different deployment environments while keeping sensitive information protected.

  * Why Use External Configuration?
  * Defining Configuration Structure
    * Example: Postgres Service Configuration
    * Configuration Schema Definition
  * Creating Configuration Files
    * Example Configuration File
  * Deploying with Configuration
    * Deploy Command Parameters
    * Deployment Process
  * Implementing Configuration in Rust
    * Applet State Setup
    * Constructor Implementation
    * Using Configuration Values



Copyright © 2026 Weilliptic, Inc.

                
