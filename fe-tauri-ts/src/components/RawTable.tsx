import React, { useState } from "react";
import { usePage } from "../contexts/PageContext";
import { User } from "../services/userService";
import { useNavigate } from "react-router-dom";

interface UserRowProps {
  user: User;
  onDelete: (id: string) => void;
}

const RawTable: React.FC<UserRowProps> = ({ user, onDelete }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const navigate = useNavigate();
  const { isMobile } = usePage();
  return (
    <tr key={user.id}>
      <td data-label="ID">{user.id}</td>
      <td data-label="Full Name">{user.full_name}</td>
      <td data-label="Email">{user.email}</td>
      <td data-label="Status">{user.active ? "Active" : "Non Active"}</td>

      {!isExpanded && isMobile && (
        <div
          style={{ textAlign: "center", cursor: "pointer" }}
          className="btn-icon expand-toggle"
          onClick={() => setIsExpanded(true)}
          aria-expanded={isExpanded}
        >
          ▼
        </div>
      )}

      {(isExpanded || !isMobile) && (
        <td className="action-cell" style={{ width: 210 }}>
          <button
            className="btn btn-edit"
            style={{ marginRight: 10 }}
            onClick={() => navigate("/edit-user/" + user.id)}
          >
            <span className="icon">✏️</span> Edit
          </button>
          <button className="btn btn-delete" onClick={() => onDelete(user.id)}>
            <span className="icon">🗑️</span> Delete
          </button>
        </td>
      )}
    </tr>
  );
};

export default RawTable;
