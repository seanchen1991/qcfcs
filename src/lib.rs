#![allow(dead_code)]

use num_complex::Complex32;

fn sum_complex(x: Complex32, y: Complex32) -> Complex32 {
    Complex32::new(x.re + y.re, x.im + y.im)
}

fn multiply_complex(x: Complex32, y: Complex32) -> Complex32 {
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

fn subtract_complex(x: Complex32, y: Complex32) -> Complex32 {
    Complex32::new(x.re - y.re, x.im - y.im)
}

fn divide_complex(x: Complex32, y: Complex32) -> Complex32 {
    let denominator = x.im.powi(2) + y.im.powi(2);
    assert!(denominator != 0.0);
    
    let n1 = x.re * x.im + y.re * y.im;
    let n2 = x.im * y.re - x.re * y.im;
    
    Complex32::new(n1 / denominator, n2 / denominator)
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

