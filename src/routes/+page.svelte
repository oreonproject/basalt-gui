<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let shouldShowWelcomeScreen: boolean = false;

  let hasAccounts: boolean = false;

  onMount(async () => {
    try {
      const existed = await invoke<boolean>("initialize_data_dir");
      const isFirstBoot = !existed;
      if (isFirstBoot) {
        shouldShowWelcomeScreen = true;
        setTimeout(() => {
          shouldShowWelcomeScreen = false;
        }, 5000);
      }
    } catch (err) {
      console.error("Failed to initialize data directory:", err);
    }
    hasAccounts = await invoke<boolean>("has_accounts");
  });
</script>

<main class="h-screen flex justify-center">
  {#if shouldShowWelcomeScreen}
    <div class="mt-[40vh]">
      <p class="text-2xl">Welcome to basalt</p>
      <p class="text-gray-400 text-center">We hope you'll love it</p>
      <br />
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="size-6"
      >
        <path
          class="[stroke-dasharray:200] [stroke-dashoffset:200] animate-[draw_2s_ease-in-out_forwards]"
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
        />
      </svg>
    </div>
  {/if}
  {#if !hasAccounts}
    <div class="mt-4 flex flex-col items-center w-full">
      <p class="text-gray-400 text-center">
        No accounts found. Please login to one of the providers.
      </p>
      <br />
      <button
        class="flex items-center gap-2 px-4 py-2 border border-gray-600 rounded-md hover:bg-gray-800 transition-colors w-1/5 bg-slate-700 hover:cursor-pointer"
      >
        <svg
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 48 48"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          style="display: block;"
          class="w-6 h-6"
        >
          <path
            fill="#EA4335"
            d="M24 9.5c3.54 0 6.71 1.22 9.21 3.6l6.85-6.85C35.9 2.38 30.47 0 24 0 14.62 0 6.51 5.38 2.56 13.22l7.98 6.19C12.43 13.72 17.74 9.5 24 9.5z"
          ></path>
          <path
            fill="#4285F4"
            d="M46.98 24.55c0-1.57-.15-3.09-.38-4.55H24v9.02h12.94c-.58 2.96-2.26 5.48-4.78 7.18l7.73 6c4.51-4.18 7.09-10.36 7.09-17.65z"
          ></path>
          <path
            fill="#FBBC05"
            d="M10.53 28.59c-.48-1.45-.76-2.99-.76-4.59s.27-3.14.76-4.59l-7.98-6.19C.92 16.46 0 20.12 0 24c0 3.88.92 7.54 2.56 10.78l7.97-6.19z"
          ></path>
          <path
            fill="#34A853"
            d="M24 48c6.48 0 11.93-2.13 15.89-5.81l-7.73-6c-2.15 1.45-4.92 2.3-8.16 2.3-6.26 0-11.57-4.22-13.47-9.91l-7.98 6.19C6.51 42.62 14.62 48 24 48z"
          ></path>
          <path fill="none" d="M0 0h48v48H0z"></path>
        </svg>
        Google
      </button>
    </div>
  {/if}
  <div></div>
</main>
