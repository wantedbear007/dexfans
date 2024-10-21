import { Ed25519KeyIdentity } from '@dfinity/identity'
import { createAgent } from '@dfinity/utils'
import { BucketCanister, Uploader } from '@ldclabs/ic_oss_ts'

const IS_LOCAL = true
const apiHost = IS_LOCAL ? 'http://127.0.0.1:4943' : 'https://icp-api.io'
const bucketCanister = 'br5f7-7uaaa-aaaaa-qaaca-cai'

// The principal is generated by generateIdentity()
// 'pxfqr-x3orr-z5yip-7yzdd-hyxgd-dktgh-3awsk-ohzma-lfjzi-753j7-tae'
// update the principal as manager
// export MYID=$(dfx identity get-principal)
// dfx canister call asset_handler admin_set_managers "(vec {principal \"$MYID\"; principal \"pxfqr-x3orr-z5yip-7yzdd-hyxgd-dktgh-3awsk-ohzma-lfjzi-753j7-tae\"})"
const idJSON =
  '["302a300506032b6570032100f6f7b1317cca7be2c3f6049da6932aadbd5549d4fd7d7d29290dead0b85d1f96","5b3770cbfd16d3ac610cc3cda0bc292a448f2c78d6634de6ee280df0a65e4c04"]'

const files = [

  {
    parent: 0,
    content: 'package.json',
    name: '',
    contentType: ''
  },
  {
    parent: 0,
    content: '30mvideo.mp4',
    name: '',
    contentType: ''
  },
  {
    parent: 0,
    content: 'video.mp4',
    name: '',
    contentType: ''
  }
]

async function main() {
  // generateIdentity()
  await uploadFiles(files)
}

async function uploadFiles(files) {
  const identity = Ed25519KeyIdentity.fromJSON(idJSON)
  const agent = await createAgent({
    identity,
    fetchRootKey: IS_LOCAL,
    host: apiHost,
    verifyQuerySignatures: true
  })
  const bucketClient = BucketCanister.create({
    agent,
    canisterId: bucketCanister
  })
  console.log('Bucket info:\n', await bucketClient.getBucketInfo())
  console.log('Bucket files in root folder:\n', await bucketClient.listFiles(0))

  const uploader = new Uploader(bucketClient)

  for (const file of files) {
    const result = await uploader.upload(file, (progress) => {
      console.log(`Upload ${file.name}:`, progress)
    })

    console.log(`Uploaded ${file.name}:`, result)
  }

  console.log('Bucket files in root folder:\n', await bucketClient.listFiles(0))
}

main().catch(console.error)

