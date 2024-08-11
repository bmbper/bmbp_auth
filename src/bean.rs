// 应用信息
pub struct BmbpApp {
    menus: Option<Vec<BmbpMenu>>,
}

// 应用目录
pub struct BmbpMenu {}

// 组织信息
pub struct BmbpOrgan {}

// 用户信息
pub struct BmbpUser {
    // 所属组织
    organ: Option<BmbpOrgan>,
    // 分配的应用
    apps: Option<Vec<BmbpApp>>,
    // 包含的菜单
    menus: Option<Vec<BmbpMenu>>,
    // 资源角色
    res_roles: Option<Vec<BmbpRole>>,
    // 数据权限
    data_roles: Option<Vec<BmbpRole>>,
}

// 角色信息
pub struct BmbpRole {
    code: Option<String>,
    name: Option<String>,
}

pub struct BmbpToken {
    token: Option<String>,
}

pub trait BmbpUserAuth {
    async fn get_current_info() -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_current_organ() -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_current_apps() -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_current_menus() -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_current_res_roles() -> BmbpAuthResp<Option<Vec<BmbpRole>>>;
    async fn get_current_data_roles() -> BmbpAuthResp<Option<Vec<BmbpRole>>>;
}

pub trait BmbpTokenAuth {
    async fn create_token(username: String, password: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn check_token(token: String) -> BmbpAuthResp<Option<bool>>;
    async fn refresh_token(token: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn invalid_token(token: String) -> BmbpAuthResp<Option<bool>>;
    async fn remove_token(token: String) -> BmbpAuthResp<Option<bool>>;
    async fn get_token_info(token: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn get_token_user(token: String) -> BmbpAuthResp<Option<BmbpUser>>;
}

pub trait BmbpAuth {
    async fn get_user_by_id(id: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_user_by_name(name: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_user_by_mobile(mobile: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_users_by_organ_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_users_by_res_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_users_by_data_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_organ_organ_by_id(id: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organ_by_code(code: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organ_by_path(path: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organs_by_parent_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organs_by_parent_path(path: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organs_by_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree() -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_code(
        code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_path(
        path: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_id(
        id: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_code(
        code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_path(
        path: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;

    async fn get_app_by_id(id: String) -> BmbpAuthResp<Option<BmbpApp>>;
    async fn get_app_by_code(code: String) -> BmbpAuthResp<Option<BmbpApp>>;
    async fn get_apps_by_user_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_user_name(name: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_user_mobile(mobile: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_role_codes(code: &[String]) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_menu_by_id(id: String) -> BmbpAuthResp<Option<BmbpMenu>>;
    async fn get_menu_by_code(code: String) -> BmbpAuthResp<Option<BmbpMenu>>;
    async fn get_menu_by_path(path: String) -> BmbpAuthResp<Option<BmbpMenu>>;
    async fn get_menus_by_app_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_app_code(code: String) -> BmbpAuthResp<Option<Vec<bmbpMenu>>>;
    async fn get_menus_by_app_ids(id: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_app_codes(code: &[String]) -> BmbpAuthResp<Option<Vec<bmbpMenu>>>;
    async fn get_menus_by_user_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_user_name(code: String) -> BmbpAuthResp<Option<Vec<bmbpMenu>>>;
    async fn get_menus_by_user_mobile(code: String) -> BmbpAuthResp<Option<Vec<bmbpMenu>>>;
    async fn get_menus_by_role_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_role_code(code: String) -> BmbpAuthResp<Option<Vec<bmbpMenu>>>;
    async fn get_menus_by_role_ids(id: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_role_codes(code: &[String]) -> BmbpAuthResp<Option<Vec<bmbpMenu>>>;
}
