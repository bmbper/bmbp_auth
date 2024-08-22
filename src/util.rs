use std::sync::Arc;
use crate::{BMBP_AUTH_CACHE, BMBP_AUTH_USER_CACHE, BmbpApp, BmbpAuth, BmbpAuthErr, BmbpAuthErrType, BmbpAuthResp, BmbpAuthToken, BmbpAuthUser, BmbpMenu, BmbpOrgan, BmbpRole, BmbpToken, BmbpUser};
use crate::cache::BMBP_AUTH_TOKEN_CACHE;

pub struct BmbpAuthUtil;

pub struct BmbpAuthTokenUtil;

pub struct BmbpAuthUserUtil;

impl BmbpAuthUtil {
    fn get_auth() -> BmbpAuthResp<Arc<Box<dyn BmbpAuth>>> {
        return if (*BMBP_AUTH_CACHE).read().unwrap().is_none() {
            Err(BmbpAuthErr::build(BmbpAuthErrType::NotFoundImpl, "9001".to_string(), "BmbpAuth未找到认证服务实现".to_string()))
        } else {
            Ok(BMBP_AUTH_CACHE.read().unwrap().as_ref().unwrap().clone())
        };
    }

    pub async fn get_user_by_id(id: String) -> BmbpAuthResp<Option<BmbpUser>> {
        BmbpAuthUtil::get_auth()?.get_user_by_id(id).await
    }

    pub async fn get_user_by_name(name: String) -> BmbpAuthResp<Option<BmbpUser>> {
        BmbpAuthUtil::get_auth()?.get_user_by_name(name).await
    }

    pub async fn get_user_by_mobile(mobile: String) -> BmbpAuthResp<Option<BmbpUser>> {
        BmbpAuthUtil::get_auth()?.get_user_by_mobile(mobile).await
    }

    pub async fn get_users_by_organ_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>> {
        BmbpAuthUtil::get_auth()?.get_users_by_organ_code(code).await
    }

    pub async fn get_users_by_res_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>> {
        BmbpAuthUtil::get_auth()?.get_users_by_res_role_code(code).await
    }

    pub async fn get_users_by_data_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpUser>>> {
        BmbpAuthUtil::get_auth()?.get_users_by_data_role_code(code).await
    }

    pub async fn get_organ_organ_by_id(id: String) -> BmbpAuthResp<Option<BmbpOrgan>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_by_id(id).await
    }

    pub async fn get_organ_organ_by_code(code: String) -> BmbpAuthResp<Option<BmbpOrgan>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_by_code(code).await
    }

    pub async fn get_organ_organ_by_path(path: String) -> BmbpAuthResp<Option<BmbpOrgan>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_by_path(path).await
    }

    pub async fn get_organ_organs_by_parent_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organs_by_parent_code(code).await
    }

    pub async fn get_organ_organs_by_parent_path(path: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organs_by_parent_path(path).await
    }

    pub async fn get_organ_organs_by_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organs_by_role_code(code).await
    }

    pub async fn get_organ_organ_tree() -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_tree().await
    }

    pub async fn get_organ_organ_tree_by_start_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_tree_by_start_id(id).await
    }

    pub async fn get_organ_organ_tree_by_start_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_tree_by_start_code(code).await
    }

    pub async fn get_organ_organ_tree_by_start_path(path: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_tree_by_start_path(path).await
    }

    pub async fn get_organ_organ_tree_by_start_parent_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_tree_by_start_parent_id(id).await
    }

    pub async fn get_organ_organ_tree_by_start_parent_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_tree_by_start_parent_code(code).await
    }

    pub async fn get_organ_organ_tree_by_start_parent_path(path: String) -> BmbpAuthResp<Option<Vec<BmbpOrgan>>> {
        BmbpAuthUtil::get_auth()?.get_organ_organ_tree_by_start_parent_path(path).await
    }

    pub async fn get_app_by_id(id: String) -> BmbpAuthResp<Option<BmbpApp>> {
        BmbpAuthUtil::get_auth()?.get_app_by_id(id).await
    }

    pub async fn get_app_by_code(code: String) -> BmbpAuthResp<Option<BmbpApp>> {
        BmbpAuthUtil::get_auth()?.get_app_by_code(code).await
    }

    pub async fn get_apps_by_user_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>> {
        BmbpAuthUtil::get_auth()?.get_apps_by_user_id(id).await
    }

    pub async fn get_apps_by_user_name(name: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>> {
        BmbpAuthUtil::get_auth()?.get_apps_by_user_name(name).await
    }

    pub async fn get_apps_by_user_mobile(mobile: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>> {
        BmbpAuthUtil::get_auth()?.get_apps_by_user_mobile(mobile).await
    }

    pub async fn get_apps_by_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpApp>>> {
        BmbpAuthUtil::get_auth()?.get_apps_by_role_code(code).await
    }

    pub async fn get_apps_by_role_codes(code: &[String]) -> BmbpAuthResp<Option<Vec<BmbpApp>>> {
        BmbpAuthUtil::get_auth()?.get_apps_by_role_codes(code).await
    }

    pub async fn get_menu_by_id(id: String) -> BmbpAuthResp<Option<BmbpMenu>> {
        BmbpAuthUtil::get_auth()?.get_menu_by_id(id).await
    }

    pub async fn get_menu_by_code(code: String) -> BmbpAuthResp<Option<BmbpMenu>> {
        BmbpAuthUtil::get_auth()?.get_menu_by_code(code).await
    }

    pub async fn get_menu_by_path(path: String) -> BmbpAuthResp<Option<BmbpMenu>> {
        BmbpAuthUtil::get_auth()?.get_menu_by_path(path).await
    }

    pub async fn get_menus_by_app_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_app_id(id).await
    }

    pub async fn get_menus_by_app_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_app_code(code).await
    }

    pub async fn get_menus_by_app_ids(id: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_app_ids(id).await
    }

    pub async fn get_menus_by_app_codes(code: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_app_codes(code).await
    }

    pub async fn get_menus_by_user_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_user_id(id).await
    }

    pub async fn get_menus_by_user_name(code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_user_name(code).await
    }

    pub async fn get_menus_by_user_mobile(code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_user_mobile(code).await
    }

    pub async fn get_menus_by_role_id(id: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_role_id(id).await
    }

    pub async fn get_menus_by_role_code(code: String) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_role_code(code).await
    }

    pub async fn get_menus_by_role_ids(id: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_role_ids(id).await
    }

    pub async fn get_menus_by_role_codes(code: &[String]) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUtil::get_auth()?.get_menus_by_role_codes(code).await
    }
}

