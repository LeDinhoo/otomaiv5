class GuideStore {
  guideProgress = $state<Record<string, number>>({});
  checkboxStates = $state<Record<string, Record<number, boolean[]>>>({});

  nextStep(tabId: string, totalSteps: number) {
    if ((this.guideProgress[tabId] ?? 0) < totalSteps - 1) {
      this.guideProgress[tabId]++;
    }
  }

  prevStep(tabId: string) {
    if ((this.guideProgress[tabId] ?? 0) > 0) {
      this.guideProgress[tabId]--;
    }
  }

  setStep(tabId: string, index: number) {
    this.guideProgress[tabId] = index;
  }

  ensureProgress(tabId: string) {
    if (this.guideProgress[tabId] === undefined) {
      this.guideProgress[tabId] = 0;
    }
    if (!this.checkboxStates[tabId]) {
      this.checkboxStates[tabId] = {};
    }
  }

  /** Restaure la progression depuis le profil sauvegardé */
  restoreFromProfile(
    guideProgress: Record<string, number>,
    checkboxStates: Record<string, Record<number, boolean[]>>,
  ) {
    this.guideProgress = guideProgress;
    this.checkboxStates = checkboxStates;
  }
}

export const guideStore = new GuideStore();
