use async_trait::async_trait;
use bmbp_auth::{BmbpApp, BmbpAuthResp, BmbpAuthUser, BmbpAuthUserUtil,BmbpMenu, BmbpOrgan, BmbpRole, BmbpUser, register_bmbp_auth_user};
#[test]
fn test_user() {
    let user = BmbpUser::default();
    assert_eq!("", "");
}

#[tokio::test]
async fn test_bmbp_auth_user() {
    pub struct BmbpAuthUserImpl;
    #[async_trait]
    impl BmbpAuthUser for BmbpAuthUserImpl {
        async fn get_current_info(&self) -> BmbpAuthResp<Option<BmbpUser>> {
            let mut  user = BmbpUser::new();
            user.set_name(Some("a".to_string()));
            Ok(Some(user))
        }

        async fn get_current_organ(&self) -> BmbpAuthResp<Option<BmbpOrgan>> {
            Ok(Some(BmbpOrgan::new()))
        }

        async fn get_current_apps(&self) -> BmbpAuthResp<Option<Vec<BmbpApp>>> {
            Ok(Some(vec![]))
        }

        async fn get_current_menus(&self) -> BmbpAuthResp<Option<Vec<BmbpMenu>>> {
            Ok(Some(vec![]))
        }

        async fn get_current_res_roles(&self) -> BmbpAuthResp<Option<Vec<BmbpRole>>> {
            Ok(Some(vec![]))
        }

        async fn get_current_data_roles(&self) -> BmbpAuthResp<Option<Vec<BmbpRole>>> {
            Ok(Some(vec![]))
        }
    }

    let test_impl: Box<dyn BmbpAuthUser> = Box::new(BmbpAuthUserImpl {});
    register_bmbp_auth_user(test_impl);

    if let Some(user) = BmbpAuthUserUtil::get_current_info().await.unwrap() {
        assert_eq!(user.get_name().is_some(), true);
    } else {
        assert!(false, "获取用户信息失败")
    }
}