import { createContext, useContext, useState } from "react";

type PageContextType = {
  page: string;
  setPage: (value: string) => void;
  isMobile: boolean;
};

const MOBILE_BREAKPOINT = 600;

const PageContext = createContext<PageContextType | null>(null);

export function PageProvider({ children }: { children: React.ReactNode }) {
  const [page, setPage] = useState<string>("login");
  const isMobile = window.innerWidth < MOBILE_BREAKPOINT;

  return <PageContext.Provider value={{ page, setPage, isMobile }}>{children}</PageContext.Provider>;
}

export function usePage() {
  const ctx = useContext(PageContext);
  if (!ctx) {
    throw new Error("usePage must be used inside PageProvider");
  }
  return ctx;
}
