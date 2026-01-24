import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs) {
  return twMerge(clsx(inputs));
}

import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export async function showCustomNotification(
  title = "Notification",
  message = "Action terminée !",
) {
  // On utilise un label unique basé sur le timestamp pour permettre plusieurs notifs
  const label = `notif-${Date.now()}`;

  // On passe les infos via l'URL (plus simple que les events pour commencer)
  const url = `/notification?title=${encodeURIComponent(title)}&message=${encodeURIComponent(message)}`;

  const webview = new WebviewWindow(label, {
    url: url,
    title: "Notification",
    width: 400,
    height: 200,
    decorations: true, // <--- METTRE A TRUE (pour voir la barre de titre)
    transparent: false, // <--- METTRE A FALSE (pour avoir un fond blanc par défaut)
    alwaysOnTop: true,
    skipTaskbar: false, // <--- METTRE A FALSE (pour la voir dans la barre des tâches)
    focus: true,
    resizable: true,
    x: 500, // <--- Positionne là bien au centre
    y: 300,
  });

  webview.once("tauri://created", () => {
    console.log("Notification créée");
  });

  webview.once("tauri://error", (e) => {
    console.error("Erreur création notification:", e);
  });
}
// eslint-disable-next-line @typescript-eslint/no-explicit-any
