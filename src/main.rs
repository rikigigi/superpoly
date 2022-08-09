use std::io;
use superpoly::poly::Poly;


fn read_number<T : std::str::FromStr >() -> T {
    let mut input = String::new();
    loop {
      io::stdin()
         .read_line(&mut input)
         .expect("Reading of the input failed");
      let n : T = match input.trim().parse() {
             Ok(num) => num,
             Err(_) => { println!("You must enter a number with correct type. You entered'{input}'");  continue },
          };
          return n;
    }
}



fn main() {

    println!("Input height at 0 and height at 3");
    let h0 : f64 = read_number();
    let h1 : f64 = read_number();
    println!("Input number of second derivative points");
    let nd2 : usize = read_number();
    println!("Input second derivative point positions and values");

    let mut p2x : Vec<f64> = Vec::new();
    let mut p2y : Vec<f64> = Vec::new();
    for _i in 0..nd2{
        p2x.push(read_number::<f64>());
        p2y.push(read_number::<f64>());
    }

    let poly = Poly::new(p2x,p2y,h0,h1);


    println!("values of coefficients a_2 ... a_{}\n{}",4+nd2,poly.a);

    for i in -350..351 {
        let x : f64 = i as f64 / 100.0 ;
        let (y,y1,y2) = poly.calc(x);
        println!("{x} {y} {y1} {y2}");
    }
}
