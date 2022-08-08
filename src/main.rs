use std::io;
use nalgebra::{DMatrix,DVector};


struct Poly {
    deriv2_x : Vec<f64>,
    b : DVector<f64>,
    a : DVector<f64>,
}

impl Poly {

fn new( deriv2_x: Vec<f64>, deriv2_y: Vec<f64>, h0 : f64, h1 : f64 ) -> Poly {

    let n = deriv2_x.len()+3;
    let mut coeff = DMatrix::<f64>::zeros(n,n);
    let mut two_pow3 : f64 = 8.0;
    let mut x2m1  = DVector::<f64>::repeat(n,1.0) ;
    for i_ in 0..n {
        let i = i_+2;
        let ifloat : f64 = i as f64;
        coeff[(0,i_)]= if i%2==0 {1.0} else {-1.0} ;
        coeff[(2,i_)]= ifloat * two_pow3;
        two_pow3 *= 8.0;
        coeff[(1,i_)] = two_pow3;
        for j in 0..deriv2_x.len() {
            let a = x2m1[j]*(ifloat-1.0)*2.0*deriv2_x[j]*deriv2_x[j];
            x2m1[j]*=deriv2_x[j]*deriv2_x[j]-1.0;
            let b = x2m1[j];
            coeff[(3+j,i_)] = 2.0*ifloat*(a+b);
        }
    }
    let mut b = DVector::<f64>::zeros(n);
    b[0]=h0;
    b[1]=h1;
    b[2]=0.0;
    for i in 3..n{
        b[i]=deriv2_y[i-3];
    }
    let a = coeff.lu().solve(&b).expect("Linear solution of the system failed");
    Poly {
        deriv2_x,
        b,
        a
    }

}


fn calc(&self, x : f64 ) -> (f64,f64,f64) {

    let mut x2 = x*x-1.0;
    let mut x2p=1.0;
    let mut y = 0.0;
    let mut y1 = 0.0;
    let mut y2 = 0.0;
    for i in 0..self.a.len(){
        let i_s = (i+2) as f64;
        y2+=2.0*i_s*self.a[i]*((i_s-1.0)*2.0*x*x*x2p+x2);
        y1+=i_s*2.0*x*self.a[i]*x2;
        x2p=x2;
        x2*=x*x-1.0;
        y+=self.a[i]*x2;
    }
    (y,y1,y2)
}


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

    let poly = Poly::new(p2x,p2y,h0,h1);


    println!("values of coefficiente a_2 ... a_{}\n{}",4+nd2,poly.a);

    for i in -350..351 {
        let x : f64 = i as f64 / 100.0 ;
        let (y,y1,y2) = poly.calc(x);
        println!("{x} {y} {y1} {y2}");
    }
}
