#![allow(dead_code)]

use num_complex::Complex32;

// Programming Drill 1.1.1
// Write a program that accepts two complex numbers and 
// outputs their sum and product.

pub fn sum_complex(x: Complex32, y: Complex32) -> Complex32 {
    Complex32::new(x.re + y.re, x.im + y.im)
}

pub fn multiply_complex(x: Complex32, y: Complex32) -> Complex32 {
    // multiply by -1 since we're squaring i
    let re = x.re * y.re + x.im * y.im * -1.0;
    let im = x.re * y.im + x.im * y.re;
    Complex32::new(re, im)
}

#[test]
fn test_sum_complex() {
    let x = Complex32::new(1.0, 3.0);
    let y = Complex32::new(2.0, 4.0);
    let z = sum_complex(x, y);
    assert_eq!(z.re, 3.0);
    assert_eq!(z.im, 7.0);
}

#[test]
fn test_multiply_complex() {
    let x = Complex32::new(3.0, -1.0);
    let y = Complex32::new(1.0, 4.0);
    let z = multiply_complex(x, y);
    assert_eq!(z.re, 7.0);
    assert_eq!(z.im, 11.0);
}

// Programming Drill 1.2.1
// Take the program that you wrote in the last programming 
// drill and make it also perform subtraction and division
// of complex numbers. In addition, let the user enter a
// complex number and have the computer return its modulus
// and conjugate.

pub fn subtract_complex(x: Complex32, y: Complex32) -> Complex32 {
    Complex32::new(x.re - y.re, x.im - y.im)
}

pub fn divide_complex(x: Complex32, y: Complex32) -> Complex32 {
    let denominator = x.im.powi(2) + y.im.powi(2);
    assert!(denominator != 0.0);
    
    let n1 = x.re * x.im + y.re * y.im;
    let n2 = x.im * y.re - x.re * y.im;
    
    Complex32::new(n1 / denominator, n2 / denominator)
}

pub fn complex_modulus(x: Complex32) -> f32 {
    let sq_sum = x.re.powi(2) + x.im.powi(2);
    sq_sum.sqrt()
}

pub fn complex_conjugate(x: Complex32) -> Complex32 {
    Complex32::new(x.re, -x.im)     
}

#[test]
fn test_subtract_complex() {
    let x = Complex32::new(1.0, 3.0);
    let y = Complex32::new(2.0, 4.0);
    let z = subtract_complex(x, y);
    assert_eq!(z.re, -1.0);
    assert_eq!(z.im, -1.0);
}

#[test]
fn test_divide_complex() {
    let x = Complex32::new(-2.0, 1.0);
    let y = Complex32::new(1.0, 2.0);
    let z = divide_complex(x, y);
    assert_eq!(z.re, 0.0);
    assert_eq!(z.im, 1.0);
}

#[test]
fn test_complex_modulus() {
    let x = Complex32::new(4.0, -3.0);
    assert_eq!(complex_modulus(x), 5.0)
}

#[test]
fn test_complex_conjugate() {
    let x = Complex32::new(4.0, -3.0);
    let conjugate = complex_conjugate(x);
    assert_eq!(conjugate.re, 4.0);
    assert_eq!(conjugate.im, 3.0);
}

// Programming Drill 1.3.1
// Write a program that converts a complex number 
// from its Cartesian representation to its polar
// representation and vice versa.

pub fn cartesian_to_polar(x: Complex32) -> (f32, f32) {
    let theta = x.im.atan2(x.re);
    (complex_modulus(x), theta)
}

pub fn polar_to_cartesian(ro: f32, theta: f32) -> Complex32 {
    Complex32::new(ro * theta.cos(), ro * theta.sin())
}

#[test]
fn test_cartesian_to_polar() {
    let x = Complex32::new(4.0, 2.0);
    assert_eq!(cartesian_to_polar(x), x.to_polar());
}

#[test]
fn test_polar_to_cartesian() {
    use num_complex::Complex;
    assert_eq!(polar_to_cartesian(4.6, -3.14), Complex::from_polar(&4.6, &-3.14));
}
