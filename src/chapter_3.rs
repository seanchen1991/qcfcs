#![allow(dead_code)]

use num_traits;
use ndarray::{
    arr1,
    arr2,
    Array1,
    Array2,
};

// Programming Drill 3.1.1
// Write a program that performs the marble experiment. The program should allow
// the user to enter a boolean matrix that describes the ways that the marbles
// move. Make sure that the matrix follows our requirements. The user should also
// be permitted to enter a starting state of how many marbles are on each vertex.
// Then the user enters how many time clicks she wants to proceed. The program
// should then calculate and output the state of the system after those time 
// clicks. We will make changes to this program later in the chapter.

pub fn matrix_pow<T>(x: Array2<T>, mut n: u32) -> Array2<T> 
    where T: num_traits::Zero + std::ops::Mul<Output=T> + Copy
{
    let result =     
}

pub fn marble_exp1(M: Array2<i32>, X: Array1<i32>, n: u32) -> Array1<i32> {
    // perform matrix multiplication on M with itself n times
    // perform Mn.dot(X) to get the desired one-dimensional state vector
}
