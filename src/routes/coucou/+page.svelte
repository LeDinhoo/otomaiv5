<script>
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { fly } from "svelte/transition";
  import { quintOut } from "svelte/easing";

  let visible = $state(false);

  // Variables pour stocker les identifiants des timers
  let timerAttente;
  let timerFermeture;

  // Lance le compte à rebours (auto-fermeture)
  function lancerCompteARebours() {
    console.log("⏳ Fermeture dans 4.5s...");
    timerAttente = setTimeout(() => {
      fermerLaFenetre();
    }, 4500);
  }

  // Arrête le timer (quand on survole)
  function stopCompteARebours() {
    console.log("✋ Pause ! (Souris détectée)");
    clearTimeout(timerAttente);
    clearTimeout(timerFermeture);
    visible = true;
  }

  // Fonction générique pour fermer proprement avec animation
  function fermerLaFenetre() {
    visible = false; // Déclenche l'animation de sortie (fade out)

    // Attend la fin de l'animation (600ms) avant de détruire la fenêtre
    timerFermeture = setTimeout(async () => {
      const win = getCurrentWindow();
      await win.close();
    }, 600);
  }

  // --- ACTIONS DES BOUTONS ---

  async function actionCapturer(e) {
    // Empêche le clic de traverser (bonnes pratiques)
    e?.stopPropagation();

    console.log("✅ BOUTON: CAPTURER cliqué !");
    // TODO: Ici tu pourras ajouter ton code pour lancer la capture

    // On force la fermeture immédiate (plus besoin d'attendre)
    clearTimeout(timerAttente);
    fermerLaFenetre();
  }

  async function actionRefuser(e) {
    e?.stopPropagation();

    console.log("❌ BOUTON: REFUSER cliqué !");

    clearTimeout(timerAttente);
    fermerLaFenetre();
  }

  onMount(() => {
    // Petit délai initial pour l'animation d'entrée
    setTimeout(() => {
      visible = true;
      lancerCompteARebours();
    }, 50);
  });
</script>

<div
  class="h-screen w-screen bg-transparent overflow-hidden flex items-center justify-center p-2"
>
  {#if visible}
    <div
      aria-role="alert"
      onmouseenter={stopCompteARebours}
      onmouseleave={lancerCompteARebours}
      transition:fly={{ x: -300, duration: 300, easing: quintOut }}
      class="w-full max-w-sm bg-stone-900 backdrop-blur-sm border border-stone-700 text-white rounded-lg shadow-2xl overflow-hidden flex flex-col"
    >
      <div class="p-4 flex items-center gap-4">
        <img src="arch.jpg" alt="archimonstre" class="w-8 rounded-full" />
        <div>
          <h3 class="font-bold text-sm">Détection Archimonstre</h3>
          <p class="text-xs text-slate-300">
            Un groupe intéressant a été trouvé.
          </p>
        </div>
      </div>

      <div class="flex border-t border-stone-700 hover:cursor-pointer">
        <button
          onclick={actionRefuser}
          class="flex-1 py-3 text-sm font-semibold text-red-400 hover:bg-red-500/10 hover:text-red-300 transition-colors border-r border-stone-700 active:bg-red-500/20 hover:cursor-pointer"
        >
          Refuser
        </button>

        <button
          onclick={actionCapturer}
          class="flex-1 py-3 text-sm font-semibold text-green-400 hover:bg-green-500/10 hover:text-green-300 transition-colors active:bg-green-500/20 hover:cursor-pointer"
        >
          Capturer
        </button>
      </div>
    </div>
  {/if}
</div>
