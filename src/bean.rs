use crate::BmbpAuthResp;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpApp {
    app_id: Option<String>,
    app_code: Option<String>,
    app_name: Option<String>,
    app_short_name: Option<String>,
    app_group_name: Option<String>,
    app_icon: Option<String>,
    app_status: Option<String>,
    app_type: Option<BmbpAppType>,
    app_menus: Option<Vec<BmbpMenu>>,
    /// SSO 配置
    app_url: Option<String>,
    app_key: Option<String>,
    app_secret: Option<String>,
    app_callback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BmbpAppType {
    /// 原生应用
    APP,
    /// 集成应用
    SSO,
    /// 配置应用
    META,
    /// 链接
    LINK,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpMenu {
    app_id: Option<String>,
    menu_id: Option<String>,
    menu_code: Option<String>,
    menu_name: Option<String>,
    menu_name_path: Option<String>,
    menu_parent_code: Option<String>,
    menu_url: Option<String>,
    menu_type: Option<BmbpMenuType>,
    children: Option<Vec<BmbpMenu>>,
    open_position: Option<MenuOpenPosition>,
    open_type: Option<MenuOpenType>,
    menu_status: Option<String>,
    extend_json: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BmbpMenuType {
    MODULE,
    FUNC,
    LINK,
    PAGE,
    ACTION,
    FIELDSET,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MenuOpenPosition {
    WINDOW,
    IFRAME,
    ROUTE,
    LINK,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MenuOpenType {
    ROUTE,
    META,
    INNER,
    OUTER,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpOrgan {
    organ_id: Option<String>,
    organ_code: Option<String>,
    organ_code_path: Option<String>,
    organ_name: Option<String>,
    organ_name_path: Option<String>,
    organ_parent_code: Option<String>,
    organ_typ_: Option<BmbpOrganType>,
    grade: Option<u32>,
    children: Vec<BmbpOrgan>,
    region_code: Option<String>,
    region_name: Option<String>,
    extend_config: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BmbpOrganType {
    REGION,
    GROUP,
    UNIT,
    DEPT,
    POST,
    PERSON,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpUser {
    user_id: Option<String>,
    user_code: Option<String>,
    user_name: Option<String>,
    user_nick_name: Option<String>,
    user_mobile: Option<String>,
    user_email: Option<String>,
    owner_organ: Option<BmbpOrgan>,
    owner_apps: Option<Vec<BmbpApp>>,
    owner_menus: Option<Vec<BmbpMenu>>,
    owner_menu_roles: Option<Vec<BmbpRole>>,
    owner_data_roles: Option<Vec<BmbpRole>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpRole {
    code: Option<String>,
    name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BmbpToken {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub expire_at: Option<u32>,
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
pub trait BmbpAuthToken: Sync + Send {
    async fn create_token(
        &self,
        username: String,
        password: String,
    ) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn check_token(&self, token: String) -> BmbpAuthResp<Option<bool>>;
    async fn refresh_token(&self, token: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn invalid_token(&self, token: String) -> BmbpAuthResp<Option<bool>>;
    async fn remove_token(&self, token: String) -> BmbpAuthResp<Option<bool>>;
    async fn get_token_info(&self, token: String) -> BmbpAuthResp<Option<BmbpToken>>;
    async fn get_token_user(&self, token: String) -> BmbpAuthResp<Option<BmbpUser>>;
}
#[async_trait]
pub trait BmbpAuthRbac: Sync + Send {
    async fn get_user_by_id(&self, id: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_user_by_name(&self, name: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_user_by_mobile(&self, mobile: String) -> BmbpAuthResp<Option<BmbpUser>>;
    async fn get_users_by_organ_code(&self, code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_users_by_res_role_code(&self, code: String)
        -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_users_by_data_role_code(
        &self,
        code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpUser>>>;
    async fn get_organ_organ_by_id(&self, id: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organ_by_code(&self, code: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organ_by_path(&self, path: String) -> BmbpAuthResp<Option<BmbpOrgan>>;
    async fn get_organ_organs_by_parent_code(
        &self,
        code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organs_by_parent_path(
        &self,
        path: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organs_by_role_code(
        &self,
        code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree(&self) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_id(
        &self,
        id: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_code(
        &self,
        code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_path(
        &self,
        path: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_id(
        &self,
        id: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_code(
        &self,
        code: String,
    ) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>>;
    async fn get_organ_organ_tree_by_start_parent_path(
        &self,
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
    async fn get_menus_by_role_codes(&self, code: &[String])
        -> BmbpAuthResp<Option<Vec<BmbpMenu>>>;
}
