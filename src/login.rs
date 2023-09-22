use crate::util::common;
use crate::util::common::CurrentUser;
use serde_json;
use user_cli::apis::user_controller_api::{LoginError, ValidateExistEmailError};
use user_cli::apis::{user_controller_api, Error};
use user_cli::models;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Default, Clone)]
enum ValidStatus {
    Valid,
    InValid(String),
    #[default]
    None,
}

#[function_component(Login)]
pub fn login() -> Html {
    let force_update = use_force_update();
    let email_valid = use_mut_ref(|| ValidStatus::default());
    let pwd_valid = use_mut_ref(|| ValidStatus::default());
    let request_fail_msg = use_mut_ref(|| String::default());
    let email_ref = use_node_ref();
    let pwd_ref = use_node_ref();

    let mut email_invalid_msg = String::from("");
    let email_input_class = match email_valid.borrow().clone() {
        ValidStatus::Valid => "input is-success is-medium is-rounded",
        ValidStatus::InValid(s) => {
            email_invalid_msg = s.clone();
            "input is-danger is-medium is-rounded"
        }
        ValidStatus::None => "input is-info is-medium is-rounded",
    };
    let mut pwd_invalid_msg = String::from("");
    let password_input_class = match pwd_valid.borrow().clone() {
        ValidStatus::Valid => "input is-success is-medium is-rounded",
        ValidStatus::InValid(e) => {
            pwd_invalid_msg = e.clone();
            "input is-danger is-medium is-rounded"
        }
        ValidStatus::None => "input is-info is-medium is-rounded",
    };

    let on_email_change = {
        let email_ref = email_ref.clone();
        let email_valid = email_valid.clone();
        let force_update = force_update.clone();
        Callback::from(move |_| {
            let el = email_ref.cast::<HtmlInputElement>();
            let email = el.map(|x| x.value()).unwrap_or_default();
            if let Err(e) = common::validate_email(&email) {
                *email_valid.borrow_mut() = ValidStatus::InValid(format!("{}", e));
                force_update.force_update();
            } else {
                let email = email.clone();
                let email_valid = email_valid.clone();
                let force_update = force_update.clone();
                spawn_local(async move {
                    match user_controller_api::validate_exist_email(
                        &common::get_cli_config_without_token().unwrap(),
                        &email,
                    )
                    .await
                    {
                        Ok(_) => {
                            *email_valid.borrow_mut() = ValidStatus::Valid;
                        }
                        Err(err) => match err {
                            Error::ResponseError(res_err) => match res_err.entity {
                                Some(ValidateExistEmailError::Status400(e))
                                | Some(ValidateExistEmailError::Status500(e)) => {
                                    *email_valid.borrow_mut() =
                                        ValidStatus::InValid(format!("{}", e.msg));
                                }
                                _ => {
                                    *email_valid.borrow_mut() =
                                        ValidStatus::InValid(format!("{}", res_err.content));
                                }
                            },
                            _ => {
                                *email_valid.borrow_mut() =
                                    ValidStatus::InValid(format!("{}", err));
                            }
                        },
                    }
                    force_update.force_update();
                })
            }
        })
    };

    let on_pwd_change = {
        let pwd_ref = pwd_ref.clone();
        let pwd_valid = pwd_valid.clone();
        let force_update = force_update.clone();
        Callback::from(move |_| {
            let el = pwd_ref.cast::<HtmlInputElement>();
            let pwd = el.map(|x| x.value()).unwrap_or_default();
            if let Err(e) = common::validate_pwd(&pwd) {
                *pwd_valid.borrow_mut() = ValidStatus::InValid(format!("{}", e));
            } else {
                *pwd_valid.borrow_mut() = ValidStatus::Valid;
            }
            force_update.force_update();
        })
    };

    let login = {
        let email_ref = email_ref.clone();
        let pwd_ref = pwd_ref.clone();
        let email_valid = email_valid.clone();
        let pwd_valid = pwd_valid.clone();
        let force_update = force_update.clone();
        let request_fail_msg = request_fail_msg.clone();
        Callback::from(move |_| {
            let email = email_ref
                .cast::<HtmlInputElement>()
                .map(|x| x.value())
                .unwrap_or_default();
            if let Err(e) = common::validate_email(&email) {
                *email_valid.borrow_mut() = ValidStatus::InValid(format!("{}", e));
                force_update.force_update();
            } else {
                let pwd = pwd_ref
                    .cast::<HtmlInputElement>()
                    .map(|x| x.value())
                    .unwrap_or_default();
                if let Err(e) = common::validate_pwd(&pwd) {
                    *pwd_valid.borrow_mut() = ValidStatus::InValid(format!("{}", e));
                    force_update.force_update();
                } else {
                    let email = email.clone();
                    let pwd = pwd.clone();
                    let email_valid = email_valid.clone();
                    let force_update = force_update.clone();
                    let request_fail_msg = request_fail_msg.clone();
                    spawn_local(async move {
                        match user_controller_api::validate_exist_email(
                            &common::get_cli_config_without_token().unwrap(),
                            &email,
                        )
                        .await
                        {
                            Ok(_) => {
                                let req = models::LoginReq {
                                    email: email,
                                    pwd: pwd,
                                };
                                match user_controller_api::login(
                                    &common::get_cli_config_without_token().unwrap(),
                                    req,
                                )
                                .await
                                {
                                    Ok(res) => {
                                        common::set_local_storage("token", &res.data);
                                        match user_controller_api::get_current_user(
                                            &common::get_cli_config().unwrap(),
                                        )
                                        .await
                                        {
                                            Ok(res) => {
                                                let a = res.data.clone();
                                                let v = CurrentUser {
                                                    id: a.id,
                                                    r#type: a.r#type.to_string(),
                                                    email: a.email,
                                                    name: a.name,
                                                    mobile: a.mobile,
                                                    laston: a.laston,
                                                    created_at: a.created_at,
                                                    updated_at: a.updated_at,
                                                    expire_at: a.expire_at,
                                                };
                                                common::set_local_storage(
                                                    "current_user",
                                                    serde_json::to_string(&v).unwrap().as_str(),
                                                );
                                                common::redirect("/main/user");
                                            }
                                            Err(err) => {
                                                *request_fail_msg.borrow_mut() =
                                                    format!("get current user failed: {}", err);
                                            }
                                        }
                                    }
                                    Err(err) => match err {
                                        Error::ResponseError(res_err) => match res_err.entity {
                                            Some(LoginError::Status400(e))
                                            | Some(LoginError::Status500(e)) => {
                                                *request_fail_msg.borrow_mut() =
                                                    format!("{}", e.msg)
                                            }
                                            _ => {
                                                *request_fail_msg.borrow_mut() =
                                                    format!("{}", res_err.content)
                                            }
                                        },
                                        _ => *request_fail_msg.borrow_mut() = format!("{}", err),
                                    },
                                }
                            }
                            Err(err) => match err {
                                Error::ResponseError(res_err) => match res_err.entity {
                                    Some(ValidateExistEmailError::Status400(e))
                                    | Some(ValidateExistEmailError::Status500(e)) => {
                                        *email_valid.borrow_mut() =
                                            ValidStatus::InValid(format!("{}", e.msg));
                                    }
                                    _ => {
                                        *email_valid.borrow_mut() =
                                            ValidStatus::InValid(format!("{}", res_err.content));
                                    }
                                },
                                _ => {
                                    *email_valid.borrow_mut() =
                                        ValidStatus::InValid(format!("{}", err));
                                }
                            },
                        }
                        force_update.force_update();
                    })
                }
            }
        })
    };

    let on_keydown = {
        let login = login.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                login.emit(());
            }
        })
    };
    let on_login = {
        let login = login.clone();
        Callback::from(move |_| {
            login.emit(());
        })
    };

    html! {
        <>
        <header>
            <link rel="stylesheet" type="text/css" href="/login.css"/>
        </header>
        <section class="hero is-fullheight">
            <div class="hero-body has-text-centered">
            <div class="login">
                <img alt="fuck you" src="/static/img/logo.png" style="height: 100px"/>
                // <svg id="logo-6" width="325" viewBox="0 0 134 34" fill="none" xmlns="http://www.w3.org/2000/svg"> <path d="M15.45 6.64999V2.39999H2.05V31.6H15.45V27.35C12.705 27.35 10.0724 26.2595 8.13144 24.3185C6.19044 22.3775 5.1 19.745 5.1 17C5.1 14.255 6.19044 11.6224 8.13144 9.68144C10.0724 7.74044 12.705 6.64999 15.45 6.64999V6.64999Z" class="ccustom" fill="#394149"></path> <path d="M15.45 6.64999V27.35C18.195 27.35 20.8276 26.2595 22.7686 24.3185C24.7096 22.3775 25.8 19.745 25.8 17C25.8 14.255 24.7096 11.6224 22.7686 9.68144C20.8276 7.74044 18.195 6.64999 15.45 6.64999V6.64999Z" class="ccustom" fill="#394149"></path> <path d="M33.32 9.25H36.32V23.7H33.32V9.25Z" class="ccustom" fill="#394149"></path> <path d="M38 18.75C38 17.7104 38.3087 16.6942 38.8869 15.8302C39.4651 14.9662 40.2868 14.2933 41.2479 13.8968C42.209 13.5004 43.2661 13.3982 44.2853 13.6032C45.3045 13.8082 46.2399 14.3112 46.9729 15.0484C47.7059 15.7857 48.2036 16.7239 48.4028 17.7443C48.6019 18.7647 48.4937 19.8212 48.0917 20.78C47.6897 21.7387 47.0122 22.5566 46.1449 23.1298C45.2776 23.7031 44.2596 24.0059 43.22 24C42.5305 24.0054 41.8468 23.8731 41.2091 23.6107C40.5714 23.3484 39.9924 22.9614 39.5062 22.4725C39.02 21.9835 38.6364 21.4023 38.3777 20.7631C38.1191 20.1239 37.9907 19.4395 38 18.75ZM45.52 18.75C45.5082 18.3018 45.3645 17.867 45.1069 17.5001C44.8493 17.1331 44.4893 16.8502 44.0717 16.6868C43.6542 16.5234 43.1978 16.4867 42.7596 16.5814C42.3213 16.676 41.9207 16.8977 41.6078 17.2189C41.2949 17.54 41.0836 17.9462 41.0004 18.3868C40.9172 18.8274 40.9657 19.2827 41.1399 19.6958C41.3141 20.1089 41.6062 20.4616 41.9798 20.7095C42.3533 20.9575 42.7916 21.0898 43.24 21.09C43.5453 21.096 43.8485 21.0389 44.1307 20.9223C44.4129 20.8058 44.668 20.6323 44.8801 20.4127C45.0922 20.1931 45.2567 19.932 45.3634 19.646C45.4701 19.3599 45.5166 19.0549 45.5 18.75H45.52Z" class="ccustom" fill="#394149"></path> <path d="M60.31 13.8V23.21C60.31 26.53 57.71 27.94 55.08 27.94C54.1523 28.0161 53.2218 27.8318 52.3933 27.4078C51.5647 26.9838 50.8709 26.3369 50.39 25.54L52.92 24.08C53.125 24.4934 53.4511 24.8344 53.8549 25.0577C54.2587 25.281 54.7209 25.376 55.18 25.33C55.4725 25.366 55.7693 25.337 56.0492 25.245C56.3292 25.1529 56.5853 25.0002 56.7993 24.7976C57.0134 24.595 57.18 24.3477 57.2873 24.0732C57.3946 23.7988 57.4399 23.504 57.42 23.21V22.3C57.0709 22.7233 56.6279 23.0593 56.1263 23.2815C55.6246 23.5037 55.0781 23.6059 54.53 23.58C53.2039 23.58 51.9322 23.0532 50.9945 22.1155C50.0568 21.1779 49.53 19.9061 49.53 18.58C49.53 17.2539 50.0568 15.9822 50.9945 15.0445C51.9322 14.1068 53.2039 13.58 54.53 13.58C55.0781 13.5541 55.6246 13.6563 56.1263 13.8785C56.6279 14.1007 57.0709 14.4367 57.42 14.86V13.86L60.31 13.8ZM57.42 18.55C57.4399 18.0716 57.3163 17.5981 57.065 17.1905C56.8137 16.7829 56.4463 16.4598 56.0098 16.2627C55.5734 16.0656 55.0881 16.0036 54.6161 16.0846C54.1442 16.1656 53.7072 16.3859 53.3615 16.7172C53.0158 17.0485 52.7771 17.4757 52.6761 17.9438C52.5751 18.4119 52.6164 18.8994 52.7947 19.3438C52.9731 19.7882 53.2803 20.1691 53.6768 20.4375C54.0734 20.7059 54.5412 20.8496 55.02 20.85C55.3325 20.8725 55.6462 20.8292 55.9408 20.723C56.2355 20.6167 56.5047 20.4498 56.7308 20.233C56.957 20.0163 57.1352 19.7545 57.2539 19.4646C57.3726 19.1747 57.4292 18.8631 57.42 18.55Z" class="ccustom" fill="#394149"></path> <path d="M62 18.75C62 17.7112 62.3082 16.6958 62.8855 15.8322C63.4628 14.9686 64.2833 14.2957 65.2432 13.8987C66.2031 13.5016 67.2592 13.3982 68.2779 13.6016C69.2966 13.805 70.232 14.306 70.9659 15.0412C71.6997 15.7765 72.1989 16.7129 72.4003 17.7319C72.6018 18.751 72.4964 19.8069 72.0975 20.766C71.6986 21.7252 71.0241 22.5444 70.1595 23.1201C69.2948 23.6958 68.2788 24.002 67.24 24C66.5492 24.0067 65.8639 23.8754 65.2245 23.6138C64.5851 23.3522 64.0043 22.9656 63.5163 22.4766C63.0282 21.9876 62.6427 21.4061 62.3823 20.7662C62.122 20.1263 61.992 19.4408 62 18.75ZM69.52 18.75C69.5082 18.3014 69.3642 17.8662 69.1062 17.499C68.8482 17.1319 68.4875 16.849 68.0694 16.6859C67.6513 16.5228 67.1944 16.4867 66.7559 16.5822C66.3174 16.6776 65.9168 16.9004 65.6043 17.2225C65.2918 17.5446 65.0813 17.9518 64.9993 18.393C64.9172 18.8342 64.9671 19.2898 65.1428 19.7027C65.3185 20.1157 65.6122 20.4676 65.9871 20.7144C66.3619 20.9611 66.8012 21.0918 67.25 21.09C67.5553 21.096 67.8585 21.0389 68.1407 20.9223C68.4229 20.8058 68.678 20.6323 68.8901 20.4127C69.1022 20.1931 69.2667 19.932 69.3734 19.646C69.4801 19.3599 69.5266 19.0549 69.51 18.75H69.52Z" class="ccustom" fill="#394149"></path> <path d="M73.87 11.15C73.87 10.798 73.9744 10.4538 74.17 10.1611C74.3656 9.86836 74.6436 9.64022 74.9688 9.50549C75.2941 9.37077 75.652 9.33552 75.9973 9.4042C76.3425 9.47288 76.6597 9.64241 76.9087 9.89135C77.1576 10.1403 77.3271 10.4575 77.3958 10.8027C77.4645 11.148 77.4292 11.5059 77.2945 11.8312C77.1598 12.1564 76.9316 12.4344 76.6389 12.63C76.3462 12.8256 76.0021 12.93 75.65 12.93C75.1795 12.9248 74.7298 12.7356 74.3971 12.4029C74.0644 12.0702 73.8752 11.6205 73.87 11.15ZM74.16 13.8H77.16V23.7H74.16V13.8Z" class="ccustom" fill="#394149"></path> <path d="M90 18.75C90.0336 19.4084 89.9367 20.067 89.7148 20.6878C89.493 21.3086 89.1505 21.8794 88.7072 22.3673C88.2638 22.8553 87.7284 23.2507 87.1316 23.5309C86.5349 23.8111 85.8886 23.9705 85.23 24C84.6711 24.0372 84.111 23.9477 83.5916 23.7382C83.0721 23.5286 82.6067 23.2045 82.23 22.79V27.68H79.23V13.8H82.23V14.73C82.6072 14.3167 83.073 13.9941 83.5925 13.7863C84.112 13.5784 84.6718 13.4908 85.23 13.53C85.886 13.5595 86.5297 13.718 87.1244 13.9963C87.7191 14.2746 88.2532 14.6674 88.6961 15.1521C89.1391 15.6368 89.4822 16.204 89.7059 16.8214C89.9296 17.4387 90.0296 18.094 90 18.75ZM87 18.75C86.9882 18.2855 86.8395 17.8349 86.5727 17.4546C86.3059 17.0743 85.9327 16.7811 85.5 16.612C85.0673 16.4428 84.5943 16.4052 84.1402 16.5037C83.6862 16.6022 83.2714 16.8326 82.9477 17.1659C82.624 17.4992 82.4059 17.9206 82.3208 18.3774C82.2356 18.8341 82.2871 19.3058 82.4689 19.7334C82.6507 20.1609 82.9546 20.5253 83.3426 20.7809C83.7306 21.0365 84.1854 21.1718 84.65 21.17C84.9682 21.1835 85.2857 21.1296 85.5816 21.0118C85.8776 20.894 86.1453 20.715 86.3672 20.4865C86.5891 20.258 86.7601 19.9851 86.8692 19.6859C86.9783 19.3866 87.0228 19.0677 87 18.75Z" class="ccustom" fill="#394149"></path> <path d="M99.13 20.73C99.13 23.01 97.13 23.98 95.01 23.98C94.1453 24.0578 93.2772 23.8745 92.5178 23.4538C91.7584 23.0331 91.1426 22.3943 90.75 21.62L93.33 20.16C93.4322 20.5168 93.6538 20.8277 93.9578 21.0407C94.2618 21.2537 94.6297 21.3558 95 21.33C95.71 21.33 96.07 21.11 96.07 20.71C96.07 19.62 91.2 20.2 91.2 16.77C91.2 14.61 93.02 13.53 95.08 13.53C95.863 13.4949 96.641 13.6726 97.3311 14.0441C98.0213 14.4156 98.598 14.9671 99 15.64L96.42 17C96.306 16.742 96.1198 16.5226 95.8838 16.3682C95.6477 16.2138 95.372 16.1311 95.09 16.13C94.57 16.13 94.26 16.33 94.26 16.68C94.26 17.82 99.13 17.07 99.13 20.73Z" class="ccustom" fill="#394149"></path> <path d="M110 13.8V23.7H107V22.77C106.659 23.1817 106.226 23.5074 105.737 23.7211C105.247 23.9348 104.714 24.0303 104.18 24C102.18 24 100.47 22.57 100.47 19.9V13.8H103.47V19.45C103.446 19.6862 103.474 19.9247 103.552 20.1491C103.63 20.3734 103.755 20.5782 103.919 20.7494C104.084 20.9205 104.284 21.0539 104.505 21.1404C104.726 21.2268 104.963 21.2642 105.2 21.25C106.28 21.25 107.04 20.61 107.04 19.25V13.8H110Z" class="ccustom" fill="#394149"></path> <path d="M127 17.62V23.7H124V17.88C124 16.88 123.53 16.26 122.58 16.26C121.63 16.26 121.03 16.95 121.03 18.12V23.7H118.03V17.88C118.03 16.88 117.56 16.26 116.61 16.26C115.66 16.26 115.06 16.95 115.06 18.12V23.7H112.06V13.8H115.06V14.71C115.371 14.308 115.776 13.9896 116.241 13.7836C116.705 13.5776 117.214 13.4904 117.72 13.53C118.227 13.5035 118.733 13.6095 119.187 13.8374C119.641 14.0653 120.028 14.4074 120.31 14.83C120.64 14.3884 121.076 14.0376 121.578 13.8103C122.079 13.5829 122.631 13.4865 123.18 13.53C125.52 13.53 127 15.15 127 17.62Z" class="ccustom" fill="#394149"></path> <path d="M129.52 13.74C130.735 13.74 131.72 12.755 131.72 11.54C131.72 10.325 130.735 9.34 129.52 9.34C128.305 9.34 127.32 10.325 127.32 11.54C127.32 12.755 128.305 13.74 129.52 13.74Z" class="ccustom" fill="#394149"></path> </svg>
                <div class="field">
                    <div class="control has-icons-left">
                    <input ref={email_ref} class={email_input_class}  type="email" placeholder="hello@example.com" onkeyup={on_email_change} onkeydown={on_keydown.clone()} />
                        <span class="icon is-small is-left">
                        <i class="fa-solid fa-envelope"></i>
                        </span>
                    </div>
                    <p class="help is-danger">
                        {email_invalid_msg}
                    </p>
                </div>
                <div class="field">
                    <div class="control has-icons-left">
                    <input ref={pwd_ref} class={password_input_class} type="password" placeholder="**********" onkeyup={on_pwd_change} onkeydown={on_keydown}/>
                        <span class="icon is-small is-left">
                        <i class="fa-solid fa-lock"></i>
                        </span>
                    </div>
                    <p class="help is-danger">
                        {pwd_invalid_msg}
                    </p>
                    <p class="help is-danger">
                        {request_fail_msg.borrow().clone()}
                    </p>
                </div>
                <br />
                <button class="button is-block is-fullwidth is-primary is-medium is-rounded" onclick={on_login}>
                    {"Login"}
                </button>
                <br/>
                <nav class="level">
                <div class="level-item has-text-centered">
                    <div>
                    <a href="/forget_pwd">{"Forgot Password?"}</a>
                    </div>
                </div>
                <div class="level-item has-text-centered">
                    <div>
                    <a href="/register">{"Create an Account"}</a>
                    </div>
                </div>
                </nav>
            </div>
            </div>
        </section>
        </>
    }
}
