# Number of posts to create
COUNT=5000

# Parameters
content="hello from wantedbha"
post_type="Gold"
video="1"
image="2"
price=11

# Loop to create posts concurrently
for ((i=1; i<=COUNT; i++))
do
  echo "Creating post #$i concurrently with delay"

  dfx canister call post_canister api_create_new_post "(
    record {
      content = \"$content\";
      post_type = variant { $post_type };
      video = opt \"$video\";
      image = opt \"$image\";
      price = opt ($price : nat8);
    }
  )" & 
  
  sleep 0.3
done

wait
echo "All posts have been created."
