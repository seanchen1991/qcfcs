#![allow(dead_code)]

use ndarray:: {
    arr1,
    arr2,
    Array1,
    Array2,
};

// Programming Drill 3.1.1
// Write a program that performs the marble experiment. The program should 
// allow the user to enter a boolean matrix that describes the ways that 
// marbles move. Make sure that the matrix follows our requirements. The
// user should also be permitted to enter a starting state of how many 
// marbles are on each vertex. Then the user enters how many time clicks
// she wants to proceed. The program should then calculate and output the
// state of the system after those time clicks. We will make changes to
// this program later in the chapter.

pub fn int_matrix_pow(x: Array2<u32>, n: u32) -> Array2<u32> {
    assert!(n > 0);

    let mut result = x.clone();
    (1..n).for_each(|_| result = result.dot(&x));
    result
}

pub fn marble_exp1(m: Array2<u32>, x: Array1<u32>, n: u32) -> Array1<u32> {
    let changes = int_matrix_pow(m, n);
    let answer = changes.dot(&x);
    answer
}

#[test]
fn test_int_matrix_pow() {
    let m = arr2(&[
        [0,0,0,0,0,0],
        [0,0,0,0,0,0],
        [0,1,0,0,0,1],
        [0,0,0,1,0,0],
        [0,0,1,0,0,0],
        [1,0,0,0,1,0],
    ]);
    let m1 = m.clone();
    assert_eq!(int_matrix_pow(m.clone(), 1), m1);

    let m2 = arr2(&[
        [0,0,0,0,0,0],
        [0,0,0,0,0,0],
        [1,0,0,0,1,0],
        [0,0,0,1,0,0],
        [0,1,0,0,0,1],
        [0,0,1,0,0,0],
    ]);

    assert_eq!(int_matrix_pow(m.clone(), 2), m2);

    let m3 = arr2(&[
        [0,0,0,0,0,0],
        [0,0,0,0,0,0],
        [0,0,1,0,0,0],
        [0,0,0,1,0,0],
        [1,0,0,0,1,0],
        [0,1,0,0,0,1],
    ]);

    assert_eq!(int_matrix_pow(m.clone(), 3), m3);

    let m6 = arr2(&[
        [0,0,0,0,0,0],
        [0,0,0,0,0,0],
        [0,0,1,0,0,0],
        [0,0,0,1,0,0],
        [1,0,0,0,1,0],
        [0,1,0,0,0,1]
    ]);

    assert_eq!(int_matrix_pow(m.clone(), 6), m6);
}

#[test]
fn test_marble_exp1() {
    let m = arr2(&[
        [0,0,0,0,0,0],
        [0,0,0,0,0,0],
        [0,1,0,0,0,1],
        [0,0,0,1,0,0],
        [0,0,1,0,0,0],
        [1,0,0,0,1,0],
    ]);
    let x1 = arr1(&[6,2,1,5,3,10]);
    let y1 = arr1(&[0,0,12,5,1,9]);
    assert_eq!(marble_exp1(m.clone(), x1, 1), y1);

    let x2 = arr1(&[5,5,0,2,0,15]);
    let y2 = arr1(&[0,0,20,2,0,5]);
    assert_eq!(marble_exp1(m.clone(), x2, 1), y2);

    let a = arr2(&[
        [0,1,0,0,0,0,0,0,0],
        [1,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,1,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,1,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,1,0,1,0,0],
        [0,0,0,0,0,1,0,1,1],
    ]);
    let x = arr1(&[1,1,1,1,1,1,1,1,1]);

    let expected = arr1(&[1,1,0,1,0,0,0,0,6]);
    assert_eq!(marble_exp1(a.clone(), x.clone(), 1), arr1(&[1,1,0,1,0,1,0,2,3]));
    assert_eq!(marble_exp1(a.clone(), x.clone(), 2), expected);
    assert_eq!(marble_exp1(a.clone(), x.clone(), 4), expected);
}

// Programming Drill 3.2.1
// Modify your program from exercise 3.1.1 so that the entries in the
// matrices can be fractions as opposed to boolean values 

pub fn f_matrix_pow(x: Array2<f32>, n: u32) -> Array2<f32> {
    assert!(n > 0);

    let mut result = x.clone();
    (1..n).for_each(|_| result = result.dot(&x));
    result
}

pub fn marble_exp2(m: Array2<f32>, x: Array1<f32>, n: u32) -> Array1<f32> {
    let changes = f_matrix_pow(m, n);
    let answer= changes.dot(&x);
    answer
}