impl BmbpAuthTokenUtil {
    fn get_auth() -> BmbpAuthResp<Arc<Box<dyn BmbpAuthToken>>> {
        return if (*BMBP_AUTH_TOKEN_CACHE).read().unwrap().is_none() {
            Err(BmbpAuthErr::build(BmbpAuthErrType::NotFoundImpl, "9002".to_string(), "BmbpAuthToken未找到认证服务实现".to_string()))
        } else {
            Ok(BMBP_AUTH_TOKEN_CACHE.read().unwrap().as_ref().unwrap().clone())
        };
    }
    pub async fn create_token(username: String, password: String) -> BmbpAuthResp<Option<BmbpToken>> {
        BmbpAuthTokenUtil::get_auth()?.create_token(username, password).await
    }
    pub async fn check_token(token: String) -> BmbpAuthResp<Option<bool>> {
        BmbpAuthTokenUtil::get_auth()?.check_token(token).await
    }
    pub async fn refresh_token(token: String) -> BmbpAuthResp<Option<BmbpToken>> {
        BmbpAuthTokenUtil::get_auth()?.refresh_token(token).await
    }
    pub async fn invalid_token(token: String) -> BmbpAuthResp<Option<bool>> {
        BmbpAuthTokenUtil::get_auth()?.invalid_token(token).await
    }
    pub async fn remove_token(token: String) -> BmbpAuthResp<Option<bool>> {
        BmbpAuthTokenUtil::get_auth()?.remove_token(token).await
    }
    pub async fn get_token_info(token: String) -> BmbpAuthResp<Option<BmbpToken>> {
        BmbpAuthTokenUtil::get_auth()?.get_token_info(token).await
    }
    pub async fn get_token_user(token: String) -> BmbpAuthResp<Option<BmbpUser>> {
        BmbpAuthTokenUtil::get_auth()?.get_token_user(token).await
    }
}

impl BmbpAuthUserUtil {
    fn get_auth() -> BmbpAuthResp<Arc<Box<dyn BmbpAuthUser>>> {
        return if (*BMBP_AUTH_USER_CACHE).read().unwrap().is_none() {
            Err(BmbpAuthErr::build(BmbpAuthErrType::NotFoundImpl, "9003".to_string(), "BmbpAuthUser未找到认证服务实现".to_string()))
        } else {
            Ok((&*BMBP_AUTH_USER_CACHE).read().unwrap().as_ref().unwrap().clone())
        };
    }
    pub async fn get_current_info() -> BmbpAuthResp<Option<BmbpUser>> {
        BmbpAuthUserUtil::get_auth()?.get_current_info().await
    }
    pub async fn get_current_organ() -> BmbpAuthResp<Option<BmbpOrgan>> {
        BmbpAuthUserUtil::get_auth()?.get_current_organ().await
    }
    pub async fn get_current_apps() -> BmbpAuthResp<Option<Vec<BmbpApp>>> {
        BmbpAuthUserUtil::get_auth()?.get_current_apps().await
    }
    pub async fn get_current_menus() -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
        BmbpAuthUserUtil::get_auth()?.get_current_menus().await
    }

    pub async fn get_current_res_roles() -> BmbpAuthResp<Option<Vec<BmbpRole>>> {
        BmbpAuthUserUtil::get_auth()?.get_current_res_roles().await
    }

    pub async fn get_current_data_roles() -> BmbpAuthResp<Option<Vec<BmbpRole>>> {
        BmbpAuthUserUtil::get_auth()?.get_current_data_roles().await
    }
}