import React, { useEffect, useState } from "react";
import "../styles/Form.css";
import { useNavigate, useParams } from "react-router-dom";
import { getUserById, saveUser, updateUser, User, UserReq } from "../services/userService";

const FormUser: React.FC = () => {
  const { id } = useParams();

  const navigate = useNavigate();
  const [formData, setFormData] = useState<UserReq>({
    full_name: "",
    email: "",
    password: null,
    active: true,
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;

    setFormData((prev) => ({
      ...prev,
      [name]: name === "active" ? value === "true" : value,
    }));

    // setFormData({
    //   ...formData,
    //   [e.target.name]: e.target.value,
    // });
  };

  async function submitForm() {
    try {
      if (id != null) {
        await updateUser(id!, formData);
      } else {
        await saveUser(formData);
      }
      navigate("/dashboard");
    } catch (e: any) {
      alert(e.message);
    }
  }

  async function getUser(id: string) {
    try {
      const user: User = await getUserById(id);
      setFormData({
        full_name: user.full_name,
        email: user.email,
        active: user.active,
        password: "",
      });
    } catch (err: any) {}
  }

  useEffect(() => {
    if (id != null) {
      const fetchUser = async () => {
        await getUser(id);
      };
      fetchUser();
    } else {
      setFormData({
        full_name: "",
        email: "",
        password: null,
        active: true,
      });
    }
  }, []);

  useEffect(() => {}, []);

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        submitForm();
      }}
      style={{ maxWidth: "400px", margin: "0 auto" }}
    >
      <h2>Form {id ? "Edit" : "Add"} User</h2>

      <div className="form-group">
        <label htmlFor="name">Full Name:</label>
        <input
          type="text"
          id="full_name"
          name="full_name"
          value={formData.full_name}
          onChange={handleChange}
          required
        />
      </div>

      <div className="form-group">
        <label htmlFor="email">Email:</label>
        <input type="email" id="email" name="email" value={formData.email} onChange={handleChange} required />
      </div>

      {id == null && (
        <div className="form-group">
          <label htmlFor="password">Password:</label>
          <input
            type="password"
            id="password"
            name="password"
            value={formData.password ?? ""}
            onChange={handleChange}
            required
          />
        </div>
      )}

      <div className="form-group">
        <label htmlFor="role">Status:</label>
        <select id="active" name="active" value={String(formData.active)} onChange={handleChange} required>
          <option value="">-- Select --</option>
          <option value="true">Active</option>
          <option value="false">Non Active</option>
        </select>
      </div>

      <button type="submit">Buat Pengguna</button>
    </form>
  );
};

export default FormUser;
