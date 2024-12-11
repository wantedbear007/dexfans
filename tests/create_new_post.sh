


RES=$(dfx canister call post_canister api_create_new_post "(
  record {
    content = \"hello\";
    video = null;
    post_visibility = variant { Everyone };
    image = opt vec {
      record {
        need_pay = true;
        source = \"12\";
        price = opt (12_000 : nat);
      };
    };
    price = null;
    post_status = variant { Published };
  }
)")

echo $RES
