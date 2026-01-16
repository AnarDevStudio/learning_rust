fn main() { 
    let a = 5;  //yeni deyisen a teyin edilir ve 5 qiymeti verilir
    let b = 10; //yeni deyisen b teyin edilir ve 10 qiymeti verilir
    let c = 4; //yeni deyisen c teyin edilir ve 4 qiymeti verilir
    let topla = a + b + c; // a, b ve c deyisenlerinin cemi topla deyisenine verilir
    println!("Cem: {}", topla); // topla deyiseninin qiymeti ekrana cap edilir

    let d = 20; //yeni deyisen d teyin edilir ve 20 qiymeti verilir
    d = 10; // xeta: deyisenlerin deyeri deyismir cunki onlar default olaraq const olaraq teyin edilir ❌
    let mut j = 15; //yeni deyisen j teyin edilir ve 15 qiymeti verilir
    j = 25; // j deyiseninin deyeri deyisir cunki o mut olaraq teyin edilib ✅

    // tipler:
    let intager: i32 = 30; // i32 tipinde deyisen teyin edilir yeni tam say saxlayir
    let onluq: f64 = 3.14; // f64 tipinde deyisen teyin edilir yeni onluq say saxlayir
    let boolean: bool = true; // bool tipinde deyisen teyin edilir icinde sadece 2 qiymet saxlaya biler: true(dogru) ve false(yanlis)
    let character: char = 'A'; // char tipinde deyisen teyin edilir yeni tek simvol saxlaya biler
    let string: &str = "Salam, Rust!"; // &str tipinde deyisen teyin edilir yeni metn saxlaya biler

    // coxlutipli deyisenler (tuples):
    let coxlutipli: (i32, f64, char) = (50, 6.28, 'B'); // coxlutipli deyisen teyin edilir yeni 3 ferqli tipde deyeri saxlaya biler
    println!("Coxlutipli deyisenin birinci elementi: {}", coxlutipli.0); // coxlutipli deyisenin birinci elementi ekrana cap edilir cixdi: 50

    // massivler (arrays):
    let massiv: [i32; 5] = [1, 2, 3, 4, 5]; // i32(tam say) tipinde 5 elementli massiv teyin edilir
    println!("Massivin ucuncu elementi: {}", massiv[2]); // massivin ucuncu elementi ekrana cap edilir cixdi: 3 not: massivlerin indeksleri 0-dan baslayir

    let mut massiv2 = [i32; 2]; // i32(tam say) tipinde 2 elementli massiv teyin edilir
    massiv2[0] = 10; // massiv2-nin birinci elementine 10 qiymeti verilir
    massiv2[1] = 20; // massiv2-nin ikinci elementine 20 qiymeti verilir
    println!("Massiv2-nin elementleri: {}, {}", massiv2[0], massiv2[1]); // massiv2-nin elementleri ekrana cap
    // not: tuples ve massivler arasindaki ferq: tuples ferqli tipde elementleri saxlaya biler, massivler ise sadece eyni tipde elementleri saxlaya biler
    // not: tuple mentiqi olaraq daha kompleks verilish strukturlarini saxlamaq ucun istifade olunur, massivler ise daha sade ve eyni tipde verilishleri saxlamaq ucun istifade olunur
}