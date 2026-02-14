import { invoke } from "@tauri-apps/api/core";

export type SyncStatus = "none" | "synced" | "recovering" | "lost";
export type AppView = "dashboard" | "library" | "settings" | "team";

export interface TeamMemberState {
  fullTitle: string;
  syncState: SyncStatus;
}

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

  // Team
  teamMode = $state(false);
  teamMembers = $state<string[]>([]);
  teamWindows = $state<Record<string, TeamMemberState>>({});

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

  // --- Team ---

  /** Restaure la team depuis le profil */
  restoreTeamFromProfile(teamMode: boolean, teamMembers: string[]) {
    this.teamMode = teamMode;
    this.teamMembers = teamMembers;
    this.teamWindows = {};
  }

  /** Toggle le mode team */
  toggleTeamMode() {
    this.teamMode = !this.teamMode;
  }

  /** Ajouter un membre à la team */
  addTeamMember(name: string) {
    const clean = name.trim();
    if (!clean || this.teamMembers.includes(clean)) return;
    // Vérifier que ce n'est pas le meneur actuel
    if (clean === this.usableTitle) return;
    this.teamMembers = [...this.teamMembers, clean];
    this.syncTeamMember(clean);
  }

  /** Retirer un membre de la team */
  removeTeamMember(name: string) {
    this.teamMembers = this.teamMembers.filter((m) => m !== name);
    const next = { ...this.teamWindows };
    delete next[name];
    this.teamWindows = next;
  }

  /** Promouvoir un membre en meneur (swap avec le meneur actuel) */
  async setAsLeader(name: string) {
    const oldLeader = this.usableTitle;
    const memberState = this.teamWindows[name];

    // Retirer le nouveau leader des membres
    this.removeTeamMember(name);

    // Ajouter l'ancien leader comme membre
    if (oldLeader && oldLeader !== "Mon Personnage") {
      this.teamMembers = [...this.teamMembers, oldLeader];
      // Copier l'état sync de l'ancien leader vers teamWindows
      this.teamWindows = {
        ...this.teamWindows,
        [oldLeader]: {
          fullTitle: this.fullTitle,
          syncState: this.syncState,
        },
      };
    }

    // Mettre le nouveau leader en place
    this.usableTitle = name;
    this.windowTitle = name;
    if (memberState?.syncState === "synced" && memberState.fullTitle) {
      this.fullTitle = memberState.fullTitle;
      this.syncState = "synced";
      this.isLocked = true;
    } else {
      this.fullTitle = "";
      this.syncState = "none";
      this.isLocked = true;
      await this.performWindowSync(name);
    }
  }

  /** Sync un membre individuel */
  async syncTeamMember(name: string) {
    this.teamWindows = {
      ...this.teamWindows,
      [name]: { fullTitle: "", syncState: "recovering" },
    };

    try {
      const result = await invoke("get_game_title_by_name", {
        characterName: name,
      });
      this.teamWindows = {
        ...this.teamWindows,
        [name]: { fullTitle: result as string, syncState: "synced" },
      };
    } catch {
      this.teamWindows = {
        ...this.teamWindows,
        [name]: { fullTitle: "", syncState: "lost" },
      };
    }
  }

  /** Sync tous les membres de la team */
  async syncAllTeam() {
    await this.performWindowSync();
    for (const name of this.teamMembers) {
      await this.syncTeamMember(name);
    }
  }

  /** Retourne tous les fullTitle synchro (leader + team) */
  get allSyncedTitles(): string[] {
    const titles: string[] = [];
    if (this.syncState === "synced" && this.fullTitle) {
      titles.push(this.fullTitle);
    }
    if (this.teamMode) {
      for (const name of this.teamMembers) {
        const member = this.teamWindows[name];
        if (member?.syncState === "synced" && member.fullTitle) {
          titles.push(member.fullTitle);
        }
      }
    }
    return titles;
  }
}

export const windowStore = new WindowStore();
