import { loadOrDownloadGuide } from "$lib/services/guideService";

export interface Tab {
  id: string;
  label: string;
}

class TabStore {
  tabs = $state<Tab[]>([]);
  activeTab = $state("");
  openGuides = $state<Record<string, any>>({});

  async openGuide(id: string): Promise<string | undefined> {
    try {
      const guideData = await loadOrDownloadGuide(id);
      const tabId = `guide_${id}`;
      this.openGuides[tabId] = guideData;

      if (!this.tabs.find((t) => t.id === tabId)) {
        this.tabs.push({ id: tabId, label: guideData.name });
      }
      this.activeTab = tabId;
      return tabId;
    } catch (e) {
      console.error("Erreur chargement guide:", e);
      return undefined;
    }
  }

  closeTab(id: string) {
    this.tabs = this.tabs.filter((t) => t.id !== id);
    if (this.activeTab === id) {
      this.activeTab = this.tabs.length > 0 ? this.tabs[0].id : "general";
    }
  }

  setActiveTab(id: string) {
    this.activeTab = id;
  }

  reorderTabs(fromIndex: number, toIndex: number) {
    const item = this.tabs[fromIndex];
    this.tabs.splice(fromIndex, 1);
    this.tabs.splice(toIndex, 0, item);
  }

  /** Restaure les onglets depuis le profil sauvegardé */
  async restoreFromProfile(openTabIds: string[], activeTabId: string) {
    const loadedTabs: Tab[] = [];
    for (const tabId of openTabIds) {
      try {
        const guideId = tabId.replace("guide_", "");
        const guideData = await loadOrDownloadGuide(guideId);
        this.openGuides[tabId] = guideData;
        loadedTabs.push({ id: tabId, label: guideData.name });
      } catch (e) {
        console.error("Erreur restauration tab:", e);
      }
    }
    this.tabs = loadedTabs;

    if (activeTabId && this.tabs.find((t) => t.id === activeTabId)) {
      this.activeTab = activeTabId;
    }
  }
}

export const tabStore = new TabStore();
