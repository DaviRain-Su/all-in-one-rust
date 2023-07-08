// 在这个例子中，use example_tests::add; 导入了要进行测试的函数。然后，像单元测试一样使用 #[test] 属性和 assert_eq! 宏来编写测试。
//
// 请注意，你需要将 example_tests 替换为你的 crate 名称。
use example_tests::add;

#[test]
fn test_add_two() {
    assert_eq!(add(2, 2), 4);
}
