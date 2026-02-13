import {
  loadProfileIndex,
  saveProfileIndex,
  loadProfile,
  saveProfile,
  deleteProfileFile,
  generateProfileId,
  type ProfileEntry,
  type ProfileIndex,
} from "$lib/services/profileService";

import { tabStore } from "$lib/stores/tabStore.svelte";
import { guideStore } from "$lib/stores/guideStore.svelte";
import { windowStore } from "$lib/stores/windowStore.svelte";

class ProfileStore {
  profiles = $state<ProfileEntry[]>([]);
  activeProfileId = $state("default");

  get activeProfileName(): string {
    return (
      this.profiles.find((p) => p.id === this.activeProfileId)?.name ??
      "Défaut"
    );
  }

  /** Charge l'index et le profil actif au démarrage */
  async init() {
    const index = await loadProfileIndex();
    this.profiles = index.profiles;
    this.activeProfileId = index.activeProfileId;

    // Charger le profil actif dans les stores
    await this._loadProfileIntoStores(this.activeProfileId);

    windowStore.isLoaded = true;
    await windowStore.performWindowSync();
    windowStore.status = "Prêt";
  }

  /** Sauvegarde le profil actif (appelé par l'auto-save) */
  async saveCurrentProfile() {
    await saveProfile(this.activeProfileId, {
      characterName: windowStore.windowTitle,
      openTabIds: tabStore.tabs.map((t) => t.id),
      activeTabId: tabStore.activeTab,
      guideProgress: $state.snapshot(guideStore.guideProgress),
      checkboxStates: $state.snapshot(guideStore.checkboxStates),
    });
  }

  /** Switcher vers un autre profil */
  async switchProfile(id: string) {
    if (id === this.activeProfileId) return;

    // Sauvegarder le profil actuel
    await this.saveCurrentProfile();

    // Charger le nouveau
    windowStore.isLoaded = false;
    this.activeProfileId = id;
    await this._loadProfileIntoStores(id);

    // Mettre à jour l'index
    await this._saveIndex();

    windowStore.isLoaded = true;
    await windowStore.performWindowSync();
  }

  /** Créer un nouveau profil et switcher dessus */
  async createProfile(name: string) {
    const existingIds = this.profiles.map((p) => p.id);
    const id = generateProfileId(name, existingIds);

    this.profiles.push({ id, name });

    // Sauvegarder un profil vide
    await saveProfile(id, {
      characterName: name,
      openTabIds: [],
      activeTabId: "general",
      guideProgress: {},
      checkboxStates: {},
    });

    await this._saveIndex();
    await this.switchProfile(id);
  }

  /** Renommer un profil */
  async renameProfile(id: string, newName: string) {
    const entry = this.profiles.find((p) => p.id === id);
    if (entry) {
      entry.name = newName;
      await this._saveIndex();
    }
  }

  /** Supprimer un profil */
  async deleteProfile(id: string) {
    if (this.profiles.length <= 1) return;

    this.profiles = this.profiles.filter((p) => p.id !== id);
    await deleteProfileFile(id);

    // Si on supprime le profil actif, switcher sur le premier disponible
    if (this.activeProfileId === id) {
      await this.switchProfile(this.profiles[0].id);
    }

    await this._saveIndex();
  }

  // --- Private ---

  private async _loadProfileIntoStores(profileId: string) {
    const profile = await loadProfile(profileId);

    windowStore.restoreFromProfile(profile.characterName || "Mon Personnage");
    guideStore.restoreFromProfile(
      profile.guideProgress || {},
      profile.checkboxStates || {},
    );
    await tabStore.restoreFromProfile(
      profile.openTabIds || [],
      profile.activeTabId || "",
    );
  }

  private async _saveIndex() {
    await saveProfileIndex({
      activeProfileId: this.activeProfileId,
      profiles: $state.snapshot(this.profiles),
    });
  }
}

export const profileStore = new ProfileStore();
