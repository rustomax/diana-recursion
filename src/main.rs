// Simple recursion example for Diana

// print_digits() function calls itself
fn print_digits(square_size: i32, digit_type: i32) {
    if square_size != 0 {
        print!("{} ", digit_type);
        print_digits(square_size - 1, digit_type);
    }
}

fn print_line(left_zeros: i32, middle_ones: i32, right_zeros: i32) {
    print_digits(left_zeros, 0);
    print_digits(middle_ones, 1);
    print_digits(right_zeros, 0);
    println!("");
}

// print_square() function is recursive too
fn print_square(square_size: i32, left_zeros: i32, middle_ones: i32, right_zeros: i32) {
    if square_size != 0 {
        print_line(left_zeros, middle_ones, right_zeros);
        print_square(square_size - 1, left_zeros, middle_ones, right_zeros);
    }
}

fn main() {
    let n = 3;
    let h = 2;
    let m = 4;

    print_square (n, 0, n, h + m);
    print_square (h, n, h, m);
    print_square (m, n + h, m, 0);
}

// Output:
// 1 1 1 0 0 0 0 0 0
// 1 1 1 0 0 0 0 0 0
// 1 1 1 0 0 0 0 0 0
// 0 0 0 1 1 0 0 0 0
// 0 0 0 1 1 0 0 0 0
// 0 0 0 0 0 1 1 1 1
// 0 0 0 0 0 1 1 1 1
// 0 0 0 0 0 1 1 1 1
// 0 0 0 0 0 1 1 1 1
