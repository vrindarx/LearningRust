 fn _variables() {
	let x = 5; 
    println!("The value of x is {}", x);
    let x = "thane" ;
    println!("The value of x is {}", x);
    
    const _SUBCOUNT: u32 = 100_000;  
}

fn _compound() {
	//Compound Data Types
	let tup = ("thane", 64); 
	let (_station, _count) = tup; 
	let _count = tup.1;
	
	let error_codes = [200, 404, 500];
	let _error_notfound = error_codes[1];
	
	let _byte = [0; 8]; //[0,0,0,0,0,0,0,0] eight zeros
	
}

fn functions() {
	
	println!("sum is {}", another_function(11, 22 )) ; 
	
}

fn another_function(x: i32, y: i32) -> i32  {
	println!("The value of x is {}",x);
	println!("The value of y is {}",y);
	
	x * y
}

fn control_flow() {
	let number = 5;
	
	if number > 10 {
		println!("Number is greater 10");
	} else if number < 5 {
		println!("Number is smaller than 5"); 
	} else {
		println!("The number is {}", number);
	}
	
	
	let condition = true; 
	let number = if condition { 5 } else { 15 };
	
	println!("The number is {}", number);
	
}
	
fn loops() {
	let mut counter = 0;
	let result = loop {
		counter += 1;
		
		if counter == 10 {
			break counter;
		}
	}; 
	
	println!("Count is {}", result); 
	
	let mut number = 3;
	while number != 0 {
		println!("{}", number);
		
		number -= 1;
	}
	
	println!("LIFTOFF!!!");
}


fn main() {
	loops();
} 
 
