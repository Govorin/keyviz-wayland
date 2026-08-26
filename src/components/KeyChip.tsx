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
        // El "+" es un elemento hermano más de la lista, no algo anidado
        // dentro de la parte siguiente: así el `gap` del contenedor le da
        // el mismo espacio a ambos lados, en vez de quedar pegado a un
        // lado y separado del otro.
        <span key={`${i}-contenido`} style={{ display: "inline-flex" }}>
          {i > 0 && (
            <span key={`${i}-sep`} style={separadorStyle}>
              +
            </span>
          )}
          {tieneIcono(parte) ? <KeyIcon label={parte} /> : parte}
        </span>
      ))}
    </span>
  );
}

const separadorStyle: CSSProperties = {
  opacity: 0.5,
  fontSize: "0.9em",
  marginRight: "8px",
};

const chipStyle: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  gap: "8px",
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
