import React from "react";
import Pagination from "./Pagination";
import RawTable from "./RawTable";
import { PagUserRes } from "../services/userService";

interface DataTableProps {
  data: PagUserRes;
  per_page: number;
  page: number;
  onDeleted: (id: string) => void;
  setPage: (id: number) => void;
}

const DataTable: React.FC<DataTableProps> = ({ data, per_page, onDeleted, setPage, page }) => {
  const totalItems = data.total;
  const currentItems = data.data;

  const handlePageChange = (page: number) => {
    setPage(page);
  };

  return (
    <div className="table-responsive-wrapper">
      <table className="user-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Full Name</th>
            <th>Email</th>
            <th>Action</th>
            <th className="user-table-th-non">Action</th>
          </tr>
        </thead>
        <tbody>
          {currentItems.map((user) => (
            <RawTable user={user} onDelete={onDeleted} key={user.id} />
          ))}
          {currentItems.length === 0 && (
            <tr key={currentItems.length}>
              <td colSpan={4} style={{ textAlign: "center" }}>
                Tidak ada data yang tersedia.
              </td>
            </tr>
          )}
        </tbody>
      </table>

      <Pagination
        totalItems={totalItems}
        itemsPerPage={per_page}
        currentPage={page}
        onPageChange={handlePageChange}
        maxPagesToShow={5}
      />
    </div>
  );
};

export default DataTable;
