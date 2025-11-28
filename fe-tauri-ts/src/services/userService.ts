import axios, { AxiosResponse } from "axios";
import api from "./api";
import { ApiError, ApiRes } from "./auth";

export interface User {
  id: string;
  full_name: string;
  phone_number: string;
  email: string;
  active: boolean;
  created_at: string;
}
export interface UserReq {
  full_name: string;
  email: string;
  password: string | null;
  active: boolean;
}

export interface PagUserRes {
  data: User[];
  page: number;
  per_page: number;
  total: number;
  total_pages: number;
}

export const getUsers = async (page: number = 1, limit: number = 2): Promise<PagUserRes> => {
  try {
    const response: AxiosResponse<ApiRes<PagUserRes>> = await api.get("/user/all", {
      params: {
        page: page,
        per_page: limit,
      },
    });
    return response.data.data;
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

export const postUser = async (req: UserReq): Promise<PagUserRes> => {
  try {
    const response: AxiosResponse<ApiRes<PagUserRes>> = await api.post("/user/all", req);
    return response.data.data;
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

export const getUserById = async (id: string): Promise<User> => {
  try {
    const response: AxiosResponse<ApiRes<User>> = await api.get("/user/" + id);
    return response.data.data;
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

export const delUserById = async (id: string): Promise<User> => {
  try {
    const response: AxiosResponse<ApiRes<User>> = await api.delete("/user/" + id);
    return response.data.data;
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

export const updateUser = async (id: string, req: UserReq): Promise<PagUserRes> => {
  try {
    const response: AxiosResponse<ApiRes<PagUserRes>> = await api.put("/user/" + id, req);
    return response.data.data;
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

export const saveUser = async (req: UserReq): Promise<PagUserRes> => {
  try {
    const response: AxiosResponse<ApiRes<PagUserRes>> = await api.post("/user", req);
    return response.data.data;
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
