#![allow(dead_code)]

use num_complex::Complex32;
use ndarray::{
    arr1, 
    arr2, 
    Array1, 
    Array2
};
use crate::chapter_1::{
    sum_complex,
    multiply_complex,
    complex_conjugate,
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
    let x = arr2(&[
        [Complex32::new(6.0,3.0), Complex32::new(0.0,0.0), Complex32::new(5.0,1.0), Complex32::new(4.0,0.0)],
        [Complex32::new(12.0,21.0), Complex32::new(0.0,0.0), Complex32::new(13.0,13.0), Complex32::new(12.0,8.0)]
    ]);

    let y = arr2(&[
        [Complex32::new(-2.0,-2.0), Complex32::new(9.0,4.0), Complex32::new(10.0,11.0), Complex32::new(0.0,2.0)],
        [Complex32::new(8.0,-9.0), Complex32::new(-12.0,-5.0), Complex32::new(9.0,-11.0), Complex32::new(-4.0,0.0)]
    ]);

    let expected = arr2(&[
        [Complex32::new(4.0,1.0), Complex32::new(9.0,4.0), Complex32::new(15.0,12.0), Complex32::new(4.0,2.0)],
        [Complex32::new(20.0,12.0), Complex32::new(-12.0,-5.0), Complex32::new(22.0,2.0), Complex32::new(8.0,8.0)]
    ]);

    assert_eq!(complex_matrix_add(x, y), expected);
}

#[test]
fn test_complex_matrix_inverse() {
    let y = arr2(&[
        [Complex32::new(-2.0,-2.0), Complex32::new(9.0,4.0), Complex32::new(10.0,11.0), Complex32::new(0.0,2.0)],
        [Complex32::new(8.0,-9.0), Complex32::new(-12.0,-5.0), Complex32::new(9.0,-11.0), Complex32::new(-4.0,0.0)]
    ]);

    let expected = arr2(&[
        [Complex32::new(2.0,2.0), Complex32::new(-9.0,-4.0), Complex32::new(-10.0,-11.0), Complex32::new(0.0,-2.0)],
        [Complex32::new(-8.0,9.0), Complex32::new(12.0,5.0), Complex32::new(-9.0,11.0), Complex32::new(4.0,0.0)]
    ]);

    assert_eq!(complex_matrix_inverse(y), expected);
}

#[test]
fn test_complex_matrix_scalar_multiply() {
    let scalar = Complex32::new(-5.0, 1.0);
    let x = arr2(&[
        [Complex32::new(2.0,2.0), Complex32::new(-9.0,-4.0), Complex32::new(-10.0,-11.0), Complex32::new(0.0,-2.0)],
        [Complex32::new(-8.0,9.0), Complex32::new(12.0,5.0), Complex32::new(-9.0,11.0), Complex32::new(4.0,0.0)]
    ]);

    let expected = arr2(&[
        [Complex32::new(-12.0,-8.0), Complex32::new(49.0,11.0), Complex32::new(61.0,45.0), Complex32::new(2.0,10.0)],
        [Complex32::new(31.0,-53.0), Complex32::new(-65.0,-13.0), Complex32::new(34.0,-64.0), Complex32::new(-20.0,4.0)]
    ]);

    assert_eq!(complex_matrix_scalar_multiply(x, scalar), expected);
}

// Programming Drill 2.2.2
// Write a function that accepts two complex matrices of the appropriate size.
// The function should do matrix multiplication and return the result.

pub fn complex_matrix_multiply(x: Array2<Complex32>, y: Array2<Complex32>) -> Array2<Complex32> {
    let answer = x.dot(&y);
    answer
}

#[test]
fn test_complex_matrix_multiply() {
    let x = arr2(&[
        [Complex32::new(6.0,3.0), Complex32::new(0.0,0.0), Complex32::new(5.0,1.0), Complex32::new(4.0,0.0)],
        [Complex32::new(12.0,21.0), Complex32::new(0.0,0.0), Complex32::new(13.0,13.0), Complex32::new(12.0,8.0)]
    ]);

    let y = arr2(&[
        [Complex32::new(-2.0,-2.0), Complex32::new(9.0,4.0)], 
        [Complex32::new(10.0,11.0), Complex32::new(0.0,2.0)],
        [Complex32::new(8.0,-9.0), Complex32::new(-12.0,-5.0)], 
        [Complex32::new(9.0,-11.0), Complex32::new(-4.0,0.0)]
    ]);
    
    let expected = arr2(&[
        [Complex32::new(79.0,-99.0), Complex32::new(-29.0,14.0)],
        [Complex32::new(435.0,-139.0), Complex32::new(-115.0,-16.0)]
    ]);

    assert_eq!(complex_matrix_multiply(x, y), expected);
}

// Programming Drill 2.4.1
// Write a function that accepts two complex vectors of length n and 
// calculates their inner product. 

pub fn complex_vector_dagger_op(x: Array1<Complex32>) -> Array1<Complex32> {
    let answer = x.reversed_axes().map(|xval| complex_conjugate(*xval));
    answer
}

