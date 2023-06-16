use rand::Rng;
use std::iter::zip;

mod number;

enum FillType {
    EMPTY,
    ZEROS,
    ONES,
    RANDOM
}

struct NDArray { 
    content: Vec<number::Number>,
    schema: Vec<usize>, // определяет как будет разделяться матрица
}

fn calc_cell(row: Vec<number::Number>, col: Vec<number::Number>) -> number::Number {
    let mut result = number::Number::Int(1);
    for (a, b) in zip(row, col) {
        print!("{} * {} + ", a, b);
        result = result + a*b;
    }
    println!("0");
    result
}


impl NDArray {
    fn new(fill_type: FillType, schema: Vec<usize>) -> Self {
        let content_size = NDArray::calc_size_by_schema(schema.clone());
        let mut rng = rand::thread_rng();

        match fill_type {
            FillType::ZEROS => NDArray { content: vec![number::Number::Int(0); content_size], schema },
            FillType::ONES => NDArray { content: vec![number::Number::Int(1); content_size], schema },
            FillType::EMPTY => NDArray { content: Vec::with_capacity(content_size), schema: schema.clone() },
            FillType::RANDOM => {
                let mut arr = NDArray { content: Vec::with_capacity(content_size), schema};
                for _ in 0..content_size {
                    let num = rng.gen_range(0f32..=100f32) as i32;
                    arr.content.push(number::Number::Int(num));
                }
                arr
            }
        }
    }

    fn calc_size_by_schema(schema: Vec<usize>) -> usize {
        let mut size: usize = 1;
        for item in &schema { size *= item; }
        size
    }

    fn resize(&mut self, schema: Vec<usize>) -> (){ 
        if schema.len() != self.schema.len() {
            panic!("Количество измерений не совпадает");
        }
        if NDArray::calc_size_by_schema(schema.clone()) != self.content.len() {
            panic!("Размерности не совпадают");
        }
        self.schema = schema;
    }

    fn transpose(&self) -> NDArray {
        if self.schema.len() > 2 { 
            panic!("Боже мой, да как транспонировать массив размерности выше 2? Это же просто преступление!");
        }

        let content: Vec<&[number::Number]> = self.content.chunks(self.schema[1]).collect();
        let mut arr = NDArray::new(FillType::EMPTY, vec![self.schema[1], self.schema[0]]);

        for i in 0..self.schema[1] {
            for j in 0..self.schema[0] {
                arr.content.push(content[j][i]); 
            }
        }

        arr
    }

    fn dot(&self, b: NDArray) -> NDArray {
        let mut arr = NDArray::new(FillType::EMPTY, vec![self.schema[0], b.schema[1]]);
        let transposed_arr = b.transpose();

        for i in 0..arr.schema[0] {
            for j in 0..arr.schema[1] {
                let a: Vec<&[number::Number]> = self.content.chunks(self.schema[1]).collect();
                let b: Vec<&[number::Number]> = transposed_arr.content.chunks(b.schema[0]).collect();

                arr.content.push(calc_cell(a[i].to_vec(), b[j].to_vec()));
            }
        }

        arr
    }
}

impl std::fmt::Display for NDArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { 
        let sliced: Vec<&[number::Number]> = self.content.chunks(self.schema[1]).collect();
        write!(f, "{:?}", sliced)
    }
}

fn calc_ndarray(operation: number::Operation, left: NDArray, right: NDArray) -> NDArray{ 
    let mut arr = NDArray::new(FillType::ZEROS, left.schema);

    for (item, (a, b)) in zip(arr.content.iter_mut(), zip(left.content, right.content)) {
        match operation {
            number::Operation::ADD => *item = a + b,
            number::Operation::SUB => *item = a - b,
            number::Operation::MUL => *item = a * b,
            number::Operation::DIV => *item = a / b
        }
    }

    arr
}

impl std::ops::Add for NDArray { 
    type Output = NDArray;
    fn add(self, other: NDArray) -> NDArray {
        calc_ndarray(number::Operation::ADD, self, other)
    }
}
impl std::ops::Sub for NDArray { 
    type Output = NDArray;
    fn sub(self, other: NDArray) -> NDArray {
        calc_ndarray(number::Operation::SUB, self, other)
    }
}
impl std::ops::Mul for NDArray { 
    type Output = NDArray;
    fn mul(self, other: NDArray) -> NDArray {
        calc_ndarray(number::Operation::MUL, self, other)
    }
}
impl std::ops::Div for NDArray { 
    type Output = NDArray;
    fn div(self, other: NDArray) -> NDArray {
        calc_ndarray(number::Operation::DIV, self, other)
    }
}

fn main() {
    println!("--- СОЗДАНИЕ МАССИВА ---");
    let mut zeros_ndarray = NDArray::new(FillType::ZEROS, vec![1, 2]);
    println!("Массив заполненный нулями: {}", zeros_ndarray);
    zeros_ndarray.resize(vec![2, 1]);

    println!("Массив с измененным размером: {}", zeros_ndarray);

    println!("--- СУММА МАССИВОВ --- ");
    let a = NDArray::new(FillType::ONES, vec![2, 2]);
    let b = NDArray::new(FillType::RANDOM, vec![2, 2]);

    println!("{} + {}", a, b);
    println!("{}", a + b);

    println!("--- ТРАНСПОНИРОВАНИЕ ---");
    let arr_before_transpose = NDArray::new(FillType::RANDOM, vec![2, 3]);
    println!("До транспонирования: {}", arr_before_transpose);
    let arr_after_transpose = arr_before_transpose.transpose();
    println!("После транспонирования: {}", arr_after_transpose);

    println!("--- МАТРИЧНОЕ УМНОЖЕНИЕ ---");
    let a = NDArray::new(FillType::RANDOM, vec![1, 2]);
    let b = NDArray::new(FillType::RANDOM, vec![2, 1]);

    println!("{}", a);
    println!("*");
    println!("{}", b);
    println!("=");
    println!("{}", a.dot(b));
}
