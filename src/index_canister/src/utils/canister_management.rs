

#[ic_cdk::update]
pub async fn create_post_canister() -> Result<candid::Principal, String> {
    let mut all_accounts: Vec<core::types::UserProfile> = Vec::new();
    let mut all_canister_ids: std::collections::HashMap<String, candid::Principal> =
        std::collections::HashMap::new();

    let mut controllers = crate::with_read_state(|state| {
        for (_, acc) in state.account.iter() {
            all_accounts.push(core::types::UserProfile {
                membership: acc.membership,
                user_id: acc.user_id,
                username: acc.username,
            });
        }

        state
            .canister_meta_data
            .get(&0)
            .expect(core::constants::ERROR_FAILED_CANISTER_DATA)
            .controllers
    });

    all_canister_ids.insert(String::from("index_canister"), ic_cdk::api::id());

    controllers.insert(ic_cdk::api::id());

    let args: core::types::PostCanisterInitArgs = core::types::PostCanisterInitArgs {
        accounts: all_accounts,
        canister_ids: all_canister_ids,
        controllers: controllers,
    };

    // converting into bytes
    let args_bytes: Vec<u8> = candid::encode_one(args).map_err(|err| err.to_string())?;

    // controllers
    let controllers: Vec<candid::Principal> = vec![ic_cdk::api::caller(), ic_cdk::api::id()];

    // canister creation args
    //   let controller_settings = kaires::canister_mgmt_types::CanisterSettings {
    //     controllers: Some(controllers),
    //     ..Default::default()
    //   }
    // ;

    let canister_args = kaires::canister_mgmt_types::CreateCanisterArgument {
        settings: Some(kaires::canister_mgmt_types::CanisterSettings {
            controllers: Some(controllers),
            ..Default::default()
        }),
    };

    // creating canister and getting canister id
    let (canister_id,) = match kaires::canister_management::create_new_canister(canister_args).await
    {
        Ok(id) => id,
        Err((_, err_string)) => {
            return Err(format!("{}", err_string));
        }
    };

    // deposit cycles to newly created canister
    kaires::canister_management::deposit_cycles_in_canister(canister_id, 150_000_000_000)
        .await
        .unwrap();

    // locating wasm module to insert in canister
    let wasm_module: Vec<u8> =
        include_bytes!("../../../../.dfx/local/canisters/post_canister/post_canister.wasm")
            .to_vec();

    // install post canister logic in new canister
    let code_args = kaires::canister_mgmt_types::InstallCodeArgument {
        mode: kaires::canister_mgmt_types::CanisterInstallMode::Install,
        canister_id: canister_id.canister_id,
        arg: args_bytes,
        wasm_module: wasm_module.clone(),
    };

    // install code
    kaires::canister_management::install_code_in_canister(code_args, wasm_module)
        .await
        .unwrap();

    Ok(canister_id.canister_id)

    // Ok(())
}