pub fn complex_vector_inner_product(x: Array1<Complex32>, y: Array1<Complex32>) -> Array1<Complex32> {
    complex_vector_dagger_op(x) * y
}

#[test]
fn test_complex_vector_inner_product() {
    let x = arr1(&[Complex32::new(14.0, 4.0), Complex32::new(2.0, -3.0)]);
    let y = arr1(&[Complex32::new(-6.0, 2.0), Complex32::new(11.0, -15.0)]);
    let expected = arr1(&[Complex32::new(-76.0, 52.0), Complex32::new(67.0, 3.0)]);
    assert_eq!(complex_vector_inner_product(x, y), expected);
}

// Programming Drill 2.4.2
// Write a function that calculates the norm of a given complex vector.

pub fn complex_norm(x: Complex32) -> f32 {
    (x.re.powi(2) + x.im.powi(2)).sqrt()
}

pub fn complex_vector_norm(x: Array1<Complex32>) -> f32 {
    let sum_of_squares = x.fold(0.0, |acc, xval| acc + complex_norm(*xval).powi(2));
    sum_of_squares.sqrt()
}

#[test]
fn test_complex_vector_norm() {
    let x = arr1(&[
        Complex32::new(4.0, 3.0), 
        Complex32::new(6.0, -4.0), 
        Complex32::new(12.0, -7.0),
        Complex32::new(0.0, 13.0)
    ]);
    let expected = 439.0_f32.sqrt();
    assert_eq!(complex_vector_norm(x), expected);
}

// Programming Drill 2.4.3
// Write a function that calculates the distance of two given complex vectors.

pub fn complex_vector_distance(x: Array1<Complex32>, y: Array1<Complex32>) -> f32 {
    complex_vector_norm(x - y)
}

#[test]
fn test_complex_vector_distance() {
    let x = arr1(&[Complex32::new(14.0, 4.0), Complex32::new(2.0, -3.0)]);
    let y = arr1(&[Complex32::new(-6.0, 2.0), Complex32::new(11.0, -15.0)]);
    let expected = 629.0_f32.sqrt(); 

    assert_eq!(complex_vector_distance(x.clone(), x.clone()), 0.0);
    assert_eq!(complex_vector_distance(x, y), expected);
}

pub fn complex_matrix_dagger_op(x: Array2<Complex32>) -> Array2<Complex32> {
    let answer = x.reversed_axes().map(|xval| complex_conjugate(*xval));
    answer
}

#[test]
fn test_complex_matrix_dagger_op() {
    let x = let x = arr2(&[
        [Complex32::new(0.5,0.5), Complex32::new(0.,1./3.0_f32.sqrt()), Complex32::new(3./(2.*15.0_f32.sqrt()),1./(2.*15.0_f32.sqrt()))],
        [Complex32::new(-0.5,0.), Complex32::new(1./3.0_f32.sqrt(),0.), Complex32::new(4./(2.*15.0_f32.sqrt()),3./(2.*15.0_f32.sqrt()))],
        [Complex32::new(0.5,0.), Complex32::new(-1./3.0_f32.sqrt()), Complex32::new(0.,5./2.*15.0_f32.sqrt()))],
    ]);
    let expected = arr2(&[
        [Complex32::new(6.,3.), Complex32::new(0.,0.), Complex32::new(1.,0.)],
        [Complex32::new(2.,-12.), Complex32::new(5.,-2.1), Complex32::new(2.,-5.)],
        [Complex32::new(0.,19.), Complex32::new(17.,0.), Complex32::new(3.,4.5)],
    ]);
    assert_eq!(complex_matrix_dagger_op(x), expected);
}

pub fn is_matrix_hermitian(x: Array2<Complex32>) -> bool {
    assert!(x.is_square());
    complex_matrix_dagger_op(x.clone()) == x
}

#[test]
fn test_is_matrix_hermitian() {
    let x = arr2(&[
        [Complex32::new(7.0, 0.0), Complex32::new(6.0, 5.0)],
        [Complex32::new(6.0, -5.0), Complex32::new(-3.0, 0.0)]
    ]); 
    let y = arr2(&[
        [Complex32::new(7.0, 0.0), Complex32::new(6.0, 5.0)],
        [Complex32::new(6.0, 5.0), Complex32::new(3.0, 0.0)]
    ]);
    assert!(is_matrix_hermitian(x));
    assert!(!is_matrix_hermitian(y));
}

// Programming Drill 2.6.2
// Write a function that accepts a square matrix and tells if it is unitary.

pub fn is_matrix_unitary(x: Array2<Complex32>) -> bool {
    assert!(x.is_square());
    let identity = arr2(&[
        [Complex32::new(1.0, 0.0), Complex32::new(0.0, 0.0)],
        [Complex32::new(0.0, 0.0), Complex32::new(1.0, 0.0)]
    ]);
    complex_matrix_dagger_op(x.clone()) * x == identity
}

