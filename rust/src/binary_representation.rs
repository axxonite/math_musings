

pub fn test() {
    unit_test(binary_to_decimal("11010010101010001010101010001"), 0b11010010101010001010101010001, "binary_to_decimal");
    unit_test(&decimal_to_binary(0b11010010101010001010101010001)[..], "11010010101010001010101010001", "decimal_to_binary");
    unit_test(negate(876562425), -876562425, "negate");
    unit_test(binary_addition(1198975389, 1896546781), 1198975389 + 1896546781, "binary_addition");
    unit_test(binary_subtraction(198975389, 896546781), 198975389 - 896546781, "binary_subtraction");
}

fn unit_test<T : std::cmp::PartialEq>(a: T, b: T, test_name: &str) {
    if a != b {
        println!("Failed {}()", test_name);
        panic!("")
    }
}

// the following functions demonstrate how decimal can be converted to binary and vice versa, but the decimal representation has no relevance to
// how values are represented in memory.
fn binary_to_decimal(binary : &str) -> u32 {
    let mut result = 0;
    for digit in binary.chars() {
        result = result * 2;
        if digit == '1' {
            result = result + 1;
        }
    }
    result
}

fn decimal_to_binary(mut decimal: u32) -> String {
    let mut result = String::new();
    while decimal > 0 {
        result.push(if decimal % 2 == 1 {'1'} else {'0'} );
        decimal /= 2;
    }
    result.chars().rev().collect::<String>()
}

// returns u32, so can overflow, but this merely mimics what will happen on the hardware.
fn binary_addition(a: u32, b: u32) -> u32 {
    let mut carry = 0;
    let mut result = 0;
    for i in 0..=31 {
        let a_bit = a & (1 << i);
        let b_bit = b & (1 << i);
        let result_bit = a_bit ^ b_bit ^ carry;
        result |= result_bit;
        // align carry bit with next bit
        carry = ((a_bit & b_bit) | (a_bit & carry) | (b_bit & carry)) << 1;
    }
    result
}

fn negate(a : i32) -> i32 {
    !a + 1
}

fn binary_subtraction(a : i32, b: i32) -> i32 {
    negate(negate(a) + b)
}

// Definition of two's complement is that the complement is the value that you add to x to obtain 2^N where N is the number of bits/digits.
// Two's complement representation is ordered in the expected order for unsigned values.
// For unsigned values, the lowest value with the signed bit set to 1 represents the lowest representable negative value (-2^N) and goes up from there.
// Hence there's a discontinuity occuring between the largest positive value and the smallest negative value (aka. 01111111 and 10000000).
// Keep in mind the negative side can represent one more value than the positive side.
// Because of the flip operation, the two's complement is esentially a mirror image of the ordering from the positive side, centered around the middle of the bit range.
// 
// Why do we need the +1? We want to keep the positive numbers from 0 to 127.
// say we flip 1 (00000001) to -1 (11111110). Well now we've assigned the negative mirror of 1 to the next-to-last bit pattern.
// Not only is the last bit pattern (11111111) left unused, the values will not wraparound properly. Hence the +1, and the reason why the negative range has one more value than
// the positive range.
//
// [0..127] - [-128,-1]

// -(-x + y) = x - y

// Definitions:
// Radix - the number of unique digits used to represent numbers.
// Radix complement : b^n - y, so 10,000 - x for a 5-digit number.
// Diminised radix complement: b^n - 1 - y, so 9,999 - x for a 5-digit number. Radix complement can be obtained from the diminished radix complement by adding 1.
// 
// To subtract, perform comp(comp(x) + y): comp(x) = b^n - 1 - x + y = b^n - 1 - (x - y)
// Then take the diminished radix complement of this: b^n - 1 - (b^n - 1 - (x-y)) = b^n - 1 -b^n + 1 + x - y = x - y
// So: x - y => comp(comp(x) + y)

// Assign c = b^n - 1 (a constant value, aka 99999)
// c - (c - x + y) = c - c + x - y
// The only interesting bit about the diminished complement is you can subtract without carry, thus simplifying the logic.
