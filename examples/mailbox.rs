use cainome::rs::abigen;
use starknet::{
    accounts::{Account, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount},
    core::types::{BlockId, BlockTag, FieldElement},
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use std::sync::Arc;
use url::Url;

// To run this example, please first run `make setup_simple_get_set` in the contracts directory with a Katana running. This will declare and deploy the testing contract.

const CONTRACT_ADDRESS: &str = "0x02547c9b670ef68e01d4ae57ef2d548fcedb225f2ceadde93b1c6bf63ce40862";
const KATANA_ACCOUNT_0: &str = "0x7e743146befa336ed5fbc330691f52b910a125f6ba0715545975289ff1d9f56";
const KATANA_PRIVKEY_0: &str = "0x135e90e280cff5a864730d22f1fc911b";
const KATANA_CHAIN_ID: &str = "0x534e5f5345504f4c4941";

// You can load of the sierra class entirely from the artifact.
// Or you can use the extracted abi entries with jq in contracts/abi/.
abigen!(
    MyContract,
    "./contracts/pre-compiled/Mailbox.contract_class.json",
    type_aliases {
        openzeppelin::access::ownable::ownable::OwnableComponent::Event as OwnableEvent;
        openzeppelin::upgrades::upgradeable::UpgradeableComponent::Event as UpgradeableEvent;
    }
);
//abigen!(MyContract, "./contracts/abi/simple_get_set.abi.json");

#[tokio::main]
async fn main() {
    let rpc_url = Url::parse("http://0.0.0.0:6060").expect("Expecting Starknet RPC URL");
    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let contract_address = FieldElement::from_hex_be(CONTRACT_ADDRESS).unwrap();

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(KATANA_PRIVKEY_0).unwrap(),
    ));
    let address = FieldElement::from_hex_be(KATANA_ACCOUNT_0).unwrap();

    let account = Arc::new(SingleOwnerAccount::new(
        provider,
        signer,
        address,
        FieldElement::from_hex_be(KATANA_CHAIN_ID).unwrap(),
        ExecutionEncoding::New,
    ));

    let contract = MyContract::new(contract_address, account);

    let a = contract
        .delivered(&FieldElement::ZERO)
        .call()
        .await
        .expect("Call to `delivered` failed");

    println!("result delivered: {:?}", a);
}
