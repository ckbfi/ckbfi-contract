# **Ckbfi-contract — A bonding curve contract implemented based on CKB contracts**

- A bonding curve is a mathematical concept that defines the relationship between the price and supply of an asset.
- Bonding curve contracts are [smart contracts](https://www.coinbase.com/learn/crypto-basics/what-is-a-smart-contract) that aim to create a market for [tokens](https://www.coinbase.com/learn/crypto-basics/what-is-a-token) independent of [cryptocurrency](https://www.coinbase.com/learn/crypto-basics/what-is-cryptocurrency) exchanges.
- The value of each token tends to increase as the number of tokens issued increases, according to the bonding curve.



A bonding curve is a mathematical concept that describes the relationship between the price and supply of an asset. The fundamental idea behind a bonding curve is that when a person acquires an asset available in a limited quantity, each subsequent participant will have to provide slightly more for it. This is because the number of available asset units decreases with each acquisition, making the asset more valuable. This mechanism seeks to provide benefits to early participants.

## Ckbfi Overview

CKB.FI support creating tokens through X content, which is natural and convenient, and conforms to the characteristics of users who directly find hot topics to issue coins from X. 

At the same time, CKB.FI will record the Twitter content of each memecoin, highlight the source of each meme, and highlight the importance of culture. After the token is successfully launched, the X  information will be uploaded to Arweave for permanent preservation. 

Through the game of memecoin, valuable content in the social network will be permanently stored on the blockchain.



### Technical Implementation

![image-20241219170908341](https://github.com/ckbfi/ckbfi-contract/blob/main/asset/image-20241219170908341.png?raw=true)

On CKBFI, when users engage in swap transactions, it primarily involves the following three steps:

1. **Order Construction**: Users construct their orders by signing a message that includes the asset type, amount, and other parameters.
2. **Aggregation and Matching**: The aggregator collects all user orders, explores on-chain and off-chain liquidity sources, and performs order matching.
3. **Transaction Submission**: The aggregator assembles all eligible transactions and submits them on-chain.





#### Order Construction

The order cell is used to record the user's transaction intent and ensure that it satisfies specific conditions when consumed.



#### AMM Cell

The AMM cell is responsible for all the verification logic related to AMM

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.



