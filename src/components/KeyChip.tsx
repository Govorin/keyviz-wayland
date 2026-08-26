import type { CSSProperties } from "react";
import KeyIcon, { tieneIcono } from "./KeyIcon";

interface KeyChipProps {
  partes: string[];
}

/**
 * Chip visual para una combinación de teclas. Cada parte (Ctrl, Shift,
 * K...) se separa con " + ", salvo que tenga icono propio (Enter,
 * Backspace, flechas...), en cuyo caso se muestra solo el icono, sin
 * texto — como en la mayoría de overlays de teclas para streaming.
 */
export default function KeyChip({ partes }: KeyChipProps) {
  return (
    <span style={chipStyle}>
      {partes.map((parte, i) => (
        <span key={i} style={parteStyle}>
          {i > 0 && <span style={separadorStyle}>+</span>}
          {tieneIcono(parte) ? <KeyIcon label={parte} /> : parte}
        </span>
      ))}
    </span>
  );
}

const parteStyle: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  gap: "8px",
};

const separadorStyle: CSSProperties = {
  opacity: 0.5,
  marginRight: "8px",
  fontSize: "0.9em",
};

const chipStyle: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  minWidth: "40px",
  height: "52px",
  padding: "0 14px",
  background: "rgba(255, 255, 255, 0.10)",
  border: "1px solid rgba(255, 255, 255, 0.18)",
  borderBottom: "3px solid rgba(255, 255, 255, 0.25)",
  borderRadius: "10px",
  color: "#FFFFFF",
  fontSize: "1.1rem",
  fontFamily: "'Inter', 'SF Pro Display', system-ui, sans-serif",
  fontWeight: 600,
  letterSpacing: "0.03em",
  boxShadow: "0 4px 12px rgba(0,0,0,0.4)",
  animation: "chipIn 0.12s cubic-bezier(0.34, 1.56, 0.64, 1)",
  whiteSpace: "nowrap",
};
