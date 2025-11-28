import React from "react";
import { Link } from "react-router-dom";
import { usePage } from "../contexts/PageContext";
import { useAuth } from "../contexts/AuthContext";

interface SidebarProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
}

const Sidebar: React.FC<SidebarProps> = ({ isOpen, setOpen }) => {
  const { setPage, isMobile } = usePage();
  const { logout } = useAuth();

  return (
    <div className={`sidebar ${isOpen ? "show-and-move" : "hide-and-move closed"}`}>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          textAlign: "justify",
        }}
      >
        <h2>Dashboard</h2>
        <div className="menu-btn-mob" onClick={() => setOpen(false)}>
          ☰
        </div>
      </div>

      <Link to="/dashboard" onClick={() => isMobile && setOpen(false)}>
        Home
      </Link>
      <Link to="/add-user" onClick={() => isMobile && setOpen(false)}>
        Add User
      </Link>
      <Link
        to="/login"
        onClick={() => {
          logout();
        }}
      >
        Logout
      </Link>
      <Link to="/login" onClick={() => setPage("login")}>
        Login
      </Link>
    </div>
  );
};

export default Sidebar;
