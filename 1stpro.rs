// 1. Bindind and Mutability
fn main() {
	let x = i32 =2;
	let _y = i32;//(_) To fix Y variable
	assert_eq!(x,2 ); //Two values are Equal
	println!("Success");
}

//2. Use mut to mark a variable as mutable 
fn main() {
	let mut x: i32=1;
	x = x+ 2;//mutating x
	assert_eq!(x,3);
	println!("Success"); 
}

//3. Scope
// A scope is the range within the program for which the item is valid 
fn main() { //Outer scope
	let x:i32=10;
	let y:i32=5;
	{ //Inner scope
		// let y:i32=5; initialize the value to outer scope
		println!("The value of x is {} and value of y is {}",x,y);
	}// Inner scope
	println!("The value of x is {} and value of y is {}",x,y);
}//Outer scope

fn main() {
	define_x(); //Execute aagara dhu kaga kudukanum
	// println!("{}, World",x );
}
fn define_x(){
	let x:&str="Hello";
	println!("{}, World",x );
}

// Shadowing 
// You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.
fn main() {
	let x: i32=5;
	{
		let x=12;
		// assert_eq!(x,5); //changes to 12
		assert_eq!(x,12);
	}
	// assert_eq!(x,12 ); changes to 5
	assert_eq!(x,5);

	let x:i32=42;
	println!("{}",x); //prints "42"
}

fn main() {
	let mut x: i32=1;
	x=7; 
	//Shadowing and re-binding 
	// let x =x; //Shadow pandra dha dhu na la endha x um mutable aagudhu
	let mut x=x;
	// x +=3; increment the X
	x=x+3;
	let y:i32=4;
	//shadowing 
	let y: &str ="I can also be bound to text!"; //Sting Type
	println!("Success");
}
