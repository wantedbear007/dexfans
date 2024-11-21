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

    // const WASM: &[u8] =
    //     include_bytes!("../../../../target/wasm32-unknown-unknown/release/post_canister.wasm.gz");

    // let princi: candid::Principal = kaires::canister_mgmt::create_users_canister(
    //     WASM,
    //     args_bytes.clone(),
    //     controllers.clone(),
    //     core::constants::ESSENTIAL_POST_CANISTER_CYCLE_THRESHOLD,
    // )
    // .await;

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
    kaires::canister_management::deposit_cycles_in_canister(
        canister_id,
        core::constants::ESSENTIAL_POST_CANISTER_CYCLE_THRESHOLD,
    )
    .await
    .unwrap();

    // locating wasm module to insert in canister
    let wasm_module: Vec<u8> =
        include_bytes!("../../../../.dfx/local/canisters/post_canister/post_canister.wasm.gz")
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

    // changing active post canister
    let _ = crate::with_write_state(|state| match state.canister_meta_data.get(&0) {
        Some(mut data) => {
            data.active_post_canister = canister_id.canister_id;
            data.all_post_canisters.insert(canister_id.canister_id);
            state.canister_meta_data.insert(0, data);
            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA)),
    });

    // Ok(canister_id.canister_id);

    Ok(canister_id.canister_id)
    // Ok(())
}
