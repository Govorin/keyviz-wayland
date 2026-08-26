import type { JSX } from "react";

/**
 * Iconos SVG inline para teclas especiales (Enter, Backspace, Tab...).
 * Un solo <svg> 20x20 por tecla, trazo (stroke) en vez de relleno para
 * que se vea nítido en cualquier tamaño de chip. `currentColor` hereda
 * el color de texto del chip, así que no hace falta tema aparte.
 */
type IconProps = { size?: number };

const base = {
  width: "1em",
  height: "1em",
  viewBox: "0 0 20 20",
  fill: "none",
  stroke: "currentColor",
  strokeWidth: 1.6,
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
};

function EnterIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M16 4v6a2 2 0 0 1-2 2H5" />
      <path d="M8 9 4.5 12 8 15" />
    </svg>
  );
}

function BackspaceIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M7.5 5h9a1.5 1.5 0 0 1 1.5 1.5v7a1.5 1.5 0 0 1-1.5 1.5h-9L3 10z" />
      <path d="M9 8.2 12.8 12M12.8 8.2 9 12" />
    </svg>
  );
}

function TabIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M3 5v10M17 5v10" />
      <path d="M7 10h7" />
      <path d="M10.5 6.5 14 10l-3.5 3.5" />
    </svg>
  );
}

function EscIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <circle cx="10" cy="10" r="7" />
      <path d="M7.5 7.5 12.5 12.5M12.5 7.5 7.5 12.5" />
    </svg>
  );
}

function SpaceIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M4 8v4h12V8" />
    </svg>
  );
}

function CapsLockIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M10 3 15.5 9H12v6H8V9H4.5z" strokeLinejoin="round" />
    </svg>
  );
}

function ShiftIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M10 3 16.5 10H13v5H7v-5H3.5z" strokeLinejoin="round" />
    </svg>
  );
}

function ArrowUpIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M10 16V4M5 9l5-5 5 5" />
    </svg>
  );
}

function ArrowDownIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M10 4v12M5 11l5 5 5-5" />
    </svg>
  );
}

function ArrowLeftIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M16 10H4M9 5l-5 5 5 5" />
    </svg>
  );
}

function ArrowRightIcon({ size }: IconProps) {
  return (
    <svg {...base} width={size} height={size}>
      <path d="M4 10h12M11 5l5 5-5 5" />
    </svg>
  );
}

/** Mapa etiqueta legible → componente de icono SVG. */
const ICONS: Record<string, (props: IconProps) => JSX.Element> = {
  Enter: EnterIcon,
  "⌫": BackspaceIcon,
  Tab: TabIcon,
  Esc: EscIcon,
  Space: SpaceIcon,
  Caps: CapsLockIcon,
  Shift: ShiftIcon,
  "↑": ArrowUpIcon,
  "↓": ArrowDownIcon,
  "←": ArrowLeftIcon,
  "→": ArrowRightIcon,
};

/**
 * Modificadores sin icono SVG propio: se usan los glifos Unicode
 * estándar de macOS (⌃⌥⌘), reconocidos también fuera de ese ecosistema
 * en la mayoría de overlays de teclado para streaming. AltGr no tiene
 * un glifo universalmente reconocido, así que se deja como texto.
 */
const GLIFOS_TEXTO: Record<string, string> = {
  Ctrl: "⌃",
  Alt: "⌥",
  Super: "⌘",
};

/** true si existe un icono (SVG o glifo de texto) para esta etiqueta. */
export function tieneIcono(label: string): boolean {
  return label in ICONS || label in GLIFOS_TEXTO;
}

export default function KeyIcon({ label, size = 22 }: { label: string; size?: number }) {
  const Icono = ICONS[label];
  if (Icono) return <Icono size={size} />;

  const glifo = GLIFOS_TEXTO[label];
  if (glifo) return <span style={{ fontSize: size, lineHeight: 1 }}>{glifo}</span>;

  return null;
}
