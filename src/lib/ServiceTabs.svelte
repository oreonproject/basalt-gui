<script lang="ts">
	import ProviderServices from './ProviderServices.svelte';
  const tabs = ["Google", "Nextcloud", "Microsoft"];
  let activeTab = "Google";
</script>

<div class="flex flex-col h-screen">
  <div class="flex gap-1 px-2 pt-2 bg-gray-800">
    {#each tabs as tab}
      <button
        class="px-4 py-2 rounded-t-lg transition-colors cursor-pointer {activeTab === tab
          ? 'bg-gray-900 text-white'
          : 'bg-gray-600 text-gray-300 hover:bg-gray-500'}"
        on:click={() => (activeTab = tab)}
      >
        {tab}
      </button>
    {/each}
  </div>

  <div class="flex flex-1 bg-gray-900">
    <ProviderServices />
  </div>
</div>
