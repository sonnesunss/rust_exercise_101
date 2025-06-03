/*
    使用借用类型作为参数

使用解引用强制转换的目标可以增加代码的灵活性，通过这种方式， 函数将
接受更多的输入类型，应该总是倾向于使用借用类型.

例如， 使用&str 而不是&string，&[T]而不是&vev<T>, 以及&T而不是&Box<T>

使用借用类型，可以避免已经提供一层间接性的所有类型上的多层间接。例如，String有一层间接，所以&String会有两层间接。我们可以通过使用&str来避免这种情况，并且让&String在函数被调用时强制变成&str。

*/

/*
   例如有一个例子，我们希望确定一个词是否包含三个连续的元音， 我们不需要拥有字符串来确定这一点，所以我们将使用一个引用
*/
#[allow(dead_code)]
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }

    false
}

fn main() {
    println!("Hello, world!");

    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();

    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // 而对于下面的传递&str参数的话，下面的将不会被工作
    // 如果我们将参数类型转变成&str的话，那么都将会正常工作
    println!("ferris: {}", three_vowels("ferris"));
    println!("curious: {}", three_vowels("curious"));

    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Feries".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }
}
