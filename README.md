# **Ckbfi-contract — A bonding curve contract implemented based on CKB contracts**

- A bonding curve is a mathematical concept that defines the relationship between the price and supply of an asset.
- Bonding curve contracts are ckb smart contracts that aim to create a market for tokens independent of cryptocurrency exchanges.
- The value of each token tends to increase as the number of tokens issued increases, according to the bonding curve.



A bonding curve is a mathematical concept that describes the relationship between the price and supply of an asset. The fundamental idea behind a bonding curve is that when a person acquires an asset available in a limited quantity, each subsequent participant will have to provide slightly more for it. This is because the number of available asset units decreases with each acquisition, making the asset more valuable. This mechanism seeks to provide benefits to early participants.

## Ckbfi Overview

CKB.FI support creating tokens through X content, which is natural and convenient, and conforms to the characteristics of users who directly find hot topics to issue coins from X. 

At the same time, CKB.FI will record the Twitter content of each memecoin, highlight the source of each meme, and highlight the importance of culture. After the token is successfully launched, the X  information will be uploaded to Arweave for permanent preservation. 

Through the game of memecoin, valuable content in the social network will be permanently stored on the blockchain.

### Key Features:

1. **Token Creation from X Content**: Users can mint tokens based on trending topics or memes from X, making the process seamless and natural.
2. **Cultural Preservation**: CKBFI records the X content associated with each memecoin, ensuring the origin and cultural relevance of the token are preserved. Once a token is launched, the associated X content is permanently stored on **Arweave**, a decentralized storage solution, ensuring it remains immutable and accessible.
3. **Memecoin Gamification**: By gamifying token creation and trading, CKBFI encourages the preservation of valuable social network content on the blockchain.



### **Exponential Bonding Curve**

