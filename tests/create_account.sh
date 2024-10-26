
BIO="hello from bhanu"
USERNAME="wantedbear007"
COVER_IMAGE="1"
AVATAR="1"

RES=$(dfx canister call index_canister api_create_account "(
  record {
    bio = opt \"$BIO\";
    username = \"$USERNAME\";
    cover_image = opt \"$COVER_IMAGE\";
    avatar = opt \"$AVATAR\";
  }
)")

echo $RES