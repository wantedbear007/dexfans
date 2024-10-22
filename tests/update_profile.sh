
BIO="hello from suraj"
USERNAME="wantedbear006"
COVER_IMAGE="1"
AVATAR="1"

RES=$(dfx canister call dexfans_backend api_update_profile "(
  record {
    bio = opt \"$BIO\";
    username = \"$USERNAME\";
    cover_image = opt \"$COVER_IMAGE\";
    avatar = opt \"$AVATAR\";
  }
)")

echo $RES