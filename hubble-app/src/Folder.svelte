<script language="typescript">
  import File from "./File.svelte";
  import Icon from "fa-svelte";
  import "@fontawesome/fontawesome-free/css/all.min.css";
  import { onMount, createEventDispatcher } from "svelte";
  export let path = "";
  export let files = [];
  export let folders = [];
  export let selected = "<None>";
  export let selectable = true;
  const dispatch = createEventDispatcher();
  let expanded = false;
  let loaded = false;
  //$: expanded = files.some(f => f.type === "folder");
  let name = "";
  export let showfiles = false;

  async function toggle() {
    selected = path;
    expanded = (files.length == 0 && folders.length === 0) || !expanded;
    if (expanded) {
      await loadFolder();
    }
  }

  async function loadFolder() {
    const res = await fetch("http://localhost:3030/folder", {
      body: JSON.stringify({ path: path, include_files: true }),
      headers: {
        "content-type": "application/json"
      },

      method: "POST"
    });
    const json = await res.json();

    files = json.files;
    folders = json.folders;
    loaded = true;
    expanded = true;
  }

  onMount(async function() {
    let lastInd = path.lastIndexOf("/");
    
    if (path.length - 1 !== lastInd) {
      name = path.slice(lastInd + 1);
    } else {
      name = path;
    }
  });

  function sendMessage() {
    dispatch("message", { path: path });
  }
</script>

<style>
  ul {
    padding: 0.15em 0 0 0.5em;
    margin: 0 0 0 0.5em;
    list-style: none;
    border-left: 1px solid #616161;
  }

  li {
    padding: 0.15em 0;
  }
</style>

<div class="ml-3 whitespace-no-wrap">

  <span on:click={toggle}>
    {#if expanded}
      <i class="text-gray-700 far fa-md fa-minus-square mr-1" />
    {:else}
      <i class="text-gray-700 far fa-md fa-plus-square mr-1" />
    {/if}

    <b>
      {#if selectable}<input type="checkbox" />{/if}
      {name}
    </b>

    <i
      on:click|stopPropagation={sendMessage}
      title="Pop out as a new tab"
      class="text-gray-700 fas fa-md fa-external-link-alt ml-2" />
  </span>

  {#if loaded && expanded}
    <ul>
      {#each folders as folder}
        <li>
          <svelte:self {...folder} on:message {showfiles} expanded={false} {selectable} />
        </li>
      {/each}
      {#if showfiles == true}
        {#each files as file}
          <li>
            <File {...file} />
          </li>
        {/each}
      {/if}
    </ul>
  {/if}

</div>
