use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bmbp_auth::{BMBP_AUTH_USER_CACHE, BmbpApp, BmbpAuthResp, BmbpAuthUser, BmbpAuthUserUtil, BmbpAuthUtil, BmbpMenu, BmbpOrgan, BmbpRole, BmbpUser, DEMO};

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
            Ok(Some(BmbpUser::new()))
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

    let a1: Box<dyn BmbpAuthUser> = Box::new(BmbpAuthUserImpl {});
    (&*BMBP_AUTH_USER_CACHE).write().unwrap().register_auth(a1);
    let a2 = (&BMBP_AUTH_USER_CACHE).read().unwrap().cache.as_ref();

    let user = BmbpAuthUserUtil::get_current_info().await.unwrap();
    assert_eq!(user.is_some(), true);
}