#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait BuySell {
    
    #[init]
    fn init(&self, token_identifier: EgldOrEsdtTokenIdentifier) {
        require!(token_identifier.is_valid(), "Invalid token provided");
        self.block_token_id().set(token_identifier);
    }
    
    /**
     * Any address can send an nft, from the collection accepted by this contract.
     * The nft will be sold by bid_apartment method when 2 addresses will bid
     */
    #[endpoint]
    #[payable("*")]
    fn send_nft(&self) {
        let caller: ManagedAddress = self.blockchain().get_caller();
        
        require!(self.bid_nft().is_empty() == true, "We need to complete previous bid(sell previous nft) before to send another nft for biding");

        let nft: EsdtTokenPayment = self.call_value().single_esdt();
        let (token, _, amount) = nft.clone().into_tuple();

        require!(amount == 1, "NFT amount should be 1");
        require!(token == self.block_token_id().get(), "We don't accept apartments from another block (token identifier)");
        
        self.bid_nft().set(nft);
        self.nft_sender().set(caller);
    }

    /**
     * Possibility to sell the nft if the destination already deposit the egold in this contract
    */
    #[endpoint]
    #[payable("*")]
    fn sell_apartment(&self, price: BigUint, destination: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        let (token, token_nonce, amount) = self.call_value().single_esdt().into_tuple();
        
        require!(amount == 1, "NFT amount should be 1");
        require!(token == self.block_token_id().get(), "We don't accept apartments from another block (token identifier)");
        
        let money: BigUint = self.deposit(&destination).get();
        let n_prev_ap: BigUint = self.n_apartments_history(&destination).get() * 500000000000000000u64; // half egld for each apartment already buyed
        
        let new_price: BigUint = if &price > &n_prev_ap { price.clone() - &n_prev_ap } else { BigUint::zero() } as BigUint;
        require!(money >= new_price, "The buyer should have sufficient egld in deposit");

        let rest: BigUint = &money - &new_price;

        self.send().direct_esdt(&destination, &token, token_nonce, &amount);
        self.send().direct_egld(&destination, &rest);
        self.send().direct_egld(&caller, &new_price);

        self.deposit(&destination).clear();
        
        self.n_apartments_history(&destination).update(|n_apartments_history| *n_apartments_history += 1u32);
    }

    /**
     * Deposit egold in this contract to be used later to buy a nft
     */
    #[endpoint]
    #[payable("EGLD")]
    fn buy_apartment(&self) {
        let payment = self.call_value().egld_value();

        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += payment);
    }

    /**
    * Allow 2 addresses to fight for a nft.
    * The address with the highest egld sent will win
    */
    #[endpoint]
    #[payable("EGLD")]
    fn bid_apartment(&self) {
        require!(self.bid_nft().is_empty() == false, "We need someone to sell an apartment before buying");


        let current_caller: ManagedAddress = self.blockchain().get_caller();
        let current_bid: BigUint = self.call_value().egld_value();
        
        if self.max_bid_value().is_empty() == true {
            self.max_bid_value().set(current_bid);
            self.max_bid_address().set(current_caller);
        } else {
            let (token, token_nonce, amount) = self.bid_nft().get().into_tuple();
            let prev_bid: BigUint = self.max_bid_value().get();
            let prev_caller: ManagedAddress = self.max_bid_address().get();
            let nft_sender = self.nft_sender().get();

            if prev_bid > current_bid {
                self.send().direct_egld(&current_caller, &current_bid);
                self.send().direct_egld(&nft_sender, &prev_bid);
                self.send().direct_esdt(&prev_caller, &token, token_nonce, &amount);
            } else {
                self.send().direct_egld(&prev_caller, &prev_bid);
                self.send().direct_egld(&nft_sender, &current_bid);
                self.send().direct_esdt(&current_caller, &token, token_nonce, &amount);
            }

            self.nft_sender().clear();
            self.max_bid_value().clear();
            self.max_bid_address();
            self.bid_nft().clear();
        }
    }

    /**
     * Get money back if no longer need to buy a nft
     */
    #[endpoint]
    fn get_money_back(&self) {
        let caller = self.blockchain().get_caller();
        let money = self.deposit(&caller).get();

        if money > 0u32 {
            self.deposit(&caller).clear();
            self.send().direct_egld(&caller, &money);
        }
    }

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getNoApartments)]
    #[storage_mapper("apartments")]
    fn n_apartments_history(&self, client: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getBlockTokenIdentifier)]
    #[storage_mapper("tokenIdentifier")]
    fn block_token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getBidAddress)]
    #[storage_mapper("maxBidAddress")]
    fn max_bid_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getBidValue)]
    #[storage_mapper("maxBidValue")]
    fn max_bid_value(&self) -> SingleValueMapper<BigUint>;

    #[view(getBidNft)]
    #[storage_mapper("bigNft")]
    fn bid_nft(&self) -> SingleValueMapper<EsdtTokenPayment>;

    #[view(getNftSender)]
    #[storage_mapper("nftSender")]
    fn nft_sender(&self) -> SingleValueMapper<ManagedAddress>;
}
