content="hello from wantedbha"
post_type="Gold"
video="1"
image="2"
price=11

RES=$(dfx canister call post_canister api_create_new_post "(
  record {
    content = \"$content\";
    video = opt \"$video\";
    image = opt \"$image\";
    price = opt ($price : nat8);
    post_visibility = varient { DiamondUser };
  }
)")

echo "Result: $RES"
    # post_type = variant { $post_type };
