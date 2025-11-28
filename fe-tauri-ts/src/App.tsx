import React from "react";
import { Routes, Route } from "react-router-dom";

import Layout from "./components/Layout";
import LoginPage from "./pages/Login";
import HomePage from "./pages/Home";
import AddUserPage from "./pages/FormUser";
import ProtectedRoute from "./components/ProtectedRoute";

const App: React.FC = () => {
  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route
        path="/dashboard"
        element={
          <Layout>
            <ProtectedRoute>
              <HomePage />
            </ProtectedRoute>
          </Layout>
        }
      />
      <Route
        path="/add-user"
        element={
          <Layout>
            <ProtectedRoute>
              <AddUserPage />
            </ProtectedRoute>
          </Layout>
        }
      />

      <Route
        path="/edit-user/:id"
        element={
          <Layout>
            <ProtectedRoute>
              <AddUserPage />
            </ProtectedRoute>
          </Layout>
        }
      />

      <Route
        path="/profile"
        element={
          <Layout>
            <ProtectedRoute>
              <AddUserPage />
            </ProtectedRoute>
          </Layout>
        }
      />
      <Route path="/" element={<LoginPage />} />
    </Routes>
  );
};

export default App;
