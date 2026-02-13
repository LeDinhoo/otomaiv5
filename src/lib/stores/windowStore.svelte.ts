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

  // Sync state
  syncState = $state<SyncStatus>("none");
  isLocked = $state(false);

  // Token pour annuler les syncs obsolètes
  private _syncToken = 0;

  setView(view: AppView) {
    this.currentView = view;
  }

  toggleView(view: AppView) {
    this.currentView = this.currentView === view ? "dashboard" : view;
  }

  async performWindowSync(nameToFind?: string) {
    const name = nameToFind ?? this.usableTitle;
    if (!name) return;

    const token = ++this._syncToken;

    try {
      const result = await invoke("sync_window_title", {
        characterName: name,
      });

      // Ignorer si un nouveau sync a été lancé entre-temps
      if (token !== this._syncToken) return;

      this.fullTitle = result as string;
      this.windowTitle = result as string;
      const cleanPseudo = (result as string).split(" - ")[0];
      if (this.usableTitle !== cleanPseudo) this.usableTitle = cleanPseudo;
      this.status = "Synchronisé ✅";
      this.syncState = "synced";
      this.isLocked = true;
    } catch (e) {
      if (token !== this._syncToken) return;

      console.error("Erreur synchro:", e);
      this.status = "Fenêtre introuvable ❌";
      this.syncState = "lost";
      this.isLocked = true;
    }
  }

  /** Verrouiller/déverrouiller le champ nom */
  toggleNameLock() {
    if (!this.isLocked) {
      if (!this.usableTitle || this.usableTitle.trim() === "") return;
      this.isLocked = true;
    } else {
      this.isLocked = false;
    }
  }

  /** Lancer ou relancer la synchronisation */
  async triggerSync() {
    if (!this.usableTitle || this.usableTitle.trim() === "") return;
    this.isLocked = true;
    this.syncState = "recovering";
    await this.performWindowSync();
  }

  /** Relancer la recherche quand la connexion est perdue */
  async triggerRecovery() {
    try {
      this.syncState = "recovering";
      await invoke("trigger_auto_recovery");
    } catch (e) {
      console.error("Erreur trigger recovery:", e);
    }
  }

  /** Restaure depuis le profil sauvegardé */
  restoreFromProfile(characterName: string) {
    this.windowTitle = characterName;
    this.usableTitle = characterName.split(" - ")[0];
  }
}

export const windowStore = new WindowStore();
