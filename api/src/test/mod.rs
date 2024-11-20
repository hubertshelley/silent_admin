use silent::prelude::{HandlerAppend, Route};

mod test_data_scope;

pub fn test_api() -> Route {
    Route::new("/data_scope").append(test_data_scope_api()) // 数据权限测试
}

fn test_data_scope_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(test_data_scope::get_sort_list)) // 获取筛选分页
        .append(Route::new("add").post(test_data_scope::add)) // 添加
        .append(Route::new("delete").delete(test_data_scope::delete)) // 硬删除
}
