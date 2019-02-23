//Struct core::marker::PhantomData
//Zero-size type used to mark things that "act like" they own a T.
//Adding a PhantomData<T> field to your type tells the compiler that
//your type act as though stores a value of type T, even
//though it doesn't really. This information is used when computing
//certain safety property


use std::marker::PhantomData;

// A phantom type struct which is generic ver a with hidden parameter B
#[derive(PartialEq)] //Allow equality test for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);


#[derive(PartialEq)] //Allow equality test for this type
struct PhantomStruct<A, B>{first: A, phantom: PhantomData<B>}
//Note : Storage is allocated for generic type A, but not for B.
//Therefore, B cannot be used in computatios.


fn main() {
    //Here, f32 and f64 are the hidden parameters
    //PhantomTuple type specidied as <char, f32>
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);

    //PhantomTuple type specified as <char, f64>
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _tuple3: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct{first:'Q',
        phantom: PhantomData,};


    let _struct2: PhantomStruct<char, f64> = PhantomStruct{first:'Q',
        phantom: PhantomData,};

    // Compile-time Error! Type mismatch so these cannot be compared:
    //println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);

    // Compile-time Error! Type mismatch so these cannot be compared:
    //println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);
    println!("_tuple2 == _tuple3 yields: {}",
              _tuple2 == _tuple3);
    

}
