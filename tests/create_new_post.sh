content="hello from wantedbha"
post_type="Gold"
video="1"
image="2"
price=11

RES=$(dfx canister call bd3sg-teaaa-aaaaa-qaaba-cai api_create_new_post "(
  record {
    content = \"$content\";
    post_type = variant { $post_type };
    video = opt \"$video\";
    image = opt \"$image\";
    price = opt ($price : nat8);
  }
)")

echo "Result: $RES"