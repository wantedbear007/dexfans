


RES=$(dfx canister call post_canister api_create_new_post "(
  record {
    content = \"hello\";
    video = null;
    post_visibility = variant { Everyone };
    image = opt vec {
      record {
        need_pay = true;
        source = 11 : nat32;
        price = opt (12000 : nat32);
      };
    };
    price = null;
    post_status = variant { Archived };
  }
)")

echo $RES
