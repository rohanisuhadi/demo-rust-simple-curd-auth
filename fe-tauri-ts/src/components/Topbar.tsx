import { useAuth } from "../contexts/AuthContext";

interface TopbarProps {
  onMenu: () => void;
}

const Topbar: React.FC<TopbarProps> = ({ onMenu }) => {
  const { user } = useAuth();
  return (
    <div className="topbar">
      <div className="menu-btn" onClick={onMenu}>
        ☰
      </div>

      <h3>Home</h3>

      <div className="profile">
        <span>{user?.full_name}</span>
        <img src="https://i.pravatar.cc/100" alt="profile" />
      </div>
    </div>
  );
};

export default Topbar;
