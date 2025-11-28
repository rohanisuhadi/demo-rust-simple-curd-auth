import axios, { AxiosResponse } from "axios";
import api from "./api";

export interface UserLog {
  id: string;
  full_name: string;
}

export interface LoggedInUser {
  id: string;
  full_name: string;
  email: string;
  token: string;
  expired_at: string;
}

export interface LoginResponse {
  status: boolean;
  data: LoggedInUser;
  token: string;
}

export interface ApiRes<T> {
  status: boolean;
  message: string;
  data: T;
}

export interface ApiError {
  message: string;
}

export const login = async (email: string, password: string): Promise<LoginResponse> => {
  try {
    const response: AxiosResponse<LoginResponse> = await api.post("/auth/login", {
      email,
      password,
    });
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error) && error.response) {
      const apiError: ApiError = error.response.data;
      throw new Error(apiError.message || "Login failed.");
    } else if (axios.isAxiosError(error)) {
      throw new Error("Network error. Please check your connection.");
    } else {
      throw new Error("An unexpected error occurred.");
    }
  }
};