![image](https://github.com/ckbfi/ckbfi-contract/blob/main/asset/image.png?raw=true)

The pricing mechanism in the Ckbfi-contract is based on an **exponential curve**. This means:

- Early buyers benefit the most, as the price starts low and increases gradually.
- As more tokens are issued, the price rises exponentially, rewarding early participants while maintaining a fair and predictable price increase for later buyers.

This design ensures that early adopters are incentivized while providing a sustainable token economy for all participants.



### Technical Implementation

![image-20241219170908341](https://github.com/ckbfi/ckbfi-contract/blob/main/asset/image-20241219170908341.png?raw=true)

On CKBFI, when users engage in swap transactions, it primarily involves the following three steps:

1. **Order Construction**: Users construct their orders by signing a message that includes the asset type, amount, and other parameters.
2. **Aggregation and Matching**: The aggregator collects all user orders, explores on-chain and off-chain liquidity sources, and performs order matching.
3. **Transaction Submission**: The aggregator assembles all eligible transactions and submits them on-chain.



#### Transaction Structure

* Buy

```

Transaction {
    cell_deps: [
        // ⭐ 必须包含的合约脚本
        CellDep { 
            out_point: bondings_curve_script.out_point,  // bondings_curve_script部署位置
            dep_type: "code"
        },
        CellDep { 
            out_point: order_script.out_point,  // order_script部署位置
            dep_type: "code"
        },
        CellDep { 
            out_point: unique_manager_liquidity_script.out_point,  // unique_manager_liquidity_script部署位置
            dep_type: "code"
        },
        CellDep { 
            out_point: xudt_script.out_point,  // xudt_script部署位置
            dep_type: "code"
        },
    ],
    inputs: [
    		bondings_curve_pool_xudt_liquidity_cell,
    		bondings_curve_pool_ckb_liquidity_cell,
    		order_cell,
    		unique_manager_liquidity_cell
    		...(transaction fee)
    ]
    outputs: [
    		// bondings_curve_pool_xudt_liquidity_cell
        Output {
            lock: Script { 
                code_hash: bondings_curve_script_code_hash,     
                args: xudt_args |  type_id  
            },
            type: Script { 
                code_hash: xudt_script_code_hahs, 
                args: xudt_args         
            },
            data: encode(output_xudt_liquidity_xudt_amount)
        },
        // bondings_curve_ckb_xudt_liquidity_cell
        Output {
            lock: Script { 
                code_hash: bondings_curve_script_code_hash,     
                args: xudt_args |  typeId  
            },
            capacity:output_ckb_liquidity_capacity,
            data: 0x
        },
        // to_user_xudt_cell
        Output {
            lock: Script { 
                code_hash: user_lock_code_hash,     
                args: user_lock_args 
            },
            type: Script { 
                code_hash: xudt_script_code_hahs,
                args: xudtArgs         
            },
            data: encode(to_user_xudt_amount)
        },
        // unique_manager_liquidity_cell
        Output {
            lock: Script { 
                code_hash: unique_manager_liquidity_script_code_hash,     
                args: xudt_args | type_id  
            },
            type: Script { 
                code_hash: unique_manager_liquidity_script_code_hash,
                args: xudt_args | type_id          
            },
            data: encode(output_xudt_liquidity_xudt_amount | output_ckb_liquidity_capacity)
        },
        // ...
        (transaction fee)
    ]
}

```

* Sell

```

Transaction {
    cell_deps: [
        // ⭐ 必须包含的合约脚本
        CellDep { 
            out_point: bondings_curve_script.out_point,  // bondings_curve_script部署位置
            dep_type: "code"
        },
        CellDep { 
            out_point: order_script.out_point,  // order_script部署位置
            dep_type: "code"
        },
        CellDep { 
            out_point: unique_manager_liquidity_script.out_point,  // unique_manager_liquidity_script部署位置
            dep_type: "code"
        },
        CellDep { 
            out_point: xudt_script.out_point,  // xudt_script部署位置
            dep_type: "code"
        },
    ],
    inputs: [
    		bondings_curve_pool_xudt_liquidity_cell,
    		bondings_curve_pool_ckb_liquidity_cell,
    		order_cell,
    		unique_manager_liquidity_cell
    		...(transaction fee)
    ]
    outputs: [
    		// bondings_curve_pool_xudt_liquidity_cell
        Output {
            lock: Script { 
                code_hash: bondings_curve_script_code_hash,     
                args: xudt_args |  type_id  
            },
            type: Script { 
                code_hash: xudt_script_code_hahs, 
                args: xudt_args         
            },
            data: encode(output_xudt_liquidity_xudt_amount)
        },
        // bondings_curve_ckb_xudt_liquidity_cell
        Output {
            lock: Script { 
                code_hash: bondings_curve_script_code_hash,     
                args: xudt_args |  typeId  
            },
            capacity:output_ckb_liquidity_capacity
            data: 0x
        },
        // to_user_ckb_cell
        Output {
            lock: Script { 
                code_hash: user_lock_code_hash,     
                args: user_lock_args 
            },
            capacity: to_user_ckb_amount
            data: 0x
        },
        // unique_manager_liquidity_cell
        Output {
            lock: Script { 
                code_hash: unique_manager_liquidity_script_code_hash,     
                args: xudt_args | type_id  
            },
            type: Script { 
                code_hash: unique_manager_liquidity_script_code_hash,
                args: xudt_args | type_id         
            },
            data: encode(output_xudt_liquidity_xudt_amount | output_ckb_liquidity_capacity)
        },
        // ...
        (transaction fee)
    ]
}

```





#### Order Construction

The order cell is used to record the user's transaction intent and ensure that it satisfies specific conditions when consumed.



#### AMM Cell

The Automated Market Maker (AMM) cell is responsible for all the verification logic related to AMM. The AMM cell is further divided into two types: Bonding Curve Cell and Unique Cell.

Bonding Curve Cell: This cell manages the buying, selling, and pricing of tokens. It implements the bonding curve logic, ensuring that the price of the token adjusts according to the supply.

Unique Cell: This cell manages the liquidity aspect of the AMM. It ensures that there is sufficient liquidity for token swaps.



## Project Compilation
```bash
make build
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.



