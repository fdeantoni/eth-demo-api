use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::H160,
};

#[derive(Clone)]
pub struct Validator {
    contract: Contract<Http>,
}

impl Validator {
    pub async fn is_valid(&self, key: String) -> bool {
        let result: bool = self
            .contract
            .query("isValid", (key.clone(),), None, Options::default(), None)
            .await
            .unwrap();
        log::debug!("{} is valid?: {}", key, result);
        result
    }

    pub fn new() -> Self {
        let infura_project =
            std::env::var("INFURA_PROJECT").expect("Please set the INFURA_PROJECT env variable");
        let network_url = format!("https://kovan.infura.io/v3/{}", infura_project);
        let vault_address = {
            let string =
                std::env::var("VAULT_ADDRESS").expect("Please set VAULT_ADDRESS env variable");
            if let Some(stripped) = string.strip_prefix("0x") {
                stripped.to_string()
            } else {
                string
            }
        };

        let http = web3::transports::Http::new(&network_url).unwrap();
        let web3 = web3::Web3::new(http);

        let bytes = hex::decode(vault_address).unwrap();
        let address = H160::from_slice(&bytes);
        let abi_json = include_bytes!("../target/DemoVault.abi");
        let contract = Contract::from_json(web3.eth(), address, abi_json).unwrap();

        Validator { contract }
    }
}
