// 所有権と関数
fn main() {
    let s = String::from("hello")  // sがスコープに入る

    takes_ownership(s);  // sの値が関数にムーブされる
    // sの値はもう有効ではない

    let x = 5;
    makes_copy(x)  // xの値がmakes_copyにムーブされる
    // xはint(uint)なのでコピーされるので、この後も有効
    
}

fn takes_ownership(some_string: String){  // some_stringがスコープに入る
    println!("{}", some_string);
}  // some_stringがスコープを抜けて, `drop`がよばれる. メモリが解放される

fn makes_copy(some_integer: i32) {  // soome_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない(copyなので)