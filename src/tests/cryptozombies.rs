use ethabi::{Address, Uint};
use ethabi_contract::use_contract;

use crate::utils::sender_name_to_eth_address;
use crate::EvmContract;

use super::test_utils;

use_contract!(cryptozombies, "src/tests/zombieAttack.abi");

fn deploy_cryptozombies(contract: &mut EvmContract) {
    let zombie_code = include_bytes!("zombieAttack.bin").to_vec();
    contract.deploy_code(
        "zombies".to_owned(),
        String::from_utf8(zombie_code).unwrap(),
    );
}

fn create_random_zombie(contract: &mut EvmContract, name: &str) {
    let (input, _decoder) = cryptozombies::functions::create_random_zombie::call(name.to_string());
    contract.call_contract("zombies".to_owned(), hex::encode(input));
}

fn get_zombies_by_owner(contract: &mut EvmContract, owner: Address) -> Vec<Uint> {
    let (input, _decoder) = cryptozombies::functions::get_zombies_by_owner::call(owner);
    let output = contract.call_contract("zombies".to_owned(), hex::encode(input));
    let output = hex::decode(output);
    cryptozombies::functions::get_zombies_by_owner::decode_output(&output.unwrap()).unwrap()
}

#[test]
fn test_send_and_retrieve() {
    test_utils::run_test(100, |mut contract| {
        deploy_cryptozombies(&mut contract);
        assert_eq!(
            contract.balance("owner1".to_string()),
            0
        );

        contract.add_near();

        // TODO: assert contract NEAR balance
        // assert_eq!(
        //     "zombies".to_string(),
        //     100
        // );
        assert_eq!(
            contract.balance("owner1".to_string()),
            100
        );

        contract.retrieve_near("owner1".to_string(), 100);
        assert_eq!(
            contract.balance("owner1".to_string()),
            0
        );
    })
}

#[test]
#[should_panic]
fn test_double_deploy() {
    test_utils::run_test(0, |mut contract| {
        deploy_cryptozombies(&mut contract);
        deploy_cryptozombies(&mut contract);
    })
}

#[test]
// CryptoZombies
fn test_create_random_zombie() {
    test_utils::run_test(0, |mut contract| {
        deploy_cryptozombies(&mut contract);

        assert_eq!(
            get_zombies_by_owner(&mut contract, sender_name_to_eth_address(&"owner1".to_string())),
            []
        );

        create_random_zombie(&mut contract, "zomb1");
        assert_eq!(
            get_zombies_by_owner(&mut contract, sender_name_to_eth_address(&"owner1".to_string())),
            [Uint::from(0)]
        );

        create_random_zombie(&mut contract, "zomb2");
        assert_eq!(
            get_zombies_by_owner(&mut contract, sender_name_to_eth_address(&"owner1".to_string())),
            [Uint::from(0)]
        );
    });
}
