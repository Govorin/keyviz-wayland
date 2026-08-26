import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { formatCombo } from "../utils/keyLabels";

export interface KeyEntry {
  id: number;
  partes: string[];
}

const CLEAR_DELAY_MS = 1200;
let nextId = 0;

/**
 * Hook que escucha el evento "key-event" de Tauri.
 * El backend ya emite la combinación completa (ej. "Ctrl+Shift+KEY_K"),
 * por lo que solo se muestra el último combo recibido, no un historial.
 * El combo se limpia automáticamente después de CLEAR_DELAY_MS
 * milisegundos sin actividad.
 */
export function useKeyEvents(): KeyEntry[] {
  const [keys, setKeys] = useState<KeyEntry[]>([]);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const resetClearTimer = () => {
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => setKeys([]), CLEAR_DELAY_MS);
  };

  useEffect(() => {
    const unlistenPromise = listen<string>("key-event", (event) => {
      if (!event.payload) return;
      const partes = formatCombo(event.payload);
      setKeys([{ id: nextId++, partes }]);
      resetClearTimer();
    });

    return () => {
      unlistenPromise.then((f) => f());
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, []);

  return keys;
}
