#![allow(dead_code)]

use num_complex::Complex32;
use ndarray::{arr2, Array2};
use crate::chapter_1::{
    sum_complex,
    multiply_complex,
};

// Programming Drill 2.1.1
// Write three functions that perform the addition,
// inverse, and scalar multiplication operations for 
// C_n (the complex plane), i.e. write a function that
// accepts the appropriate input for each of the 
// operations and outputs the vector.

pub fn complex_vector_add(x: Vec<Complex32>, y: Vec<Complex32>) -> Vec<Complex32> {
    assert!(x.len() == y.len());

    let answer: Vec<Complex32> = x.iter()
        .zip(&y)
        .map(|(xval, yval)| sum_complex(*xval, *yval))
        .collect();

    answer
}

pub fn complex_inverse(x: Complex32) -> Complex32 {
    Complex32::new(-x.re, -x.im)
}

pub fn complex_vector_inverse(x: Vec<Complex32>) -> Vec<Complex32> {
    let answer: Vec<Complex32> = x.iter()
        .map(|xval| complex_inverse(*xval))
        .collect();

    answer
}

pub fn complex_vector_scalar_multiply(x: Vec<Complex32>, y: Complex32) -> Vec<Complex32> {
    let answer: Vec<Complex32> = x.iter()
        .map(|xval| multiply_complex(*xval, y))
        .collect();

    answer
}

#[test]
fn test_complex_vector_add() {
    let v = vec![
        Complex32::new(6.0, -4.0), 
        Complex32::new(7.0, 3.0),
        Complex32::new(4.2, -8.1),
        Complex32::new(0.0, -3.0)
    ]; 

    let w = vec![
        Complex32::new(16.0, 2.3),
        Complex32::new(0.0, -7.0),
        Complex32::new(6.0, 0.0),
        Complex32::new(0.0, -4.0)
    ];

    let expected = vec![
        Complex32::new(22.0, -1.7),
        Complex32::new(7.0, -4.0),
        Complex32::new(10.2, -8.1),
        Complex32::new(0.0, -7.0)
    ];

    assert_eq!(complex_vector_add(v, w), expected);
}

#[test]
fn test_complex_vector_inverse() {
    let v = vec![
        Complex32::new(6.0, -4.0), 
        Complex32::new(7.0, 3.0),
        Complex32::new(4.2, -8.1),
        Complex32::new(0.0, -3.0)
    ];

    let expected = vec![
        Complex32::new(-6.0, 4.0), 
        Complex32::new(-7.0, -3.0),
        Complex32::new(-4.2, 8.1),
        Complex32::new(0.0, 3.0)
    ];

    assert_eq!(complex_vector_inverse(v), expected);
}

#[test]
fn test_complex_vector_scalar_multiply() {
    let scalar = Complex32::new(3.0, 2.0);
    let v = vec![
        Complex32::new(6.0, 3.0),
        Complex32::new(0.0, 0.0),
        Complex32::new(5.0, 1.0),
        Complex32::new(4.0, 0.0)
    ];

    let expected = vec![
        Complex32::new(12.0, 21.0),
        Complex32::new(0.0, 0.0),
        Complex32::new(13.0, 13.0),
        Complex32::new(12.0, 8.0)
    ];

    assert_eq!(complex_vector_scalar_multiply(v, scalar), expected);
}

// Programming Drill 2.2.1
// Convert your functions from the last exercise so that instead
// of accepting elements of C_n, they accept elements of C_mxn

pub fn complex_matrix_add(x: Array2<Complex32>, y: Array2<Complex32>) -> Array2<Complex32> {
    let answer = x + y;
    answer 
}

pub fn complex_matrix_inverse(x: Array2<Complex32>) -> Array2<Complex32> {
    let answer = x.map(|xval| complex_inverse(*xval));
    answer
}

pub fn complex_matrix_scalar_multiply(x: Array2<Complex32>, y: Complex32) -> Array2<Complex32> {
    let answer = y * x;
    answer
}

#[test]
fn test_complex_matrix_add() {
    let x = arr2(&[[Complex32::new(6.0,3.0), Complex32::new(0.0,0.0), Complex32::new(5.0,1.0), Complex32::new(4.0,0.0)],
                   [Complex32::new(12.0,21.0), Complex32::new(0.0,0.0), Complex32::new(13.0,13.0), Complex32::new(12.0,8.0)]]);

    let y = arr2(&[[Complex32::new(-2.0,-2.0), Complex32::new(9.0,4.0), Complex32::new(10.0,11.0), Complex32::new(0.0,2.0)],
                   [Complex32::new(8.0,-9.0), Complex32::new(-12.0,-5.0), Complex32::new(9.0,-11.0), Complex32::new(-4.0,0.0)]]);

    let expected = arr2(&[[Complex32::new(4.0,1.0), Complex32::new(9.0,4.0), Complex32::new(15.0,12.0), Complex32::new(4.0,2.0)],
                         [Complex32::new(20.0,12.0), Complex32::new(-12.0,-5.0), Complex32::new(22.0,2.0), Complex32::new(8.0,8.0)]]);

    assert_eq!(complex_matrix_add(x, y), expected);
}

#[test]
fn test_complex_matrix_inverse() {
    let y = arr2(&[[Complex32::new(-2.0,-2.0), Complex32::new(9.0,4.0), Complex32::new(10.0,11.0), Complex32::new(0.0,2.0)],
                   [Complex32::new(8.0,-9.0), Complex32::new(-12.0,-5.0), Complex32::new(9.0,-11.0), Complex32::new(-4.0,0.0)]]);

    let expected = arr2(&[[Complex32::new(2.0,2.0), Complex32::new(-9.0,-4.0), Complex32::new(-10.0,-11.0), Complex32::new(0.0,-2.0)],
                   [Complex32::new(-8.0,9.0), Complex32::new(12.0,5.0), Complex32::new(-9.0,11.0), Complex32::new(4.0,0.0)]]);

    assert_eq!(complex_matrix_inverse(y), expected);
}

#[test]
fn test_complex_matrix_scalar_multiply() {
    let scalar = Complex32::new(-5.0, 1.0);
    let x = arr2(&[[Complex32::new(2.0,2.0), Complex32::new(-9.0,-4.0), Complex32::new(-10.0,-11.0), Complex32::new(0.0,-2.0)],
                   [Complex32::new(-8.0,9.0), Complex32::new(12.0,5.0), Complex32::new(-9.0,11.0), Complex32::new(4.0,0.0)]]);

    let expected = arr2(&[[Complex32::new(-12.0,-8.0), Complex32::new(49.0,11.0), Complex32::new(61.0,45.0), Complex32::new(2.0,10.0)],
                         [Complex32::new(31.0,-53.0), Complex32::new(-65.0,-13.0), Complex32::new(34.0,-64.0), Complex32::new(-20.0,4.0)]]);

    assert_eq!(complex_matrix_scalar_multiply(x, scalar), expected);
}
