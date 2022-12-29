PEM_FILE="./dany_wallet.pem"
wallet_address=erd1240dtgjc42mrfkd7ge20m3pt98xqwpmngflur429cjarmjwaja6s7u3kw5
nft_identifier=0x4149422d313761613365

# Check my nfts
# https://devnet-gateway.elrond.com/address/erd1240dtgjc42mrfkd7ge20m3pt98xqwpmngflur429cjarmjwaja6s7u3kw5/esdt

create_nft_apartment() {
    #"AIB-17aa3e"
    nft_name=0x$(echo "Apartment Top" | xxd -p)
    image_link="https://images.squarespace-cdn.com/content/v1/5a20a850f43b558a9e315d39/1519696736925-M943SJ4T2Q52SEXWEUJ2/On-point-capital-whay-are-so-many-luxury-apartments-being-built.jpeg"
    nft_hash=0x$(echo -n $image_link | openssl dgst -sha256)
    nft_link=0x68747470733a2f2f696d616765732e73717561726573706163652d63646e2e636f6d2f636f6e74656e742f76312f3561323061383530663433623535386139653331356433392f313531393639363733363932352d4d393433534a3454325135325345585745554a322f4f6e2d706f696e742d6361706974616c2d776861792d6172652d736f2d6d616e792d6c75787572792d61706172746d656e74732d6265696e672d6275696c742e6a706567
    pem_file="./dany_wallet.pem"

    erdpy contract call erd1240dtgjc42mrfkd7ge20m3pt98xqwpmngflur429cjarmjwaja6s7u3kw5 --function ESDTNFTCreate --arguments $nft_identifier 1 $nft_name 0 $nft_hash 0 $nft_link  --recall-nonce --gas-limit 70000000 --pem $pem_file --chain=D --send
}

buy_apartment() {
    buyer_address=erd17kces5uqkakp6zu2mzzz96vfr295wztrttwklw7p5uhm3na4hy0sea50kv
    buyer_pem_file="./pemfile.pem"
    half_egld=500000000000000000
    erdpy --verbose contract call $contract_address --recall-nonce --pem=${buyer_pem_file} --gas-limit=50000000 --function="buy_apartment" --value=$half_egld --send --chain=D
}

sell_apartment() {
    user_address=erd1240dtgjc42mrfkd7ge20m3pt98xqwpmngflur429cjarmjwaja6s7u3kw5
    sft_token=str:AIB-17aa3e
    sft_token_nonce=9
    sft_token_amount=1
    sell_apartment=0x73656c6c5f61706172746d656e74
    buyer_address=erd17kces5uqkakp6zu2mzzz96vfr295wztrttwklw7p5uhm3na4hy0sea50kv
    price_apartment=250000000000000000
    
    erdpy --verbose contract call $user_address --recall-nonce --pem=${PEM_FILE} --gas-limit=100000000 --chain=D --function="ESDTNFTTransfer" --arguments $sft_token $sft_token_nonce $sft_token_amount $contract_address $sell_apartment $price_apartment $buyer_address --send || return
}

getDeposit() {
    buyer_address=erd17kces5uqkakp6zu2mzzz96vfr295wztrttwklw7p5uhm3na4hy0sea50kv
    erdpy --verbose contract query ${ADDRESS} --function="getDeposit" --arguments $buyer_address
}


getNoApartments() {
    buyer_address=erd17kces5uqkakp6zu2mzzz96vfr295wztrttwklw7p5uhm3na4hy0sea50kv
    erdpy --verbose contract query ${ADDRESS} --function="getNoApartments" --arguments $buyer_address
}

deploy() {
    erdpy contract build
    
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} --gas-limit=50000000 --send --outfile="deploy-devnet.interaction.json" --chain=D --arguments $nft_identifier || return
    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")
    contract_address=$ADDRESS
}

send_nft() {
    user_address=erd1240dtgjc42mrfkd7ge20m3pt98xqwpmngflur429cjarmjwaja6s7u3kw5
    sft_token=str:AIB-17aa3e
    sft_token_nonce=11
    sft_token_amount=1
    send_nft=0x73656e645f6e6674
    erdpy --verbose contract call $user_address --recall-nonce --pem=${PEM_FILE} --gas-limit=100000000 --chain=D --function="ESDTNFTTransfer" --arguments $sft_token $sft_token_nonce $sft_token_amount $contract_address $send_nft --send || return
}

bid_apartment() {
    buyer_address=erd17kces5uqkakp6zu2mzzz96vfr295wztrttwklw7p5uhm3na4hy0sea50kv
    half_egld=500000000000000000
    bid_apt_fname="bid_apartment"
    erdpy --verbose contract call $contract_address --recall-nonce --pem=${buyer_pem_file} --gas-limit=50000000 --function=$bid_apt_fname --value=$half_egld --send --chain=D   
}
