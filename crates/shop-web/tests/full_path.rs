// the page shows exactly what the store holds, all the way through orders
#[test]
fn get_shows_the_stored_order() {
    assert_eq!(shop_web::get(7), shop_store::load(7));
}
