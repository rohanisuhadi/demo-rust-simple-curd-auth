import { useState } from "react";
// import { invoke, isTauri } from "@tauri-apps/api/core";
import "../styles/App.css";
import "../styles/Login.css";
import { useAuth } from "../contexts/AuthContext";

function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  type GreetMsg = {
    success: boolean;
    message: string | null;
  };

  const [greetMsg, setGreetMsg] = useState<GreetMsg>({
    success: false,
    message: null,
  });

  const { loginApi } = useAuth();

  async function login() {
    try {
      await loginApi(username, password);
    } catch (err: any) {
      setGreetMsg({
        success: false,
        message: err.message,
      });
    }
  }

  return (
    <main className="container">
      <h1>Welcome App Demo - Server Side</h1>

      <div className="row">
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
      </div>

      <form
        className="form-login"
        onSubmit={(e) => {
          e.preventDefault();
          login();
        }}
      >
        <div className="login-container">
          <input
            className={`${!greetMsg.success && greetMsg.message === null ? "login-input" : "input-error login-input"}`}
            placeholder="Email"
            onChange={(e) => setUsername(e.target.value)}
          />

          <input
            className={`${!greetMsg.success && greetMsg.message === null ? "login-input" : "input-error login-input"}`}
            placeholder="Password"
            type="password"
            onChange={(e) => setPassword(e.target.value)}
          />

          <button className="login-button" type="submit">
            Login
          </button>
        </div>
      </form>

      <p>
        User Demo <br /> email: test_a@gmail.com, password: 12345678
        <br />
        <span className={`${greetMsg.success ? "" : "error-text"}`}>{greetMsg.message}</span>
      </p>
    </main>
  );
}

export default Login;
