// 一个调试类，其主程序可以通过以下方式调用：
//      cargo run --bin dump_obj FILE...
// 其中每个 FILE 是由 Utils::write_object 生成的文件（或任何
// 包含序列化对象的文件）。这只会读取 FILE，
// 反序列化它，并在生成的对象上调用 dump 方法。
// 该对象必须实现 gitlet::Dumpable 特质才能
// 正常工作。例如，您可能会像这样定义您的类：
//
//        use gitlet::Dumpable;
//        use std::collections::BTreeMap;
//        struct MyClass {
//            size: usize,
//            mapping: BTreeMap<String, String>,
//        }
//        impl Dumpable for MyClass {
//            fn dump(&self) {
//                println!("size: {}", self.size);
//                println!("mapping: {:?}", self.mapping);
//            }
//        }
//
// 如图所示，您的 dump 方法应该打印出您的类对象中的有用信息。
// @author P. N. Hilfinger

use std::env;

// 从文件中反序列化并对其内容应用 dump 方法
fn main() {
    let args: Vec<String> = env::args().collect();
    
    for file_name in &args[1..] {
        // TODO: 实现反序列化和转储逻辑
        println!("处理文件: {}", file_name);
    }
}
