use std::io;
use nalgebra::{DMatrix,DVector};


fn big_poly_linear_system_rows(deriv2_x: Vec<f64> ) -> Vec< Vec<f64> > {

    let n = deriv2_x.len()+3;
    let mut coeff : Vec<Vec<f64>> = Vec::new();
    coeff.resize(n,Vec::new());
    let mut two_pow3 : f64 = 8.0;
    let mut x2m1 : Vec<f64> = Vec::new() ;
    x2m1.resize(deriv2_x.len(),1.0);
    for i_ in 0..n {
        let i = i_+2;
        let ifloat : f64 = i as f64;
        coeff[0].push( if i%2==0 {1.0} else {-1.0} );
        coeff[2].push( ifloat * two_pow3  );
        two_pow3 *= 8.0;
        coeff[1].push( two_pow3);
        for j in 0..deriv2_x.len() {
            let a = x2m1[j]*(ifloat-1.0)*deriv2_x[j];
            x2m1[j]*=(deriv2_x[j]*deriv2_x[j]-1.0);
            let b = x2m1[j];
            coeff[3+j].push(2.0*ifloat*(a+b));
        }
    }
    coeff

}

fn big_poly_calc(x : f64, param : &DVector<f64> ) -> f64 {

    let mut x2 = x*x-1.0;
    let mut y = 0.0;
    x2*=x2;
    for i in 0..param.len(){
        y+=param[i]*x2;
        x2*=x*x-1.0;
    }
    y
}


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

    let system = big_poly_linear_system_rows(p2x);

    let mut mat = DMatrix::<f64>::zeros(system.len(),system.len());
    let mut b = DVector::<f64>::zeros(system.len());

    for i in 0..(3+nd2){
        for j in 0..(3+nd2){
            let v = system[i][j];
            mat[(i,j)]=v;
        }
    }

    b[0]=h0;
    b[1]=h1;
    b[2]=0.0;
    for i in 0..nd2{
        b[2+i]=p2y[i];
    }

    println!("{}",mat);
    println!("{}",b);

    let decomp = mat.lu();
    let xb = decomp.solve(&b).expect("Linear solution of the system failed");
    println!("values of coefficiente a_2 ... a_{}\n{}",4+nd2,xb);

    for i in -350..351 {
        let x : f64 = i as f64 / 100.0 ;
        let px3 = big_poly_calc(x,&xb);
        println!("{x} {px3}");
    }
}
