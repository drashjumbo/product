use soroban_sdk::{contractimpl, Bytes, BytesN, Env, Symbol, Address, IntoVal};

#[derive(Clone)]
struct Product {
    name: String,
    description: String,
    price: i32, 
    seller: Address,
}

pub struct ProductListingContract;

#[contractimpl]
impl ProductListingContract {
    // Add a new product listing
    pub fn list_product(e: Env, seller: Address, name: String, description: String, price: i32) {
        let product_id = get_next_product_id(&e);
        e.data().set(product_id, Product { seller, name, description, price });
    }

    
    pub fn update_product(e: Env, product_id: Bytes, name: String, description: String, price: i32) {
        let product = e.data().get::<Product>(product_id).unwrap().unwrap();

        if product.seller != e.invoker() {
            panic!("Only the seller can update a product");
        }

        e.data().set(product_id, Product { seller: product.seller, name, description, price });
    }


    pub fn delete_product(e: Env, product_id: Bytes) {
        let product = e.data().get::<Product>(product_id).unwrap().unwrap();

        if product.seller != e.invoker() {
            panic!("Only the seller can delete a product");
        }

        e.data().remove(product_id);
    }

    
    pub fn get_product(e: Env, product_id: Bytes) -> Product {
        e.data().get::<Product>(product_id).unwrap().unwrap()
    }
}


fn get_next_product_id(e: &Env) -> Bytes {
    
}

