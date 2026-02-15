import { invoke } from "@tauri-apps/api/core";
import {
  getCurrentWindow,
  LogicalSize,
  LogicalPosition,
} from "@tauri-apps/api/window";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export type SyncStatus = "none" | "synced" | "recovering" | "lost";
export type AppView = "dashboard" | "library" | "settings" | "team";

export interface TeamMemberState {
  fullTitle: string;
  syncState: SyncStatus;
  breedIcon: number | null;
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
  breedIcon = $state<number | null>(null);
  teamMemberBreeds = $state<Record<string, number>>({});
  leaderKeybind = $state<string | null>(null);
  teamMemberKeybinds = $state<Record<string, string>>({});

  // Mini mode
  isMini = $state(false);

  // Guide controls
  autoPilot = $state(false);
  listenKeys = $state(false);

  // Combat watcher
  combatWatcherActive = $state(false);
  private _savedMaxPos: { x: number; y: number } | null = null;
  private _savedMaxHeight: number | null = null;
  private _savedMiniPos: { x: number; y: number } | null = null;

  // Token pour annuler les syncs obsolètes
  private _syncToken = 0;

  setView(view: AppView) {
    this.currentView = view;
  }

  toggleView(view: AppView) {
    this.currentView = this.currentView === view ? "dashboard" : view;
  }

  async toggleMini() {
    const win = getCurrentWindow();
    const factor = await win.scaleFactor();
    const size = await win.innerSize();
    const pos = await win.outerPosition();
    const width = Math.round(size.width / factor);
    const curPos = {
      x: Math.round(pos.x / factor),
      y: Math.round(pos.y / factor),
    };

    if (this.isMini) {
      // Sauvegarder la position mini
      this._savedMiniPos = curPos;
      // Restaurer la taille et position max
      await win.setMinSize(new LogicalSize(330, 500));
      await win.setSize(
        new LogicalSize(width, this._savedMaxHeight ?? 900),
      );
      if (this._savedMaxPos) {
        await win.setPosition(
          new LogicalPosition(this._savedMaxPos.x, this._savedMaxPos.y),
        );
      }
      await win.setResizable(true);
      this.isMini = false;
    } else {
      // Sauvegarder la position et hauteur max
      this._savedMaxPos = curPos;
      this._savedMaxHeight = Math.round(size.height / factor);
      // Passer en mode mini
      await win.setResizable(false);
      await win.setMinSize(null);
      await win.setSize(new LogicalSize(width, 33));
      if (this._savedMiniPos) {
        await win.setPosition(
          new LogicalPosition(this._savedMiniPos.x, this._savedMiniPos.y),
        );
      }
      this.isMini = true;
    }
  }

  async toggleCombatWatcher() {
    if (this.combatWatcherActive) {
      await invoke("stop_combat_watcher");
      this.combatWatcherActive = false;
    } else {
      await invoke("start_combat_watcher", {
        combatStartImage: "resources/ui/combat_start.png",
        combatEndImages: [
          "resources/ui/combat_end_1.png",
          "resources/ui/combat_end_2.png",
        ],
      });
      this.combatWatcherActive = true;
    }
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
  restoreTeamFromProfile(teamMode: boolean, teamMembers: string[], breedIcon: number | null, teamMemberBreeds: Record<string, number>, leaderKeybind: string | null, teamMemberKeybinds: Record<string, string>) {
    this.teamMode = teamMode;
    this.teamMembers = teamMembers;
    this.teamWindows = {};
    this.breedIcon = breedIcon;
    this.teamMemberBreeds = teamMemberBreeds;
    this.leaderKeybind = leaderKeybind;
    this.teamMemberKeybinds = teamMemberKeybinds;
  }

  /** Changer l'icône de classe du meneur */
  setBreedIcon(breedId: number | null) {
    this.breedIcon = breedId;
  }

  /** Changer l'icône de classe d'un membre */
  setMemberBreedIcon(name: string, breedId: number | null) {
    if (breedId === null) {
      const next = { ...this.teamMemberBreeds };
      delete next[name];
      this.teamMemberBreeds = next;
    } else {
      this.teamMemberBreeds = { ...this.teamMemberBreeds, [name]: breedId };
    }
  }

  /** Changer le keybind du meneur */
  setLeaderKeybind(key: string | null) {
    this.leaderKeybind = key;
  }

  /** Changer le keybind d'un membre */
  setMemberKeybind(name: string, key: string | null) {
    if (key === null) {
      const next = { ...this.teamMemberKeybinds };
      delete next[name];
      this.teamMemberKeybinds = next;
    } else {
      this.teamMemberKeybinds = { ...this.teamMemberKeybinds, [name]: key };
    }
  }

  /** Retourne le mapping keybind -> fullTitle pour le Rust */
  get focusKeybinds(): Record<string, string> {
    const binds: Record<string, string> = {};
    if (this.leaderKeybind && this.syncState === "synced" && this.fullTitle) {
      binds[this.leaderKeybind] = this.fullTitle;
    }
    if (this.teamMode) {
      for (const name of this.teamMembers) {
        const key = this.teamMemberKeybinds[name];
        const member = this.teamWindows[name];
        if (key && member?.syncState === "synced" && member.fullTitle) {
          binds[key] = member.fullTitle;
        }
      }
    }
    return binds;
  }

  /** Toggle le mode team */
  toggleTeamMode() {
    this.teamMode = !this.teamMode;
    if (this.teamMode) {
      this.openFocusOverlay();
    } else {
      this.closeFocusOverlay();
    }
  }

  /** Ouvrir la fenêtre flottante de focus */
  async openFocusOverlay() {
    try {
      const existing = await WebviewWindow.getByLabel("focus-overlay");
      if (existing) return; // déjà ouverte

      new WebviewWindow("focus-overlay", {
        url: "/focus-overlay",
        title: "Focus",
        width: 280,
        height: 36,
        decorations: false,
        resizable: false,
        alwaysOnTop: true,
        transparent: true,
        shadow: false,
        skipTaskbar: true,
      });
    } catch (e) {
      console.error("Erreur ouverture focus overlay:", e);
    }
  }

  /** Fermer la fenêtre flottante de focus */
  async closeFocusOverlay() {
    try {
      const win = await WebviewWindow.getByLabel("focus-overlay");
      if (win) await win.close();
    } catch (e) {
      console.error("Erreur fermeture focus overlay:", e);
    }
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
    const nextWindows = { ...this.teamWindows };
    delete nextWindows[name];
    this.teamWindows = nextWindows;
    const nextBreeds = { ...this.teamMemberBreeds };
    delete nextBreeds[name];
    this.teamMemberBreeds = nextBreeds;
    const nextKeybinds = { ...this.teamMemberKeybinds };
    delete nextKeybinds[name];
    this.teamMemberKeybinds = nextKeybinds;
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
    const existingBreed = this.teamWindows[name]?.breedIcon ?? null;
    this.teamWindows = {
      ...this.teamWindows,
      [name]: { fullTitle: "", syncState: "recovering", breedIcon: existingBreed },
    };

    try {
      const result = await invoke("get_game_title_by_name", {
        characterName: name,
      });
      this.teamWindows = {
        ...this.teamWindows,
        [name]: { fullTitle: result as string, syncState: "synced", breedIcon: existingBreed },
      };
    } catch {
      this.teamWindows = {
        ...this.teamWindows,
        [name]: { fullTitle: "", syncState: "lost", breedIcon: existingBreed },
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
