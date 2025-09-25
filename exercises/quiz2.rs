// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!



// 定义命令枚举，包含三种操作：转为大写、修剪、追加"bar"指定次数
#[derive(Debug)]
enum Command {
    Uppercase,
    Trim,
    Append(usize), // 关联一个 usize 类型的参数，表示追加次数
}

// 处理命令的函数
// 输入：包含字符串和对应命令的元组向量
// 输出：处理后的字符串向量
fn process_commands(operations: Vec<(String, Command)>) -> Vec<String> {
    let mut results = Vec::with_capacity(operations.len()); // 预分配容量提升性能
    
    for (input_str, cmd) in operations {
        let processed_str = match cmd {
            // 处理大写命令：调用字符串的 to_uppercase 方法
            Command::Uppercase => input_str.to_uppercase(),
            
            // 处理修剪命令：调用 trim 方法去除首尾空白，再转为 String
            Command::Trim => input_str.trim().to_string(),
            
            // 处理追加命令：重复 "bar" n 次并拼接到原字符串后
            Command::Append(n) => format!("{}{}", input_str, "bar".repeat(n)),
        };
        
        results.push(processed_str);
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_processing() {
        // 准备测试用例：(输入字符串, 命令) 的向量
        let operations = vec![
            ("hello".to_string(), Command::Uppercase),
            ("  rust  ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(2)),
            ("".to_string(), Command::Trim),
            ("Test".to_string(), Command::Uppercase),
            ("  example  ".to_string(), Command::Append(1)),
        ];

        // 执行处理函数
        let results = process_commands(operations);

        // 验证结果
        assert_eq!(results, vec![
            "HELLO".to_string(),
            "rust".to_string(),
            "foobarbar".to_string(),
            "".to_string(),
            "TEST".to_string(),
            "  example  bar".to_string(),
        ]);
    }
}
    