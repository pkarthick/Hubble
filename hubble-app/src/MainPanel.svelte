<script lang="typescript">
  import Folder from "./Folder.svelte";
  import FolderView from "./FolderView.svelte";
  import { TabItem } from "./Models";
  
  export let visible = true;

  export let items: TabItem<any>[] = [
    {
      id: 0,
      path: "/mnt/c",
      icon: "folder_open",
      component: Folder,
      props: {}
    },
    {
      id: 1,
      path: "/mnt/d",
      icon: "folder_open",
      component: Folder,
      props: {}
    }
  ];

  export let tabitems: TabItem<any>[] = [
    {
      id: 0,
      path: "/mnt/d/Mirror/Fun",
      icon: "folder_open",
      component: Folder,
      props: {}
    },
    {
      id: 0,
      path: "/mnt/d/Mirror/Explore",
      icon: "folder_open",
      component: Folder,
      props: {}
    }
  ];

  let count = 0;

  function addTab(evt) {
    count = count + 1;

    tabitems.push({
      id: count,
      path: evt.detail.path,
      icon: "folder_open",
      component: Folder,
      props: {}
    });
    tabitems = tabitems;
  }

  function closeTab(evt) {
    let index = tabitems.findIndex(ti => ti.path == evt.detail.path);

    if (index >= 0) {
      tabitems.splice(index, 1);
      tabitems = tabitems;
    }
  }
</script>

<style>

/* width */
::-webkit-scrollbar {
  width: 10px;
}

/* Track */
::-webkit-scrollbar-track {
  background: #e0e0e0;
}

/* Handle */
::-webkit-scrollbar-thumb {
  background: #c0c0c0;
}

/* Handle on hover */
::-webkit-scrollbar-thumb:hover {
  background: #90cbf9;
}

</style>

<div class="flex flex-1 flex-row" class:visible>

  
    <div
      class="flex flex-initial flex-grow-1 flex-shrink-0 flex-col overflow-auto
      p-3 bg-primary-300"
      style=" height: 94vh;"
      class:visible
      >
      {#each items as item}
        <Folder on:message={addTab} {...item} selectable={false} />
      {/each}
    </div>
  
  <div class="flex flex-1 flex-row">

    {#each tabitems as item (item.path)}
      <div class="flex">
        <FolderView
          on:closetab={closeTab}
          on:message={addTab}
          {...item}
          showfiles />
      </div>
    {/each}


    <!-- 
    <Tabs on:closetab={closeTab} on:message={addTab} items={tabitems}>
      <div slot="content" class="flex flex-col flex-grow-0 bg-gray-500" />

    </Tabs> -->

  </div>
</div>
