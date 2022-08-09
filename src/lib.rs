pub mod poly;

#[cfg(test)]
mod tests {
    use crate::poly::*;
    #[test]
    fn test_poly1(){
       let p = Poly::new(vec![3.0,4.55,4.3,5.02,7.1],vec![0.0,0.0,0.0,9.0,90.0], 1.0, 3.0);
       insta::assert_yaml_snapshot!(p.a.iter().map(|x| x).collect::<Vec<&f64> >() );
       insta::assert_yaml_snapshot!( (-350..351).map(|x| p.calc(x as f64 /100.0 ).0 ).collect::<Vec<f64> >());
       insta::assert_yaml_snapshot!( (-350..351).map(|x| p.calc(x as f64 /100.0 ).1 ).collect::<Vec<f64> >());
       insta::assert_yaml_snapshot!( (-350..351).map(|x| p.calc(x as f64 /100.0 ).2 ).collect::<Vec<f64> >());


    }
}
