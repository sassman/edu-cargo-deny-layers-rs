// the page shows exactly what a store holds, all the way through orders
#[test]
fn get_shows_the_stored_order() {
    assert_eq!(shop_web::get(7), shop_store_in_mem::load(7));
}
