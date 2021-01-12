<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import Folder from "./Folder.svelte";

  import FolderView from "./FolderView.svelte";

  const dispatch = createEventDispatcher();

  let selected = "";
</script>

<nav class="flex items-center bg-primary-900 justify-between flex-wrap p-2">

  <div class="flex flex-initial flex-row">
    <button
      class="flex items-center px-3 py-2 border rounded text-white
      hover:bg-primary-300"
      on:click={() => dispatch('togglesidebar', {})}>

      <i class="fas fa-bars" />

    </button>
    <span class="ml-2 font-semibold text-xl text-white tracking-tight">
      <i class="ml-3 fas fa-lg fa-binoculars text-gray-900" />
      <span class="ml-1">Hubble</span>
    </span>
  </div>
  <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
    <div class="text-sm lg:flex-grow" />
    <div>
      <!-- <a
        href="#responsive-header"
        class="block mt-4 lg:inline-block lg:mt-0 text-teal-200 hover:text-white
        mr-4">
        Docs
      </a>
       -->
      <a
        href="javascript:void(0);"
        class="inline-block text-sm px-4 py-2 leading-none border rounded
        text-white border-white hover:border-transparent hover:text-teal-500
        hover:bg-white mt-4 lg:mt-0">
        Docs
      </a>
      <a
        href="javascript:void(0);"
        class="inline-block text-sm px-4 py-2 leading-none border rounded
        text-white border-white hover:border-transparent hover:text-teal-500
        hover:bg-white mt-4 lg:mt-0">
        About
      </a>
    </div>
  </div>
</nav>
