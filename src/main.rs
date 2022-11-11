mod rpc;

use rand::Rng;
use std::str::FromStr;
use chainlib::core::{
    PrivateKey,
    PublicKey,
    Address,
    Transaction,
};

mod btc {
    use super::*;
    use chainlib::bitcoin::{
        private_key::BitcoinPrivateKey,
        public_key::BitcoinPublicKey,
        amount::BitcoinAmount,
        network::{
            Testnet,
            Mainnet,
            BitcoinNetwork,
        },
        transaction::{
            SignatureHash,
            BitcoinTransaction,
            BitcoinTransactionInput,
            BitcoinTransactionOutput,
        },
        transaction::BitcoinTransactionParameters,
        BitcoinAddress,
        BitcoinFormat,
    };

    static privkey_1: &str = "cTuqvkNTYsrbcUNPC2XYoBy8XpnEc8UHEdYqcLLqjgvg83jfNexE";
    static pubkey_1: &str = "03d75ab088dde3629a1271a1f7bb2f614da7876a6c368b87434f4c9c1530d5d81d";
    static bech32_1: &str = "tb1qkqu2z204tkdqvmdcy5y9j7nj09r8gy2hpkq8eq";
    static p2pkh_1: &str = "mwaj4WJb12aLM2dDxVVroGZrEoAc4S9fcW";
    static p2sh_p2wpkh_1: &str = "2N6d4JweCd6NcMDwMZ83qZkYjyrBK3sx1hR";

    static privkey_2: &str = "cQu5wkqTQwQbk8msF626q9U35MS1iUDxhNg1mUqNCyYM6iuYGAR3";
    static pubkey_2: &str = "03b7f0bd964888f4dfda6ec11a7fcca9b5166dbea25677604d3d45e2c01d8dfb47";
    static bech32_2: &str = "tb1qksn24p2fp7t8cer50fnc8k63ka5erl0x2msdq3";
    static p2pkh_2: &str = "mwwWF9i6ckyP4sQUNaRcQ5sjh46rRnKiQG";
    static p2sh_p2wpkh_2: &str = "2MyhacsFMbzHvAwAh2VvKLY3yC7GXUuij9c";

    static faucet: &str = "moneyqMan7uh8FqdCA2BV5yZ8qVrc9ikLP";

    #[test]
    fn btc_tx_gen() {
        let signing_key = BitcoinPrivateKey::<Testnet>::from_str(privkey_1).unwrap();

        let txid = "31ccb320ed15ac6cdf57381ce23e3d4b5601ad971eea800d131f1d6e7c2dd559";
        let txid = hex::decode(txid).unwrap();
        
        let from = BitcoinAddress::<Testnet>::from_str(p2pkh_1).unwrap();
        let to = BitcoinAddress::<Testnet>::from_str(faucet).unwrap();
        let balance = BitcoinAmount::from_ubtc(200).unwrap();
        let value = BitcoinAmount::from_ubtc(100).unwrap();
        let fee = BitcoinAmount::from_ubtc(10).unwrap();
        let change = balance.sub(value).unwrap().sub(fee).unwrap();

        let input = BitcoinTransactionInput::new(
            txid,
            0,
            Some(from.clone()),
            Some(balance),
            None,
            None,
            None,
            SignatureHash::SIGHASH_ALL
        ).unwrap();

        let output_transfer =
            BitcoinTransactionOutput::new(&to, value).unwrap();
        
        let output_change =
            BitcoinTransactionOutput::new(&from, change).unwrap();

        let params = BitcoinTransactionParameters::new(
            vec![input],
            vec![output_transfer, output_change],
        ).unwrap();

        let mut tx = BitcoinTransaction::new(&params).unwrap();
        let stream = tx.sign_with_private_key(&signing_key).unwrap();

        println!("tx hex = {}", hex::encode(&stream));
    }

    #[test]
    fn omni_tx_gen() {
        let signing_key = BitcoinPrivateKey::<Testnet>::from_str(privkey_1).unwrap();
        
        let txid = "31ccb320ed15ac6cdf57381ce23e3d4b5601ad971eea800d131f1d6e7c2dd559";
        let txid = hex::decode(txid).unwrap();

        // indicates the sender of OMNI coins
        let from = BitcoinAddress::<Testnet>::from_str(p2pkh_1).unwrap();

        // indicates the recipient of OMNI coins
        let to = BitcoinAddress::<Testnet>::from_str(p2pkh_2).unwrap();
        
        // indicates the Bitcoin balance in the address of the sender, which provides the gas fee
        let balance = BitcoinAmount::from_ubtc(100).unwrap();

        // indicates the amount of Bitcoin required to be sent to the recipient of OMNI coins,
        // as specified in the omni protocol
        let value = BitcoinAmount::from_satoshi(546).unwrap();

        // indicates the gas fee for this tx
        let fee = BitcoinAmount::from_ubtc(10).unwrap();
        
        // indicates the Bitcoin change paid back to the sender
        let change = balance.sub(value).unwrap().sub(fee).unwrap();

        // indicates the amount of OMNI coins to be sent in terms of its basic units
        let amount = BitcoinAmount::from_ubtc(100).unwrap();

        // construct the input (sender address)
        let input = BitcoinTransactionInput::new(
            txid,
            0,
            Some(from.clone()),
            Some(balance),
            None,
            None,
            None,
            SignatureHash::SIGHASH_ALL
        ).unwrap();
        
        // construct the change output (sender address again, for OMNI change and Bitcoin change both)
        let output_change =
            BitcoinTransactionOutput::new(&from, change).unwrap();

        // construct the data output, which specifies how many OMNI coins to be sent
        let output_data = BitcoinTransactionOutput::omni_data_output(amount).unwrap();

        // construct the reference output, which contains the recipient address
        let output_reference = BitcoinTransactionOutput::new(
            &to,
            value
        ).unwrap();

        // construct the parameters
        let params = BitcoinTransactionParameters::new(
            vec![input],
            vec![output_change, output_data, output_reference],
        ).unwrap();

        // construct the tx
        let mut tx = BitcoinTransaction::new(&params).unwrap();

        let stream = tx.sign_with_private_key(&signing_key).unwrap();

        println!("tx hex = {}", hex::encode(&stream));
    }
}

