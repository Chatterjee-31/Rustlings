// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount.

fn calculate_apple_price(num_apples: u32)->u32{
    if num_apples < 60{
        num_apples*2
    }else{
        num_apples
    }
}
 
// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}