#[test]
fn test_is_matrix_unitary() {
    let x = arr2(&[
        [Complex32::new(0.5,0.5), Complex32::new(0.,1./3.0_f32.sqrt()), Complex32::new(3./(2.*15.0_f32.sqrt()),1./(2.*15.0_f32.sqrt()))],
        [Complex32::new(-0.5,0.), Complex32::new(1./3.0_f32.sqrt(),0.), Complex32::new(4./(2.*15.0_f32.sqrt()),3./(2.*15.0_f32.sqrt()))],
        [Complex32::new(0.5,0.), Complex32::new(-1./3.0_f32.sqrt()), Complex32::new(0.,5./2.*15.0_f32.sqrt()))],
    ]);
    assert!(is_matrix_unitary(x));
}

// Programming Drill 2.7.1
// Write a function that accepts two matrices and constructs their tensor product.

pub fn matrix_tensor_product(x: &Array2<Complex32>, y: &Array2<Complex32>) -> Array2<Complex32> {
    assert!(x.is_square() && y.is_square());

    let xdim = x.shape()[0];
    let ydim = y.shape()[0];
    let zdim = xdim * ydim;
    let mut answer = Array2::zeros((zdim, zdim));

    for (mut chunk, elem) in answer
        .exact_chunks_mut((ydim, ydim))
        .into_iter()
        .zip(x.iter()) {
            chunk.assign(&(*elem * y))
        }

    answer
}

#[test]
fn test_matrix_tensor_product() {
    let x = arr2(&[
        [Complex32::new(3.0,2.0), Complex32::new(5.0,-1.0), Complex32::new(0.0,2.0)],
        [Complex32::new(0.0,0.0), Complex32::new(12.0,0.0), Complex32::new(6.0,-3.0)],
        [Complex32::new(2.0,0.0), Complex32::new(4.0,4.0), Complex32::new(9.0,3.0)],
    ]);

    let y = arr2(&[
        [Complex32::new(1.0,0.0), Complex32::new(3.0,4.0), Complex32::new(5.0,-7.0)],
        [Complex32::new(10.0,2.0),Complex32::new(6.0,0.0),Complex32::new(2.0,5.0)],
        [Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(2.0,9.0)],
    ]);

     let expected = arr2(&[
         [Complex32::new(3.,2.),Complex32::new(1.,18.),Complex32::new(29.,-11.),Complex32::new(5.,-1.),Complex32::new(19.,17.),Complex32::new(18.,-40.),Complex32::new(0.,2.),Complex32::new(-8.,6.),Complex32::new(14.,10.)],
         [Complex32::new(26.,26.),Complex32::new(18.,12.),Complex32::new(-4.,19.),Complex32::new(52.,0.),Complex32::new(30.,-6.),Complex32::new(15.,23.),Complex32::new(-4.,20.),Complex32::new(0.,12.),Complex32::new(-10.,4.)],
         [Complex32::new(0.,0.),Complex32::new(3.,2.),Complex32::new(-12.,31.),Complex32::new(0.,0.),Complex32::new(5.,-1.),Complex32::new(19.,43.),Complex32::new(0.,0.),Complex32::new(0.,2.),Complex32::new(-18.,4.)],
         [Complex32::new(0.,0.),Complex32::new(0.,0.),Complex32::new(0.,0.),Complex32::new(12.,0.),Complex32::new(36.,48.),Complex32::new(60.,-84.),Complex32::new(6.,-3.),Complex32::new(30.,15.),Complex32::new(9.,-57.)],
         [Complex32::new(0.,0.),Complex32::new(0.,0.),Complex32::new(0.,0.),Complex32::new(120.,24.),Complex32::new(72.,0.),Complex32::new(24.,60.),Complex32::new(66.,-18.),Complex32::new(36.,-18.),Complex32::new(27.,24.)],
         [Complex32::new(0.,0.),Complex32::new(0.,0.),Complex32::new(0.,0.),Complex32::new(0.,0.),Complex32::new(12.,0.),Complex32::new(24.,108.),Complex32::new(0.,0.),Complex32::new(6.,-3.),Complex32::new(39.,48.)],
         [Complex32::new(2.,0.),Complex32::new(6.,8.),Complex32::new(10.,-14.),Complex32::new(4.,4.),Complex32::new(-4.,28.),Complex32::new(48.,-8.),Complex32::new(9.,3.),Complex32::new(15.,45.),Complex32::new(66.,-48.)],
         [Complex32::new(20.,4.),Complex32::new(12.,0.),Complex32::new(4.,10.),Complex32::new(32.,48.),Complex32::new(24.,24.),Complex32::new(-12.,28.),Complex32::new(84.,48.),Complex32::new(54.,18.),Complex32::new(3.,51.)],
         [Complex32::new(0.,0.),Complex32::new(2.,0.),Complex32::new(4.,18.),Complex32::new(0.,0.),Complex32::new(4.,4.),Complex32::new(-28.,44.),Complex32::new(0.,0.),Complex32::new(9.,3.),Complex32::new(-9.,87.)],
     ]);

    assert_eq!(matrix_tensor_product(&x, &y), expected);
}

