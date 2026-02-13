import { invoke } from "@tauri-apps/api/core";

export type SyncStatus = "none" | "synced" | "recovering" | "lost";
export type AppView = "dashboard" | "library" | "settings";

class WindowStore {
  windowTitle = $state("Mon Personnage");
  usableTitle = $state("Mon Personnage");
  fullTitle = $state("");
  status = $state("Initialisation...");
  currentView = $state<AppView>("dashboard");
  isLoaded = $state(false);

  // Sync state (anciennement dans TitleBar)
  syncState = $state<SyncStatus>("none");
  isLocked = $state(false);

  setView(view: AppView) {
    this.currentView = view;
  }

  toggleView(view: AppView) {
    this.currentView = this.currentView === view ? "dashboard" : view;
  }

  async performWindowSync(nameToFind?: string) {
    const name = nameToFind ?? this.usableTitle;
    if (!name) return;
    try {
      const result = await invoke("sync_window_title", {
        characterName: name,
      });
      this.fullTitle = result as string;
      this.windowTitle = result as string;
      const cleanPseudo = (result as string).split(" - ")[0];
      if (this.usableTitle !== cleanPseudo) this.usableTitle = cleanPseudo;
      this.status = "Synchronisé ✅";
      this.syncState = "synced";
      this.isLocked = true;
    } catch (e) {
      console.error("Erreur synchro:", e);
      this.status = "Fenêtre introuvable ❌";
      this.syncState = "lost";
      this.isLocked = true;
    }
  }

  async handleLockAction() {
    if (!this.isLocked) {
      if (!this.usableTitle || this.usableTitle.trim() === "") return;
      this.isLocked = true;
      this.syncState = "recovering";
      await this.performWindowSync();
    } else {
      if (this.syncState === "synced") {
        this.isLocked = false;
        this.syncState = "none";
      } else if (this.syncState === "lost") {
        try {
          await invoke("trigger_auto_recovery");
        } catch (e) {
          console.error("Erreur trigger recovery:", e);
        }
      }
    }
  }

  /** Restaure depuis le profil sauvegardé */
  restoreFromProfile(characterName: string) {
    this.windowTitle = characterName;
    this.usableTitle = characterName.split(" - ")[0];
  }
}

export const windowStore = new WindowStore();
