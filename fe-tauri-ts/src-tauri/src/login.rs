use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub fn login(username: String, password: String) -> LoginResponse {
    if username == "rohanisuhadi" && password == "567543" {
        LoginResponse {
            success: true,
            message: "Login success".into(),
        }
    } else {
        LoginResponse {
            success: false,
            message: "Username and password wrong".into(),
        }
    }
}
