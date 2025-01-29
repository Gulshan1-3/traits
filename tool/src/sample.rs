// sample.rs

use std::fmt::{Debug, Display};
use std::ops::Add;

// A trait with associated types and lifetime parameters
trait DataProcessor<'a, T> {
    type Output;
    fn process(&'a self, data: T) -> Self::Output;
    fn validate(&self, input: &'a T) -> bool;
}

// A trait with multiple generic parameters and bounds
trait Container<T, U> 
where 
    T: Clone + Debug,
    U: Display,
{
    fn store(&self, item1: T, item2: U);
    fn retrieve(&self) -> (T, U);
}

// A struct with multiple lifetime parameters and generic types
struct DataHolder<'a, 'b, T, U> 
where
    T: Debug + Clone,
    U: Display + Add<Output = U>,
{
    primary_data: &'a T,
    secondary_data: &'b U,
    processed: Vec<U>,
}

// Implementation block with lifetime and type constraints
impl<'a, 'b, T, U> DataHolder<'a, 'b, T, U> 
where
    T: Debug + Clone,
    U: Display + Add<Output = U>,
{
    fn new(primary: &'a T, secondary: &'b U) -> Self {
        Self {
            primary_data: primary,
            secondary_data: secondary,
            processed: Vec::new(),
        }
    }

    fn combine<V>(&self, extra: V) -> String 
    where
        V: AsRef<str>,
    {
        format!("{:?} - {} - {}", 
            self.primary_data, 
            self.secondary_data, 
            extra.as_ref()
        )
    }
}

// A generic function with multiple bounds and lifetimes
fn process_data<'a, 'b, T, U, P>(
    processor: &'a P,
    data: &'b T,
    default: U
) -> P::Output 
where
    P: DataProcessor<'a, T>,
    T: Debug + Clone,
    U: Default + Display,
    'b: 'a,
{
    if processor.validate(data) {
        processor.process(data.clone())
    } else {
        panic!("Invalid data: {:?}", data)
    }
}

// An enum with generic parameters and where clause
enum DataResult<T, E>
where 
    T: Debug,
    E: Display,
{
    Success(T),
    Error(E),
    Pending,
}

// A struct implementing multiple traits
struct NumberProcessor<T> 
where 
    T: Add<Output = T> + Clone + Debug,
{
    base_value: T,
}

impl<'a, T> DataProcessor<'a, T> for NumberProcessor<T>
where 
    T: Add<Output = T> + Clone + Debug,
{
    type Output = T;

    fn process(&'a self, data: T) -> Self::Output {
        self.base_value.clone() + data
    }

    fn validate(&self, input: &'a T) -> bool {
        true  // Simplified validation
    }
}