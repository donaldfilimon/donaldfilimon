use wdbx::Linear;
use mlx_rs::array;

#[test]
fn test_linear_forward() {
    // Input: [1, 2], Weight: [[1, 2], [3, 4]], Bias: [0.5, 0.5]
    // Output: [1*1 + 2*3 + 0.5, 1*2 + 2*4 + 0.5] = [7.5, 10.5]
    let input = array!([1.0, 2.0]);
    let weight = array!([[1.0, 3.0], [2.0, 4.0]]); // MLX matmul is x @ w
    let bias = array!([0.5, 0.5]);
    
    let linear = Linear::new(weight, bias);
    let output = linear.forward(&input);
    
    let expected = vec![5.5, 11.5];
    let result: Vec<f32> = output.as_slice::<f32>().to_vec();
    assert_eq!(result, expected);
}
