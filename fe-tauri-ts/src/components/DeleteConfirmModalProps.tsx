import { FC } from "react";

interface DeleteConfirmModalProps {
  open: boolean;
  title?: string;
  message?: string;
  onConfirm: () => void;
  onCancel: () => void;
}

const DeleteConfirmModal: FC<DeleteConfirmModalProps> = ({
  open,
  title = "Delete Confirmation",
  message = "Are you sure you want to delete this item?",
  onConfirm,
  onCancel,
}) => {
  if (!open) return null;

  return (
    <div
      style={{
        position: "fixed",
        inset: 0,
        background: "rgba(0,0,0,0.4)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        zIndex: 9999,
      }}
    >
      <div
        style={{
          background: "white",
          padding: 20,
          borderRadius: 8,
          width: 360,
          maxWidth: "90%",
        }}
      >
        <h3 style={{ margin: 0, fontSize: 18 }}>{title}</h3>
        <p style={{ marginTop: 10 }}>{message}</p>

        <div
          style={{
            marginTop: 20,
            display: "flex",
            justifyContent: "flex-end",
            gap: 10,
          }}
        >
          <button
            onClick={onCancel}
            style={{
              padding: "8px 14px",
              borderRadius: 6,
              border: "1px solid #ccc",
              background: "#eee",
              cursor: "pointer",
            }}
          >
            Batal
          </button>

          <button
            onClick={onConfirm}
            style={{
              padding: "8px 14px",
              borderRadius: 6,
              background: "#e53935",
              border: "none",
              color: "white",
              cursor: "pointer",
            }}
          >
            Hapus
          </button>
        </div>
      </div>
    </div>
  );
};

export default DeleteConfirmModal;
