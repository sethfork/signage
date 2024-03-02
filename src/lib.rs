use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PromiseOrValue};
use serde::{Serialize, Deserialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
pub struct Signature {
    account_id: String,
    code: String,
    signature: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    signatures: std::collections::HashMap<String, Signature>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            signatures: std::collections::HashMap::new(),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn sign(&mut self, code: String, path: String) -> PromiseOrValue<Signature> {
        let payload = env::sha256(code.as_bytes());
        let account_id = env::signer_account_id();
        let signature = format!("{}:{}", env::signer_account_pk().to_string(), code);

        let signature_record = Signature {
            account_id: account_id.clone().to_string(),
            code: code.clone(),
            signature: signature.clone(),
        };

        self.signatures.insert(account_id.clone().to_string(), signature_record.clone());

        PromiseOrValue::Value(signature_record)
    }

    pub fn get_signature(&self, account_id: String) -> Option<Signature> {
        self.signatures.get(&account_id).cloned()
    }
}