fn main() {

    let degisken = 5;
    let mut degisken3 = 7;
    let degisken2 = "atakan";
    
    // bu şekilde olduğunda degisken değişkeni değişmez.
    // let yanına "mut" ifadesi eklersek değişebilir hale gelir.
    degisken3 = 10;

    println!("Hello, world!");
    println!("Sayi: {}", degisken);
    println!("Sayi: {}", degisken3);

       
}
