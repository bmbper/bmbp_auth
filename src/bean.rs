use bmbp_marco_bean::*;
use crate::err::BmbpAuthResp;
use serde::Deserialize;
use serde::Serialize;
use async_trait::async_trait;

// 应用信息
#[bean_option]
pub struct BmbpApp {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    group: Option<String>,
    status: Option<String>,
    menus: Option<Vec<BmbpMenu>>,
}

// 应用目录
#[bean_option]
pub struct BmbpMenu {
    app_code: Option<String>,
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    name_path: Option<String>,
    parent_code: Option<String>,
    children: Option<Vec<BmbpMenu>>,
    // 打开功能地址
    open_url: Option<String>,
    // 打开功能位置：
    open_position: Option<String>,
    // 打开功能的方式
    open_type: Option<String>,
    status: Option<String>,
    extend_json: Option<String>,
}

// 组织信息
#[bean_option]
pub struct BmbpOrgan {
    id: Option<String>,
    code: Option<String>,
    code_path: Option<String>,
    name: Option<String>,
    name_path: Option<String>,
    parent_code: Option<String>,

    typ_: Option<String>,
    grade: Option<u32>,
    children: Vec<BmbpOrgan>,

    region_code: Option<String>,
    region_name: Option<String>,
}

// 用户信息
#[bean_option]
pub struct BmbpUser {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    nick_name: Option<String>,
    mobile: Option<String>,
    email: Option<String>,

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
#[bean_option]
pub struct BmbpRole {
    code: Option<String>,
    name: Option<String>,
}

#[bean_option]
pub struct BmbpToken {
    token: Option<String>,
    refresh_token: Option<String>,
    expire_at: Option<u32>,
}

#[async_trait]
pub trait BmbpAuthUser: Sync + Send {
    async fn get_current_info(&self) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_current_organ(&self) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_current_apps(&self) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_current_menus(&self) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_current_res_roles(&self) -> BmbpAuthResp<Option<Vec<BmbpRole>>>;
    async fn get_current_data_roles(&self) -> BmbpAuthResp<Option<Vec<BmbpRole>>>;
}
#[async_trait]
pub trait BmbpAuthToken: Sync + Send{
    async fn create_token(&self, username: String, password: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn check_token(&self, token: String) -> BmbpAuthResp<Option<bool>>;
    async fn refresh_token(&self, token: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn invalid_token(&self, token: String) -> BmbpAuthResp<Option<bool>>;
    async fn remove_token(&self, token: String) -> BmbpAuthResp<Option<bool>>;
    async fn get_token_info(&self, token: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn get_token_user(&self, token: String) -> BmbpAuthResp<Option<BmbpUser>>;
}
#[async_trait]
pub trait BmbpAuth :Sync + Send{
    async fn get_user_by_id(&self, id: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_user_by_name(&self, name: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_user_by_mobile(&self, mobile: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_users_by_organ_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_users_by_res_role_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_users_by_data_role_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_organ_organ_by_id(&self, id: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organ_by_code(&self, code: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organ_by_path(&self, path: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organs_by_parent_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organs_by_parent_path(&self, path: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organs_by_role_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree(&self) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_id(&self, id: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_code(&self,
                                                code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_path(&self,
                                                path: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_id(&self,
                                                     id: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_code(&self,
                                                       code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_path(&self,
                                                       path: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;

    async fn get_app_by_id(&self, id: String) -> BmbpAuthResp<Option<BmbpApp>>;
    async fn get_app_by_code(&self, code: String) -> BmbpAuthResp<Option<BmbpApp>>;
    async fn get_apps_by_user_id(&self, id: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_user_name(&self, name: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_user_mobile(&self, mobile: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_role_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_apps_by_role_codes(&self, code: &[String]) -> BmbpAuthResp<Option<Vec<BmbpApp>>>;
    async fn get_menu_by_id(&self, id: String) -> BmbpAuthResp<Option<BmbpMenu>>;
    async fn get_menu_by_code(&self, code: String) -> BmbpAuthResp<Option<BmbpMenu>>;
    async fn get_menu_by_path(&self, path: String) -> BmbpAuthResp<Option<BmbpMenu>>;
    async fn get_menus_by_app_id(&self, id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_app_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_app_ids(&self, id: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_app_codes(&self, code: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_user_id(&self, id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_user_name(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_user_mobile(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_role_id(&self, id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_role_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_role_ids(&self, id: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
    async fn get_menus_by_role_codes(&self, code: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
}
