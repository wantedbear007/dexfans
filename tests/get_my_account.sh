RES=$(dfx canister call index_canister api_get_my_profile)
echo $RES



  #  record {
  #     content = \"$content\";
  #     post_type = variant { $post_type };
  #     video = $video_field;
  #     image = $image_field;
  #     price = $price_field;
  #     post_status = variant { Published };
  #     post_visibility = variant { Everyone };
  #   }