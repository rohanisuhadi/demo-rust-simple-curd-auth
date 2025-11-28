import React, { useState, ReactNode } from "react";
import Sidebar from "./Sidebar";
import Topbar from "./Topbar";
import "../styles/layout.css";

interface LayoutProps {
  children: ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const [open, setOpen] = useState(true);

  return (
    <div className="layout">
      <Sidebar isOpen={open} setOpen={setOpen} />

      <div style={{ flex: 1 }}>
        <Topbar onMenu={() => setOpen(!open)} />
        <div className="content">{children}</div>
      </div>
    </div>
  );
};

export default Layout;
