import { lazy, StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@/index.css";

const App = lazy(() => import("@/app"));
const BrowserRouter = lazy(() =>
  import("react-router-dom").then((module) => ({ default: module.BrowserRouter }))
);

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </StrictMode>
);
