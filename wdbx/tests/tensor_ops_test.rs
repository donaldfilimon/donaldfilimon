use wdbx::{add, sub, mul, div, matmul};
use mlx_rs::array;

#[test]
fn test_tensor_matmul() {
    // 2x3 matrix
    let a = array!([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    // 3x2 matrix
    let b = array!([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);
    let c = matmul(&a, &b);
    
    // Result should be 2x2:
    // [1*7+2*9+3*11, 1*8+2*10+3*12] = [7+18+33, 8+20+36] = [58, 64]
    // [4*7+5*9+6*11, 4*8+5*10+6*12] = [28+45+66, 32+50+72] = [139, 154]
    let expected = vec![58.0, 64.0, 139.0, 154.0];
    let result: Vec<f32> = c.as_slice::<f32>().to_vec();
    assert_eq!(result, expected);
    assert_eq!(c.shape(), &[2, 2]);
}


#[test]
fn test_tensor_add() {
    let a = array!([1.0, 2.0, 3.0]);
    let b = array!([4.0, 5.0, 6.0]);
    let c = add(&a, &b);
    
    let expected = vec![5.0, 7.0, 9.0];
    let result: Vec<f32> = c.as_slice::<f32>().to_vec();
    assert_eq!(result, expected);
}

#[test]
fn test_tensor_sub() {
    let a = array!([4.0, 5.0, 6.0]);
    let b = array!([1.0, 2.0, 3.0]);
    let c = sub(&a, &b);
    
    let expected = vec![3.0, 3.0, 3.0];
    let result: Vec<f32> = c.as_slice::<f32>().to_vec();
    assert_eq!(result, expected);
}

#[test]
fn test_tensor_mul() {
    let a = array!([1.0, 2.0, 3.0]);
    let b = array!([4.0, 5.0, 6.0]);
    let c = mul(&a, &b);
    
    let expected = vec![4.0, 10.0, 18.0];
    let result: Vec<f32> = c.as_slice::<f32>().to_vec();
    assert_eq!(result, expected);
}

#[test]
fn test_tensor_div() {
    let a = array!([4.0, 10.0, 18.0]);
    let b = array!([2.0, 2.0, 3.0]);
    let c = div(&a, &b);
    
    let expected = vec![2.0, 5.0, 6.0];
    let result: Vec<f32> = c.as_slice::<f32>().to_vec();
    assert_eq!(result, expected);
}
