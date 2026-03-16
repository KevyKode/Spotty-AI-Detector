<script>
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';

  let status = "idling";
  let detectionResult = "";

  onMount(async () => {
    await invoke('check_models');
  });

  async function handleImageScan() {
    if (status !== "idling") return;
    status = "scanning_img";
    try {
      let result = await invoke('analyze_screen');
      setTimeout(() => {
        status = "detected";
        detectionResult = result;
      }, 500);
    } catch (e) {
      status = "idling";
    }
  }

  async function handleVideoScan() {
    if (status !== "idling") return;
    status = "scanning_vid";
    try {
      let result = await invoke('analyze_video');
      status = "detected";
      detectionResult = result;
    } catch (e) { 
      status = "detected";
      detectionResult = `*Whines* ${e}`;
    }
  }

  async function sleepSpotty() {
    try {
      await getCurrentWindow().hide();
    } catch (e) {
      status = "detected";
      detectionResult = "I can't sleep! Error: " + e;
    }
  }

  function reset() {
    status = "idling";
  }

  $: mascotAnimClass = 
    (status === 'scanning_img' || status === 'scanning_vid') ? 'anim-sniff' : 
    status === 'detected' ? 'animate-bounce' : 'anim-float';
</script>

<main class="w-screen h-screen flex flex-col items-center justify-center font-sans overflow-hidden" 
      style="-webkit-app-region: drag; background-color: transparent;">

  <!-- Dialogue Box Area -->
  <div class="h-28 w-full flex items-end justify-center pb-2 z-10 px-4 text-center">
    {#if status === 'detected'}
      <div class="bg-white text-black p-2 rounded-xl shadow-lg border-2 border-red-500 relative w-full items-center justify-center flex flex-col">
        
        <!-- THE FIX: Made the text fully copyable, selectable, and scrollable! -->
        <p class="font-bold text-[10px] w-full max-h-16 overflow-y-auto break-words select-text cursor-text"
           style="-webkit-app-region: no-drag; pointer-events: auto;">
          {detectionResult}
        </p>
        
        <button on:click={reset} 
                style="-webkit-app-region: no-drag; pointer-events: auto;" 
                class="mt-2 px-6 py-1 bg-red-100 text-red-700 hover:bg-neutral-200 rounded text-[10px] font-bold transition-colors">
          Good Boy (Dismiss)
        </button>
        
        <div class="absolute -bottom-2 left-1/2 -translate-x-1/2 w-0 h-0 border-l-[8px] border-r-[8px] border-t-[10px] border-l-transparent border-r-transparent border-t-red-500"></div>
      </div>
    {:else if status !== 'idling'}
      <div class="bg-gray-800 text-white p-2 rounded-xl shadow-lg relative font-medium w-max px-3">
        <p class="text-xs">{status === 'scanning_vid' ? '* Checking movement... *' : '* Sniff sniff sniff... *'}</p>
        <div class="absolute -bottom-2 left-1/2 -translate-x-1/2 w-0 h-0 border-l-[8px] border-r-[8px] border-t-[10px] border-l-transparent border-r-transparent border-t-gray-800"></div>
      </div>
    {/if}
  </div>

  <div class="group flex flex-col items-center">
     <!-- The Dog -->
     <div class="w-28 h-28 relative {mascotAnimClass} rounded-full flex items-center justify-center pointer-events-none">
          <svg viewBox="0 0 100 100" class="w-[85%] h-[85%] drop-shadow-2xl">
             <path d="M 75 55 Q 90 70 80 90 Q 75 80 70 65 Z" fill="#1f2937" /> 
             <circle cx="50" cy="65" r="25" fill="#f8fafc" />
             <circle cx="38" cy="55" r="4.5" fill="#1f2937" />
             <circle cx="62" cy="72" r="3.5" fill="#1f2937" />
             <circle cx="45" cy="82" r="2.5" fill="#1f2937" />
             <path d="M 25 55 Q 10 70 20 90 Q 25 80 30 65 Z" fill="#f8fafc" stroke="#e2e8f0" stroke-width="1"/>
             <circle cx="21" cy="78" r="3" fill="#1f2937" />
             <path d="M 15 48 Q 50 40 85 48 L 95 50 Q 50 45 5 50 Z" fill="#b91c1c" />
             <path d="M 25 45 Q 50 5 75 45 Z" fill="#ef4444" />
             <path d="M 40 25 L 60 25 L 55 45 L 50 48 L 45 45 Z" fill="#fbbf24" stroke="#d97706" stroke-width="1.5" />
             <text x="50" y="40" font-size="12" font-family="monospace" font-weight="900" fill="#78350f" text-anchor="middle">AI</text>
             <circle cx="42" cy="62" r="3.5" fill="#1f2937" />
             <circle cx="58" cy="62" r="3.5" fill="#1f2937" />
             <circle cx="41" cy="61" r="1" fill="white" />
             <circle cx="57" cy="61" r="1" fill="white" />
             <ellipse cx="50" cy="73" rx="5" ry="3.5" fill="#1f2937" />
             <circle cx="48.5" cy="72" r="1" fill="white" opacity="0.6"/>
             {#if status === 'detected'}
               <path d="M 45 80 Q 50 86 55 80" stroke="#1f2937" fill="transparent" stroke-width="2" stroke-linecap="round"/>
               <path d="M 48 83 Q 50 90 52 83 Z" fill="#fca5a5" />
             {:else}
               <path d="M 45 79 Q 50 82 55 79" stroke="#1f2937" fill="transparent" stroke-width="2" stroke-linecap="round"/>
             {/if}
          </svg>
     </div>

     <!-- Under-Belly Menu Area -->
     <div class="h-10 mt-1 flex items-start justify-center text-center z-50">
       {#if status === 'idling'}
       <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity" style="-webkit-app-region: no-drag; pointer-events: auto;">
         <button on:click={handleImageScan} class="px-2 py-1 bg-blue-500 text-white text-[11px] font-bold rounded shadow hover:bg-blue-600 transition">Image</button>
         <button on:click={handleVideoScan} class="px-2 py-1 bg-purple-500 text-white text-[11px] font-bold rounded shadow hover:bg-purple-600 transition">Video</button>
         <button on:click={sleepSpotty} class="px-2 py-1 bg-gray-600 text-white text-[11px] font-bold rounded shadow hover:bg-gray-700 transition">Sleep</button>
       </div>
       {/if}
     </div>
  </div>
</main>