mod eth {
    use std::str::FromStr;
    use chainlib::core::{
        PrivateKey,
        Address,
        Transaction,
        ethereum_types::U256
    };
    use chainlib::ethereum::{
        EthereumPrivateKey,
        EthereumPublicKey,
        EthereumAddress,
        Goerli,
        Mainnet,
        EthereumAmount,
        EthereumFormat,
        EthereumTransactionParameters,
        EthereumTransaction,
        encode_transfer,
    };
    
    #[test]
    fn eth_tx_gen() {
        let to = "0x47e9feFa599905371827d5188B0f4E610B765707";
        let to = EthereumAddress::from_str(to).unwrap();
        let value = EthereumAmount::from_eth("0.001").unwrap();

        let gas = U256::from(300000);
        let gas_price = EthereumAmount::from_gwei("150").unwrap();
        let nonce = U256::from(33);

        let params = EthereumTransactionParameters {
            receiver: to,
            amount: value,
            gas: gas,
            gas_price: gas_price,
            nonce: nonce,
            data: vec![]
        };

        let mut tx =
            EthereumTransaction::<Mainnet>::new(&params).unwrap();

        // compute 'signature' and 'recid' from elsewhere 
        let signature: Vec<u8> = vec![];
        let recid: u8 = 0;

        // insert 'sig' and 'recid' into this tx
        let stream = tx.sign(signature, recid).unwrap();
        
        let tx_hex = hex::encode(&stream);
        
        println!("tx hex = {}", tx_hex);
    }

    #[test]
    fn eth_erc20_tx_gen() {
        let erc20_contract = "0x47e9feFa599905371827d5188B0f4E610B765707";
        let erc20_token_recipient = "0x0c9E0e96eBCce0636C7da29A28FABD1ce37B593b";
        
        let erc20_contract = EthereumAddress::from_str(erc20_contract).unwrap();
        let erc20_token_recipient = EthereumAddress::from_str(erc20_token_recipient).unwrap();
        let eth_value = EthereumAmount::from_eth("0.001").unwrap();
        let token_value = U256::from(1000);

        let gas = U256::from(300000);
        let gas_price = EthereumAmount::from_gwei("150").unwrap();
        let nonce = U256::from(33);
        
        let data = encode_transfer("transfer", &erc20_token_recipient, token_value);
        
        let params = EthereumTransactionParameters {
            receiver: erc20_contract,
            amount: eth_value,
            gas: gas,
            gas_price: gas_price,
            nonce: nonce,
            data: data
        };

        let mut tx =
            EthereumTransaction::<Mainnet>::new(&params).unwrap();

        // compute 'sig' and 'recid' elsewhere
        let signature: Vec<u8> = vec![];
        let recid: u8 = 0;
        
        let stream = tx.sign(signature, recid).unwrap();
        
        let tx_hex = hex::encode(&stream);
        
        println!("tx hex = {}", tx_hex);
    }
}

mod fil {
    use super::*;
    use chainlib::filecoin::{
        private_key::FilecoinPrivateKey,
        public_key::FilecoinPublicKey,
        address::FilecoinAddress,
        format::FilecoinFormat,
        amount::{
            FilecoinAmount,
            FilecoinAmountConverter,
        },
        transaction::{
            FilecoinTransactionParameters,
            FilecoinTransaction,
            RawBytes,
        },
    };

    #[test]
    fn fil_tx_gen() {
        let signing_key = FilecoinPrivateKey::new_bls().unwrap();
        let to_key = FilecoinPrivateKey::new_secp256k1().unwrap();

        let format = FilecoinFormat::Base32;
        
        let from = signing_key.to_address(&format).unwrap();
        let to = to_key.to_address(&format).unwrap();
        let value = FilecoinAmount::from_fil("0.01");

        let params = FilecoinTransactionParameters {
            version: 0,
            from: from,
            to: to,
            sequence: 33,
            value: value,
            method_num: 0,
            params: RawBytes::new(vec![]),
            gas_limit: 500000,
            gas_fee_cap: FilecoinAmount::from_nano_fil("10000"),
            gas_premium: FilecoinAmount::from_nano_fil("1000"),
        };

        let mut tx = FilecoinTransaction::new(&params).unwrap();
        
        let tx_json = tx.sign_with_private_key(&signing_key).unwrap();

        println!("tx json = {}", String::from_utf8(tx_json).unwrap());
    }
}

fn main() {

}
