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
  class="fixed inset-x-0 top-0 flex items-center justify-between h-8 px-2 border-b select-none bg-gray-900/95 backdrop-blur-md border-gray-700/30"
  ondblclick={handleToggleMaximize}
  onmousedown={() => appWindow.startDragging()}
  role="toolbar"
  tabindex="-1"
>
  <span class="pl-1 text-sm font-medium text-white/70">Basalt GUI</span>

  <div class="flex items-center h-full">
    <button
      class="flex items-center justify-center w-12 h-full transition-colors cursor-pointer text-white/70 hover:bg-white/10 hover:text-white"
      onclick={() => appWindow.minimize()}
      aria-label="Minimize"
    >
      <i class="text-xs fas fa-minus"></i>
    </button>

    <button
      class="flex items-center justify-center w-12 h-full transition-colors cursor-pointer text-white/70 hover:bg-red-500 hover:text-white"
      onclick={() => appWindow.close()}
      aria-label="Close"
    >
      <i class="text-xs fas fa-times"></i>
    </button>
  </div>
</div>
