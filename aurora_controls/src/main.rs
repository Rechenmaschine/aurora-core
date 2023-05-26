

trait Block {
    // should be constrained to vec or tuples 
    // uniform or not?

	type Input;
	type Output: Copy;

	fn step(self, input: Self::Input) -> Self::Output;

}


pub struct Adder<const N: usize> {
    // Phantom Type
}


impl <const N: usize> Block for Adder<N> {
    type Input =  [f32; N];
    type Output = f32;
    
    fn step(self, x: Self::Input) -> Self::Output {
        // sum<T> doesn't have a direct implementation for Add 
        return x.iter().sum();
    }
}


fn main() {
    let x = [4.0, 7.0];

    println!("{:?}",x);
    let a = Adder{};
    println!("{:?}",a.step(x));
}
