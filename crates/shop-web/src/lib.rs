pub fn get(id: u32) -> String { shop_orders::show(id) }
pub fn get_fast(id: u32) -> String { shop_store::load(id) }   // nobody checks the order now
