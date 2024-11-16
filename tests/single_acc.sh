
USERNAME="BHANUPRATAP"


 RES=$(dfx canister call index_canister api_create_account "(
      record {
        username = \"$USERNAME\";
        bio = opt \"BIO\";
        cover_image = opt \"COVER_IMAGE\";
        avatar = opt \"AVATAR\";
      }
    )")


echo $RES