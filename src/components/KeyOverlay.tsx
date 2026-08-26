import type { CSSProperties } from "react";
import { useKeyEvents } from "../hooks/useKeyEvents";
import KeyChip from "./KeyChip";

/**
 * Contenedor principal del overlay.
 * Solo se encarga del layout y de delegar:
 *  - lógica de eventos → useKeyEvents
 *  - render de cada tecla → KeyChip
 */
export default function KeyOverlay() {
  const keys = useKeyEvents();

  if (keys.length === 0) return null;

  return (
    <div data-tauri-drag-region style={wrapperStyle}>
      {keys.map((k) => (
        <KeyChip key={k.id} partes={k.partes} />
      ))}
    </div>
  );
}

const wrapperStyle: CSSProperties = {
  display: "inline-flex",
  flexDirection: "row",
  alignItems: "center",
  justifyContent: "center",
  padding: "10px 18px",
  minWidth: "0",
  cursor: "move",
  background: "rgba(18, 18, 22, 0.82)",
  backdropFilter: "blur(12px)",
  WebkitBackdropFilter: "blur(12px)",
  borderRadius: "14px",
  border: "1px solid rgba(255, 255, 255, 0.08)",
  boxSizing: "border-box",
  animation: "fadeIn 0.15s ease",
};
