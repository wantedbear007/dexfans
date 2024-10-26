  # record {
  #   subscribed_by = principal "fsefm-f46ro-lulwk-ex4sf-z33o5-oihe2-lly2w-uommw-7u5xl-6spjb-eae";
  #   subscribed_to = principal "zgulm-lnuba-qhhqd-eumo2-6bmj2-6vsf5-yi5jw-2t34o-6kt7m-xrm34-5qe";
  # }

RESULT=$(dfx canister call post_canister controller_unsubscribe "(
  record {
    subscribed_by = principal "fsefm-f46ro-lulwk-ex4sf-z33o5-oihe2-lly2w-uommw-7u5xl-6spjb-eae";
    subscribed_to = principal "zgulm-lnuba-qhhqd-eumo2-6bmj2-6vsf5-yi5jw-2t34o-6kt7m-xrm34-5qe";
  }
)")

echo $RESULT