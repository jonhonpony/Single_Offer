use soroban_sdk::Env;
#[no_std]
use soroban_sdk::{
contract,contractimpl,contracttype,token,unwrap::UnwrapOptimized, Address


};

#[derive(Clone)]
#[contracttype]// Rust'ta öznitelikler (attributes) genellikle hemen altındaki yapı (struct, enum, function, modül, vb.) için geçerlidir. Yani, öznitelikler sadece doğrudan uygulandıkları öğeye etki ederler ve tüm projeye etki etmezler
pub enum DataKey {
    Offer,

    
}

#[derive(Clone)] //Bu öznitelik, Offer yapısının Clone trait'ini otomatik olarak türetir. Bu, Offer yapısının kopyalanabilir olmasını sağlar. Yani, Offer yapısının bir örneğini başka bir örneğe kopyalayabilirsiniz.
#[contracttype]//Bu öznitelik, muhtemelen bir özel özniteliktir ve belirli bir crate veya framework tarafından tanımlanmış olabilir. Bu tür öznitelikler, genellikle belirli bir işlevselliği veya davranışı belirtmek için kullanılır.Örneğin, bir akıllı sözleşme framework'ü, belirli veri türlerini veya işlevleri işaretlemek için bu tür öznitelikleri kullanabilir. Bu durumda, #[contracttype] özniteliği Offer yapısının bir akıllı sözleşme türü olarak tanımlanmasını sağlayabilir.
pub struct Offer { // pub : (public) olarak erişilebilir olduğunu belirtir.
    pub Seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_price:u32, // -li int de kullanılabiliyor böyle
    pub buy_price:u32,

}

// sözleşmeye geçtiğimizi belli etmek için  #[contract] özniteliğini kullanıyoruz.
#[contract]
pub struct SingleOffer{}

#[contractimpl]
impl SingleOffer {
    pub fn create(e: Env, seller: Address, sell_token: Address, buy_token: Address, sell_price: u32, buy_price: u32) {
      if e.storage().instance().has(&DataKey::Offer){
        panic!("Offer already exists");

      }
      if buy_price == 0 || sell_price == 0 {
        panic!("Price cannot be 0");
      }
      seller.require_auth();
      write_offer(
        &e,
        Offer{
          Seller: seller,
          sell_token,
          buy_token,
          sell_price,
          buy_price,
        },
    );
      
  }

  pub fn trade (e: Env, buyer: Address, buy_amount :i128, mins_sell_token_amount: i128) { //Rust'ta &e ifadesi, e değişkenine bir referans oluşturur. Referanslar, bir değişkenin değerine doğrudan erişmek yerine, o değişkenin bellekteki adresine erişim sağlar. Bu, veri kopyalamadan veri üzerinde işlem yapmayı mümkün kılar ve performans açısından avantaj sağlar.

    buyer.require_auth();
    let offer = load_offer(&e);
    let sell_token_amount = token::Client::new(&e, &offer.sell_token);
    let buy_token_amount = token::Client::new(&e, &offer.buy_token);

    let sell_amount=buy_token_amount.checked_mul(offer.sell_price as i128).unwrap_optimized()/offer.buy_price as i128;

    if sell_token_amount < min_sell_token_amount{
        panic!("Not enough sell token amount");
    }

    let contract = e.current_contact_adress();

    buy_token_client.transfer(&buyer,&contract, &buy_token_amount); 
    sell_token_client.transfer(&contract, &offer.Seller, &sell_token_amount);
    buy_token_client.transfer(&contract, &offer.Seller, &buy_token_amount);

  
  }
  pub fn withdraw(e: Env, token: Address, amount: i128){
    let offer = load_offer(&e);
    offer.Seller.require_auth();
   token::Client::new(&e,&token).transfer(
        &e.current_contact_adress(),
        &offer.Seller,
        &amount
    );
   
  }

  pub fn updt_price(e: Env, sell_price: u32, buy_price: u32){

        if buy_price == 0 || sell_price == 0 {
            panic!("Price cannot be 0");
          }

          let mut offer = load_offer(&e);
          offer.Seller.require_auth();
          offer.sell_price = sell_price;
          write_offer(&e, offer);   
    }

    //Returns the current state of the offer
    pub fn get_offer(e: Env) -> Offer {
        load_offer(&e)
    }

}

fn load_offer(e: &Env) -> Offer {
    e.storage().instance().get(&DataKey::Offer).unwrap().unwrap()
}

fn write_offer(e: &Env, offer: Offer) {
    e.storage().instance().set(&DataKey::Offer, &offer);
}

mod test; // test modülüne geçiş yaparız.