import { useEffect, useState } from "react";
import DataTable from "../components/DataTable";
import { delUserById, getUsers, PagUserRes } from "../services/userService";
import DeleteConfirmModal from "../components/DeleteConfirmModalProps";

function Home() {
  const [currentPage, setCurrentPage] = useState(1);

  const [users, setUsers] = useState<PagUserRes>({
    data: [],
    page: 1,
    per_page: 10,
    total: 0,
    total_pages: 0,
  });

  const [id, setId] = useState<string>("");
  async function getPagUsers() {
    try {
      const users: PagUserRes = await getUsers(currentPage, 10);
      setUsers(users);
    } catch (err: any) {}
  }

  const fetchUsers = async () => {
    await getPagUsers();
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const onDelete = (id: string) => {
    setOpen(true);
    setId(id);
  };

  async function delUserApi(id: string) {
    try {
      await delUserById(id);
      fetchUsers();
    } catch (err: any) {
      alert(err.message);
    }
  }

  const handleDelete = () => {
    delUserApi(id);
    setOpen(false);
  };

  const [open, setOpen] = useState(false);

  return (
    <div>
      <div className="table-responsive-wrapper">
        <DataTable
          data={users}
          onDeleted={onDelete}
          per_page={users.per_page}
          page={currentPage}
          setPage={setCurrentPage}
        />
      </div>

      <DeleteConfirmModal
        open={open}
        title="Delete user?"
        message={`Are you sure you delete ID ${id}?`}
        onConfirm={handleDelete}
        onCancel={() => setOpen(false)}
      />
    </div>
  );
}

export default Home;
