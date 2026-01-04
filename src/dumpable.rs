// 描述可转储对象的接口
// @author P. N. Hilfinger

// 可转储对象的特质
pub trait Dumpable {
    // 在标准输出中打印有关此对象的有用信息
    fn dump(&self);
}
