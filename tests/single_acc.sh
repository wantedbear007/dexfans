
USERNAME="bhanu"
CAPTCHA="dbSa"

 RES=$(dfx canister call index_canister api_create_account "(
      record {
        username = \"$USERNAME\";
        bio = opt \"BIO\";
        cover_image = opt \"1\";
        captcha_solution = \"$CAPTCHA\";
        avatar = opt \"AVATAR\";
      }
    )")


echo $RES