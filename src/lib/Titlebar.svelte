<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  const appWindow = getCurrentWindow();
  let isMaximized = $state(false);

  onMount(async () => {
    isMaximized = await appWindow.isMaximized();
  });

  const handleToggleMaximize = async () => {
    await appWindow.toggleMaximize();
    isMaximized = await appWindow.isMaximized();
  };
</script>

<div
  class="fixed inset-x-0 top-0 h-8 bg-gray-900/95 backdrop-blur-md border-b border-gray-700/30 select-none flex items-center justify-between px-2"
  ondblclick={handleToggleMaximize}
  onmousedown={() => appWindow.startDragging()}
  role="toolbar"
  tabindex="-1"
>
  <span class="text-sm text-white/70 font-medium pl-1">Basalt GUI</span>

  <div class="flex items-center h-full">
    <button
      class="w-12 h-full flex items-center justify-center text-white/70 transition-colors hover:bg-white/10 hover:text-white cursor-pointer"
      onclick={() => appWindow.minimize()}
      aria-label="Minimize"
    >
      <i class="fas fa-minus text-xs"></i>
    </button>

    <button
      class="w-12 h-full flex items-center justify-center text-white/70 transition-colors hover:bg-red-500 hover:text-white cursor-pointer"
      onclick={() => appWindow.close()}
      aria-label="Close"
    >
      <i class="fas fa-times text-xs"></i>
    </button>
  </div>
</div>
