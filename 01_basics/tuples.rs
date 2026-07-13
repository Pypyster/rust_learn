@ -0,0 +1,16 @@
fn main(){
    // tuple
    let mut user_alex: (i8, bool, f32, char) = (42, true, 1.86, 'R');
        println!("Info: {}",user_alex.0);

    user_alex.3 = 'G';
        println!("Info: {}",user_alex.3);

   // array
   let mut  nums:[i8; 6] = [1, 5,2 , 7, 8, 32];
        println!("Info: {}",nums[0]);

   nums[0] = 10;
        println!("Changed: {}", nums[0]);

}